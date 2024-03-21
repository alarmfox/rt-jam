use argon2::{password_hash::SaltString, Argon2, PasswordVerifier};
use base64::{engine::general_purpose, Engine};
use rand_core::OsRng;
use sqlx::prelude::FromRow;
use sqlx::PgPool;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::service::{
    email,
    error::{Error, Result},
};

#[derive(Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: Option<String>,
    pub verification_token: Option<String>,
    pub verification_token_expires_in: Option<OffsetDateTime>,
    pub enabled: bool,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("firstname", &self.first_name)
            .field("lastname", &self.last_name)
            .field("password", &"[redacted]")
            .finish()
    }
}

pub mod auth {
    use std::ops::Add;

    use argon2::{PasswordHash, PasswordHasher};
    use time::Duration;

    use super::*;

    pub struct Service {
        db: PgPool,
        email_service: email::Service,
    }

    impl Service {
        pub fn new(db: PgPool, email_service: email::Service) -> Self {
            Self { db, email_service }
        }
    }

    impl Service {
        pub async fn register(
            &self,
            first_name: String,
            last_name: String,
            email: String,
            username: String,
        ) -> Result<User> {
            let id = Uuid::new_v4();
            let token = session::Service::generate_token();
            let token = general_purpose::STANDARD.encode(token);
            let user = sqlx::query_as!(User,
            r#"
            INSERT INTO users
            (id, first_name, last_name, email, username, verification_token, verification_token_expires_in, enabled)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
           "#,
        id,
        first_name,
        last_name,
        email,
        username,
        token.clone(),
        OffsetDateTime::now_utc().add(Duration::days(7)),
            false
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            if let Some(err) = e.as_database_error() {
                if err.is_unique_violation() {
                    return Error::UserAlreadyExists;
                }
            }
            Error::DatabaseError(e)
        })?;

            tokio::spawn(async move {
                // self.email_service
                //     .send_verification_link(&token, &user)
                // .await
            });

            Ok(user)
        }

        pub async fn login(&self, username: String, password: String) -> Result<User> {
            let user = sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE username= $1 AND enabled = TRUE",
                username
            )
            .fetch_optional(&self.db)
            .await
            .map_err(|e| Error::DatabaseError(e))?;

            let user = match user {
                Some(user) => user,
                None => return Err(Error::InvalidCredentials),
            };

            let password_hash = user.clone().password.ok_or(Error::InvalidCredentials)?;

            let password_hash =
                PasswordHash::new(&password_hash).map_err(|_e| Error::CryptoError)?;

            Argon2::default()
                .verify_password(password.as_bytes(), &password_hash)
                .map_err(|_e| Error::InvalidCredentials)
                .map(|_a| user)
        }

        pub async fn start_password_reset(&self, email: String) -> Result<()> {
            let token = session::Service::generate_token();
            let token = general_purpose::STANDARD.encode(token);
            let user = sqlx::query_as!(
                User,
                r#"UPDATE users SET 
            verification_token = $2,
            verification_token_expires_in = $3
            WHERE email = $1
            RETURNING *
            "#,
                email,
                token.clone(),
                OffsetDateTime::now_utc().add(Duration::days(7)),
            )
            .fetch_optional(&self.db)
            .await
            .map_err(Error::DatabaseError)?
            .ok_or(Error::NoAuth)?;

            self.email_service
                .send_reset_link(&token, &user)
                .await
                .unwrap();

            Ok(())
        }

        pub async fn reset_password(&self, password: String, token: String) -> Result<()> {
            let password = Self::hash_password(password)?;

            sqlx::query!(
                r#"UPDATE users SET 
            verification_token = NULL,
            verification_token_expires_in = NULL,
            enabled = TRUE,
            password = $2
            WHERE verification_token = $1 AND verification_token_expires_in > now()
            RETURNING *
            "#,
                token,
                password
            )
            .fetch_optional(&self.db)
            .await
            .map_err(Error::DatabaseError)?
            .ok_or(Error::InvalidCredentials)?;

            Ok(())
        }

        fn hash_password(plain_text: String) -> Result<String> {
            let argon2 = Argon2::default();
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2
                .hash_password(plain_text.as_bytes(), &salt)
                .map_err(|_e| Error::CryptoError)?;
            Ok(password_hash.to_string())
        }
    }
}

pub mod session {
    use rand_chacha::ChaCha8Rng;
    use rand_core::{RngCore, SeedableRng};
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Clone)]
    pub struct Service {
        db: PgPool,
    }

    impl Service {
        pub fn new(db: PgPool) -> Self {
            Self { db }
        }
    }

    pub struct Session {
        pub id: String,
        pub data: Vec<u8>,
        pub expiry_date: OffsetDateTime,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct SessionData {
        pub id: Uuid,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
        pub username: String,
    }

    impl Service {
        pub async fn create(
            &self,
            token: &str,
            data: &SessionData,
            expiry_date: OffsetDateTime,
        ) -> Result<Session> {
            let session = sqlx::query_as!(
                Session,
                r#"INSERT INTO sessions (id, data, expiry_date) 
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET 
                data = excluded.data,
                expiry_date = excluded.expiry_date
            RETURNING *
            "#,
                token,
                serde_json::to_vec(data).map_err(|e| Error::SerializationError(e))?,
                expiry_date
            )
            .fetch_one(&self.db)
            .await?;
            Ok(session)
        }
        pub async fn get(&self, token: String) -> Result<Option<SessionData>> {
            let session = sqlx::query_as!(
                Session,
                r#"SELECT * FROM sessions WHERE id = $1 AND expiry_date > $2"#,
                token,
                OffsetDateTime::now_utc()
            )
            .fetch_optional(&self.db)
            .await?;

            if let Some(session) = session {
                return Ok(Some(
                    serde_json::from_slice(&session.data)
                        .map_err(|e| Error::SerializationError(e))?,
                ));
            }

            Ok(None)
        }

        pub async fn delete(&self, token: &str) -> Result<()> {
            sqlx::query!(r#"DELETE FROM sessions WHERE id = $1"#, token)
                .execute(&self.db)
                .await?;

            Ok(())
        }

        pub async fn continously_delete_expired_sessions(
            self,
            period: tokio::time::Duration,
        ) -> Result<()> {
            let mut interval = tokio::time::interval(period);
            loop {
                self.delete_expired_sessions().await?;
                interval.tick().await;
            }
        }
        pub fn generate_token() -> String {
            let mut bytes = [0u8; 16];
            let mut random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());
            random.fill_bytes(&mut bytes);

            u128::from_le_bytes(bytes).to_string()
        }

        async fn delete_expired_sessions(&self) -> Result<()> {
            sqlx::query!(r#"DELETE FROM sessions WHERE expiry_date < (now() AT TIME ZONE 'utc')"#)
                .execute(&self.db)
                .await?;
            Ok(())
        }
    }

    impl From<User> for SessionData {
        fn from(
            User {
                id,
                email,
                first_name,
                last_name,
                username,
                ..
            }: User,
        ) -> Self {
            Self {
                id,
                email,
                first_name,
                last_name,
                username,
            }
        }
    }
}

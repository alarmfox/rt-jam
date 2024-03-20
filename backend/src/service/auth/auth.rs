use std::ops::Add;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::extract::FromRef;
use base64::{engine::general_purpose, Engine};
use sqlx::{prelude::FromRow, PgPool};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::service::email;

use super::{error::{Error, Result}, session};

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

#[derive(Clone, FromRef)]
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

        let password_hash = PasswordHash::new(&password_hash).map_err(|_e| Error::CryptoError)?;

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

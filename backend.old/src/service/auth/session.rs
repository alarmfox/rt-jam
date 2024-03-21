use axum::extract::FromRef;
use rand_chacha::ChaCha8Rng;
use rand_core::{OsRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::{auth::User, error::{Error, Result}};

#[derive(Clone, FromRef)]
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
    ) -> Result<Session>{
        let session = sqlx::query_as!(Session,
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
                serde_json::from_slice(&session.data).map_err(|e| Error::SerializationError(e))?,
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

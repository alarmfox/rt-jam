use std::{collections::HashMap, sync::Arc};

use axum::extract::FromRef;
use serde::Deserialize;
use sqlx::{prelude::FromRow, PgPool};
use time::PrimitiveDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

use super::{error::Error, SearchResult};

struct Session {
    current_people_listening: i32,
    current_people_playing: i32,
    max_people_playing: i32,
}

#[derive(Clone, FromRef)]
pub struct Service {
    db: PgPool,
    sessions: Arc<Mutex<HashMap<Uuid, Session>>>,
}

#[derive(FromRow)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner: String,
    pub private: bool,
    pub open: bool,
    pub max_people_playing: i32,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct SearchParams {
    pub owner: Option<String>,
    pub private: Option<bool>,
    pub open: Option<bool>,
    pub page: i64,
    pub page_size: i64,
}

impl Default for SearchParams {
    fn default() -> Self {
        Self {
            owner: None,
            private: Some(false),
            open: Some(true),
            page: 1,
            page_size: 10,
        }
    }
}

impl Service {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// crud operations
impl Service {
    pub async fn create(
        &self,
        owner: String,
        name: String,
        description: Option<String>,
        private: bool,
        open: bool,
        max_people_playing: i32,
    ) -> Result<Room, Error> {
        let id = Uuid::new_v4();
        let room = sqlx::query_as!(
            Room,
            r#"INSERT INTO rooms (id, owner, name, description, private, open, max_people_playing) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            id,
            owner,
            name,
            description,
            private,
            open,
            max_people_playing
        )
        .fetch_one(&self.db)
        .await?;

        self.sessions.lock().await.insert(
            id,
            Session {
                current_people_playing: 0,
                current_people_listening: 0,
                max_people_playing,
            },
        );

        Ok(room)
    }

    pub async fn search(
        &self,
        SearchParams {
            owner,
            page,
            page_size,
            private,
            open,
        }: SearchParams,
    ) -> Result<SearchResult<Room>, Error> {
        let rooms = sqlx::query_as!(
            Room,
            r#" SELECT * FROM rooms
            WHERE ($1::varchar IS NULL OR owner = $1)
            AND ($2::boolean IS NULL OR private = $2)
            AND ($3::boolean IS NULL OR open = $3)
            LIMIT $4 OFFSET $5
            "#,
            owner,
            private,
            open,
            page_size,
            (page - 1) * page_size
        )
        .fetch_all(&self.db)
        .await?;

        // TODO: pagination
        Ok(SearchResult {
            data: rooms,
            next: None,
            prev: None,
        })
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Room, Error> {
        let room = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE id = $1", id)
            .fetch_one(&self.db)
            .await?;

        Ok(room)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM rooms WHERE id = $1"#, id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}

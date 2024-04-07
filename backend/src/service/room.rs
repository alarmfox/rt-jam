use axum::extract::FromRef;
use sqlx::{prelude::FromRow, PgPool};
use time::PrimitiveDateTime;
use uuid::Uuid;

pub(crate) use super::error::Error;


#[derive(Clone, FromRef)]
pub struct Service {
    db: PgPool,
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


impl Service {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
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


        Ok(room)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Room>, Error> {
        let room = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE id = $1", id)
            .fetch_optional(&self.db)
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

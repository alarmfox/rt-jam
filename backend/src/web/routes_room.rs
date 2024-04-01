use crate::service::room::{self, Room};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, post},
    Json as AJson, Router,
};
use common::types::{CreateRoomRequest, RoomResponse};

use super::{
    error::{Error, Result},
    json::Json,
    mw_auth::CtxW,
};

#[derive(Clone)]
struct AppState {
    room_service: room::Service,
}

pub fn router(room_service: room::Service) -> Router {
    Router::new()
        .route("/", post(create))
        .route("/:id", delete(delete_room).get(get_by_id))
        .with_state(AppState { room_service })
}

async fn create(
    context: CtxW,
    State(AppState { room_service }): State<AppState>,
    Json(CreateRoomRequest { name }): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse> {
    let username = context.0.get_session().username;

    let room = room_service
        .create(username, name, None, false, true, 5)
        .await?;

    Ok((StatusCode::CREATED, AJson(RoomResponse::from(room))))
}

async fn delete_room(
    Path(id): Path<uuid::Uuid>,
    State(AppState { room_service, .. }): State<AppState>,
    context: CtxW,
) -> Result<impl IntoResponse> {
    let room = room_service.get_by_id(id).await?;
    if let Some(room) = room {
        if room.owner != context.0.get_session().username {
            return Err(Error::NotAllowed);
        }

        room_service.delete(id).await?;
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn get_by_id(
    Path(id): Path<uuid::Uuid>,
    State(AppState { room_service }): State<AppState>,
) -> Result<impl IntoResponse> {
    let room = room_service.get_by_id(id).await?;

    if let Some(room) = room {
        Ok(AJson(RoomResponse::from(room)))
    } else {
        Err(Error::NotFound)
    }

}

impl From<Room> for RoomResponse {
    fn from(
        Room {
            id, name, owner, ..
        }: Room,
    ) -> Self {
        Self { id, name, owner }
    }
}

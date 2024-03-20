use axum::{
    extract::{FromRef},
    Router,
};

use crate::{
    service::{
        room::{self},
    },
};


#[derive(Clone, FromRef)]
struct AppState {
    room_service: room::Service,
}

pub fn routes(room_service: room::Service) -> Router {
    let app_state = AppState { room_service };
    Router::new()
        .with_state(app_state)
}

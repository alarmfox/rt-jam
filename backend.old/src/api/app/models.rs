use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateRoomRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Room name length must be between 3 and 50 characters"
    ))]
    pub name: String,
    #[validate(length(
        max = 120,
        message = "Room description length must be maximum 120 characters"
    ))]
    pub description: Option<String>,
    pub max_people_playing: i32,

    #[serde(default)]
    pub open: bool,

    #[serde(default)]
    pub private: bool,
}

#[derive(Deserialize, Validate)]
pub struct GetRoom {
    pub id: Uuid,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username length must be between 3 and 50 characters"
    ))]
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "First name length must be between 3 and 50 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Last name length must be between 3 and 50 characters"
    ))]
    pub last_name: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 50,
        message = "Username length must be between 3 and 50 characters"
    ))]
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct ChangePasswordRequest {
    pub token: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Serialize, Deserialize, Clone, Validate)]
pub struct StartResetRequest{
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

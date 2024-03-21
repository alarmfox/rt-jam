use serde::Deserialize;
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, max = 50, message = "First name length must be between 3 and 50 characters"))]
    pub first_name: String,
    #[validate(length(min = 3, max = 50, message = "Last name length must be between 3 and 50 characters"))]
    pub last_name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username length must be between 3 and 50 characters"))]
    pub username: String
}

#[derive(Deserialize, Validate)]
pub struct ResetPayload {
    pub token: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    pub confirm_password: String
}
#[derive(Deserialize, Validate)]
pub struct StartResetPayload {
    #[validate(email(message="Invalid email"))]
    pub email: String,
}

use actix_web_validator::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate, Debug)]
pub struct SignupRequest {
    pub name: String,
    pub age: u8,

    #[validate(email)]
    pub id: String,
    #[validate(length(min = 8))]
    pub password: String,
}

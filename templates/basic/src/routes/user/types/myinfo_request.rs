use actix_web_validator::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate, Debug)]
pub struct MyInfoRequest {
    pub need_more: bool,
}

use actix_web::{
    http::{StatusCode},
    post,
    Responder,
    HttpRequest,
    HttpResponse,
};
use actix_web_validator::{Json};

use super::types::{LoginRequest, LoginResponse};

#[post("/auth/login")]
pub async fn login(Json(body): Json<LoginRequest>, _request :HttpRequest) -> impl Responder
{
    let _id = body.id;
    let _password = body.password;

    // do something

    let response = LoginResponse {
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    };

    HttpResponse::build(StatusCode::OK).json(response)
}

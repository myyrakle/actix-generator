use actix_web::{
    http::{StatusCode},
    get,
    post,
    Responder,
    HttpRequest,
    HttpResponse,
};
use actix_web_validator::{Query, Json};

use super::types::{SignupRequest, SignupResponse};

#[post("/user/signup")]
pub async fn signup(Json(body): Json<SignupRequest>, _request :HttpRequest) -> impl Responder
{
    let _id = body.id;
    let _password = body.password;
    let _name = body.name;
    let _age = body.age;

    // do something

    let response = SignupResponse {
        access_token: "access_token".to_string(),
        refresh_token: "refresh_token".to_string(),
    };

    HttpResponse::build(StatusCode::OK).json(response)
}

use super::types::{MyInfoRequest, MyInfoResponse};

#[get("/user/my_info")]
pub async fn my_info(Query(query): Query<MyInfoRequest>, _request :HttpRequest) -> impl Responder
{
    let _need_more = query.need_more;

    // do something

    let response = MyInfoResponse {
        name: "foo bar".to_string(),
        age: 20,
    };

    HttpResponse::build(StatusCode::OK).json(response)
}

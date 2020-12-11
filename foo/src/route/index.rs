use actix_web::{
    http::{StatusCode},
    get,
    post,
    Responder,
    HttpRequest,
    HttpResponse,
};

#[get("/")]
pub async fn helloworld(_request :HttpRequest) -> impl Responder
{
    let json_value = serde_json::json!({
        "Hello": "World!!",
    });

    HttpResponse::build(StatusCode::OK).json(json_value)
}

#[post("/foobar")]
pub async fn foobar(_request :HttpRequest) -> impl Responder
{
    let json_value = serde_json::json!({
        "foo": "bar",
    });

    HttpResponse::build(StatusCode::OK).json(json_value)
}
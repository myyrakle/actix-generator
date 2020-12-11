mod middleware;
mod route;

use actix_web::{App, HttpServer}; 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = "localhost";
    let port = 11111; 
    let address = format!("{}:{}", host, port);

    let _ = listenfd::ListenFd::from_env();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new())
            .service(route::index::helloworld)
            .service(route::index::foobar)
            .service(actix_files::Files::new("/static", "static").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}

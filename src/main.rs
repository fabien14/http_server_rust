use actix_web::{web, App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    HttpServer::new(|| {
        App::new()
        .route("/devices", web::get().to(routes::devices::devices))
        .route("/device/{device_name}", web::get().to(routes::devices::device))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
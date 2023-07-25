mod routes;

use actix_web::{web, App, HttpServer};
use routes::{ devices, device };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    HttpServer::new(|| {
        App::new()
        .route("/devices", web::get().to(devices))
        .route("/devices/{device_name}", web::get().to(device))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
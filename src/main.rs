mod routes;
mod framework_ble;

use actix_web::{web, App, HttpServer};
use routes::{ Devices, devices, device, device_states };
use framework_ble::Device;
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let device_ble_chambre = Device { 
        name: String::from("led_chambre"),
        adress: [1, 2, 3, 4, 5, 6],
        states: HashMap::from([
            (String::from("color"), String::from("#FFFFFF")),
            (String::from("lightless"), String::from("100")),
            (String::from("where"), String::from("1011")),
        ]),
    };
    let device_ble_couloir = Device { 
        name: String::from("led_couloir"),
        adress: [20, 12, 43, 24, 5, 86],
        states: HashMap::from([
            (String::from("color"), String::from("#000000")),
            (String::from("lightless"), String::from("90")),
            (String::from("where"), String::from("0011")),
        ]),
    };

    let device_list = Devices {
        devices: vec![
            device_ble_chambre, 
            device_ble_couloir,
        ]
    };

    HttpServer::new(move || {
        App::new()
        .route("/devices", web::get().to(devices))
        .route("/devices/{device_name}", web::get().to(device))
        .route("/devices/{device_name}/states", web::get().to(device_states))
        .app_data(web::Data::new(device_list.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
mod framework_ble;
mod routes;

use actix_web::{web, App, HttpServer};
use framework_ble::{Device, DeviceName, DeviceStates, DeviceAddress, DeviceColor};
use routes::{device, device_states, devices, Devices};
use std::collections::HashMap;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let device_ble_chambre = Device {
        name: DeviceName(String::from("led_chambre")),
        address: DeviceAddress([1, 2, 3, 4, 5, 6]),
        states: DeviceStates(HashMap::from([
            (String::from("color"), DeviceColor(String::from("#FFFFFF"))),
            (String::from("lightless"), DeviceColor(String::from("100"))),
            (String::from("where"), DeviceColor(String::from("1011"))),
        ])),
    };
    let device_ble_couloir = Device {
        name: DeviceName(String::from("led_couloir")),
        address: DeviceAddress([20, 12, 43, 24, 5, 86]),
        states: DeviceStates(HashMap::from([
            (String::from("color"), DeviceColor(String::from("#000000"))),
            (String::from("lightless"), DeviceColor(String::from("90"))),
            (String::from("where"), DeviceColor(String::from("0011"))),
        ])),
    };

    let device_list = Devices {
        devices: vec![device_ble_chambre, device_ble_couloir],
    };

    HttpServer::new(move || {
        App::new()
            .route("/devices", web::get().to(devices))
            .route("/devices/{device_name}", web::get().to(device))
            .route(
                "/devices/{device_name}/states",
                web::get().to(device_states),
            )
            .app_data(web::Data::new(device_list.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

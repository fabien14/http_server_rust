use actix_web::{ Responder, web, Result };
use serde::{ Serialize, Deserialize };

#[derive(Serialize)]
struct Device {
    name: String,
}

#[derive(Serialize)]
struct Devices {
    devices: Vec<Device>,
}

#[derive(Deserialize)]
pub struct Info {
    device_name: String,
}

pub async fn devices() -> Result<impl Responder> {
    let dev = vec![
        Device { name: "ble".to_string() },
        Device { name: "div".to_string() }
    ];

    let devices_list_struct = Devices {
        devices: dev
    };

    Ok(web::Json(devices_list_struct))
}

pub async fn device(info: web::Path<Info>) -> Result<impl Responder> {
    let devices_list = vec![
        Device { name: "ble".to_string() }, 
        Device { name: "div".to_string() }
    ];

    let device = devices_list.into_iter().find(|device| device.name == info.device_name);

    Ok(web::Json(device))
}
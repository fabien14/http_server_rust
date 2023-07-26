use crate::framework_ble::Device;

use actix_web::{web, Responder, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Serialize)]
struct DeviceStatesSerialize {
    states: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Info {
    device_name: String,
}

pub async fn devices(data: web::Data<Devices>) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let mut dev: Vec<Device> = Vec::new();
    for d in device_list_data {
        dev.push(Device::default_with_name(d.name.clone()));
    }

    let devices_list_struct = Devices { devices: dev };

    Ok(web::Json(devices_list_struct))
}

pub async fn device(info: web::Path<Info>, data: web::Data<Devices>) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let device = match device_list_data
        .into_iter()
        .find(|device| device.name == info.device_name)
    {
        Some(device) => device.clone(),
        None => Device {
            name: String::from("NA"),
            address: [0, 0, 0, 0, 0, 0],
            states: HashMap::new(),
        },
    };

    let device_serialized = Device::default_with_name(device.name.clone());

    Ok(web::Json(device_serialized))
}

pub async fn device_states(
    info: web::Path<Info>,
    data: web::Data<Devices>,
) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let device_states = match device_list_data
        .into_iter()
        .find(|device| device.name == info.device_name)
    {
        Some(device) => device.states.clone(),
        None => HashMap::new(),
    };

    let device_states_serialized = DeviceStatesSerialize {
        states: device_states,
    };

    Ok(web::Json(device_states_serialized))
}

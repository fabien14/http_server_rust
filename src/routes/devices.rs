use crate::framework_ble::{Device, DeviceName, DeviceStates};

use actix_web::{web, Responder, Result};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Devices {
    pub devices: Vec<Device>,
}

#[derive(Clone, Serialize)]
pub struct DevicesSerializer {
    pub devices: Vec<DeviceName>,
}

#[derive(Serialize)]
struct DeviceStatesSerialize {
    states: HashMap<String, String>,
}

pub async fn devices(data: web::Data<Devices>) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let mut dev: Vec<DeviceName> = Vec::new();
    for d in device_list_data {
        dev.push(d.name.clone());
    }

    let devices_list_struct = DevicesSerializer { devices: dev };

    Ok(web::Json(devices_list_struct))
}

pub async fn device(info: web::Path<DeviceName>, data: web::Data<Devices>) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let device_name = match device_list_data
        .into_iter()
        .find(|device| device.name == info.clone())
    {
        Some(device) => device.name.clone(),
        None => DeviceName(String::from("NA")),
    };

    let device_serialized = device_name;

    Ok(web::Json(device_serialized))
}

pub async fn device_states(
    info: web::Path<DeviceName>,
    data: web::Data<Devices>,
) -> Result<impl Responder> {
    let device_list_data = &data.devices;

    let device_states = match device_list_data
        .into_iter()
        .find(|device| device.name == info.clone())
    {
        Some(device) => device.states.clone(),
        None => DeviceStates(HashMap::new()),
    };

    Ok(web::Json(device_states))
}

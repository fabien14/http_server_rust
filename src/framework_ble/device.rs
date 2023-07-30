use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct DeviceAddress(pub [u8; 6]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceName(pub String);

impl PartialEq for DeviceName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceColor(pub String);

#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceStates(pub HashMap<String, DeviceColor>);

#[derive(Clone)]
pub struct Device {
    pub name: DeviceName,
    pub address: DeviceAddress,
    pub states: DeviceStates,
}

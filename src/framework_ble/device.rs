use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Default)]
pub struct Device {
    pub name: String,
    #[serde(skip_serializing)]
    pub address: [u8; 6],
    #[serde(skip_serializing)]
    pub states: HashMap<String, String>,
}

impl Device {
    pub fn default_with_name(name: String) -> Self {
        let mut device = Self::default();
        device.name = name;
        device
    }
}

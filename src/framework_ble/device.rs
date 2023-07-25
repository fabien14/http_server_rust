use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    pub adress: [u8; 6],
    pub states: HashMap<String, String>,
}

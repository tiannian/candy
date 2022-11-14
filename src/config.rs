use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IpxeConfig {
    pub enable: bool,
    pub script_file: Option<String>,
    pub script: Option<String>,
    pub script_url: Option<String>,
    pub uuid: Option<String>,
    pub ip: Option<String>,
    pub mac: Option<String>,
    pub serial: Option<String>,
    pub uefi: Option<bool>,
    pub bios: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonConfig {
    pub listen: Vec<String>,
    pub enable_swagger_ui: bool,
    pub enable_item_list: bool,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub config: CommonConfig,

    pub ipxe: HashMap<String, IpxeConfig>,
}

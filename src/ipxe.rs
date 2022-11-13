use tokio::fs;

use crate::config::{Config, IpxeConfig};

pub const EMPTY_IPXE: &'static str = "#!ipxe\n";

pub async fn get_script(v: IpxeConfig) -> String {
    if let Some(s) = v.script {
        return s;
    }

    if let Some(s) = v.script_file {
        if let Ok(s) = fs::read_to_string(s).await {
            return s;
        }
    }

    String::from(EMPTY_IPXE)
}

pub async fn ipxe_script(
    config: &str,
    uuid: &Option<String>,
    ip: &Option<String>,
    mac: &Option<String>,
    serial: &Option<String>,
    platform: &Option<String>,
) -> String {
    if let Ok(s) = fs::read_to_string(config).await {
        if let Ok(t) = toml::from_str::<Config>(&s) {
                println!("{:?}", t);
            for (_, v) in t.ipxe {
                println!("{:?}", v);

                if v.enable && uuid == &v.uuid && ip == &v.ip && mac == &v.mac && serial == &v.serial {
                    let uefi = Some(String::from("efi"));

                    if v.uefi.unwrap_or(false) && platform == &uefi {
                        return get_script(v).await;
                    }

                    let bios = Some(String::from("pcbios"));

                    if v.bios.unwrap_or(false) && platform == &bios {
                        return get_script(v).await;
                    }
                }
            }
        }
    }

    String::from(EMPTY_IPXE)
}

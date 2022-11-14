use tokio::fs;

use crate::{
    config::{Config, IpxeConfig},
    Result,
};

pub const EMPTY_IPXE: &'static str = "#!ipxe\n";

pub async fn get_script(v: IpxeConfig) -> Result<String> {
    if let Some(s) = v.script {
        return Ok(s);
    }

    if let Some(p) = v.script_file {
        let s = fs::read_to_string(p).await?;

        return Ok(s);
    }

    Ok(String::from(EMPTY_IPXE))
}

async fn ipxe_script_inner(
    config: &str,
    uuid: &Option<String>,
    ip: &Option<String>,
    mac: &Option<String>,
    serial: &Option<String>,
    platform: &Option<String>,
) -> Result<String> {
    let s = fs::read_to_string(config).await?;

    let t: Config = toml::from_str(&s)?;

    for (_, v) in t.ipxe {
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

    Ok(String::from(EMPTY_IPXE))
}

pub async fn ipxe_script(
    config: &str,
    uuid: &Option<String>,
    ip: &Option<String>,
    mac: &Option<String>,
    serial: &Option<String>,
    platform: &Option<String>,
) -> String {
    match ipxe_script_inner(config, uuid, ip, mac, serial, platform).await {
        Ok(s) => s,
        Err(e) => {
            log::error!("ipxe match error: {:?}", e);
            String::from(EMPTY_IPXE)
        }
    }
}

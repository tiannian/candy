use tokio::fs::File;

pub async fn ipxe_script(
    config: &str,
    uuid: &Option<String>,
    ip: &Option<String>,
    mac: &Option<String>,
    serial: &Option<String>,
    platform: &Option<String>,
) -> String {
    if let Ok(file) = File::open(config).await {}

    String::new()
}

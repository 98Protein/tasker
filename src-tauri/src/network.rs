use serde::Serialize;

#[derive(Serialize)]
pub struct NetworkInfo {
    pub ip_address: String,
    pub mac_address: String,
}

#[tauri::command]
pub fn get_network_info() -> Result<NetworkInfo, String> {
    let ip = get_local_ip()?;
    let mac = get_mac_address()?;

    Ok(NetworkInfo {
        ip_address: ip,
        mac_address: mac,
    })
}

fn get_local_ip() -> Result<String, String> {
    local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .map_err(|e| format!("Failed to get IP address: {}", e))
}

fn get_mac_address() -> Result<String, String> {
    mac_address::get_mac_address()
        .map_err(|e| format!("Failed to get MAC address: {}", e))?
        .map(|mac| mac.to_string())
        .ok_or_else(|| "No MAC address found".to_string())
}

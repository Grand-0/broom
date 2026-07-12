use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandInfo {
    pub target_server_name: String,
    pub target_account: String,
    pub instance_dir: String,
    pub app_name: String,
    pub db_provider: String,
    pub db_server: String,
    pub db_owner: String,
    pub db_admin: String,
    pub web_server_name: String,
    pub port_a: u16,
    pub port_b: u16,
    pub smb_server_address: String,
}

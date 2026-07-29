use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandInfo {
    #[serde(rename = "targetServerName")]
    pub target_server_name: String,
    #[serde(rename = "targetAccount")]
    pub target_account: String,
    #[serde(rename = "instanceDir")]
    pub instance_dir: String,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "dbProvider")]
    pub db_provider: String,
    #[serde(rename = "dbServer")]
    pub db_server: String,
    #[serde(rename = "dbOwner")]
    pub db_owner: String,
    #[serde(rename = "dbAdmin")]
    pub db_admin: String,
    #[serde(rename = "webServerName")]
    pub web_server_name: String,
    #[serde(rename = "portA")]
    pub port_a: u16,
    #[serde(rename = "portB")]
    pub port_b: u16,
    #[serde(rename = "smbServerAddress")]
    pub smb_server_address: String,
}

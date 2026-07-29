use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionsConfig {
    pub collections: HashMap<String, Collection>,
    pub defaults: Defaults,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Collection {
    pub path_prefix: String,
    pub tfvs_collection_uri: String,
    pub tfvs_team_project: String,
    pub stand_domain: String,
    pub projects: HashMap<String, Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub collection_path_suffix: String,
    pub default_instance_dir: String,
    pub default_target_account: String,
    pub versions: HashMap<String, Version>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub configure_vm: ConfigureVm,
    pub deploy: DeployParams,
    pub stages: HashMap<String, Stage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stage {
    pub stand_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigureVm {
    pub dotnet: String,
    pub postgresql: String,
    pub nginx: String,
    pub elasticsearch: Option<String>,
    pub kafka: Option<String>,
    pub chromium: Option<String>,
    pub samba: Option<String>,
    #[serde(rename = "caTrust")]
    pub ca_trust: bool,
    pub keycloak: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeployParams {
    pub db_provider: String,
    pub db_server: String,
    pub db_owner: String,
    pub db_admin: String,
    pub create_db: String,
    pub load_initial_data: Option<String>,
    pub reg_host: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Defaults {
    pub temp_files_path: String,
}

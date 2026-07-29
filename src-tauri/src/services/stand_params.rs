use crate::constants;
use crate::models::collection::{Collection, Project, Stage};

pub struct StandContext {
    pub stand_id: String,
    pub collection_name: String,
    pub project_name: String,
    pub web_server_name: String,
    pub target_server_name: String,
    pub app_name: String,
    pub instance_dir: String,
    pub target_account: String,
    pub port_a: u16,
    pub port_b: u16,
    pub smb_server_address: String,
}

pub fn generate_stand_context(
    collection_name: &str,
    collection: &Collection,
    project_name: &str,
    project: &Project,
    stage: &Stage,
    version: &str,
    stand_id: &str,
) -> StandContext {
    let domain = &collection.stand_domain;
    let stand_prefix = &stage.stand_prefix;
    let web_server_name =
        format!("{}-{}-{}.{}", stand_prefix, version, stand_id, domain);

    let version_compact = version.replace('.', "");
    let app_name = format!(
        "{}_{}-{}",
        collection_name.to_lowercase(),
        version_compact,
        stand_id
    );

    let id_num: u32 = stand_id.parse().unwrap_or(1);
    let port_a = constants::PORT_RANGE_START + ((id_num - 1) * 2) as u16;
    let port_b = port_a + 1;

    StandContext {
        stand_id: stand_id.to_string(),
        collection_name: collection_name.to_string(),
        project_name: project_name.to_string(),
        web_server_name: web_server_name.clone(),
        target_server_name: web_server_name.clone(),
        app_name,
        instance_dir: project.default_instance_dir.clone(),
        target_account: project.default_target_account.clone(),
        port_a,
        port_b,
        smb_server_address: web_server_name,
    }
}

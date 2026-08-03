use serde::{Deserialize, Serialize};

// ── Form data (get_create_stand_form_data) ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateStandFormData {
    pub collections: Vec<CollectionFormItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionFormItem {
    #[serde(rename = "collectionName")]
    pub collection_name: String,
    pub projects: Vec<ProjectFormItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectFormItem {
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub versions: Vec<VersionFormItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionFormItem {
    pub version: String,
    pub stages: Vec<StageFormItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageFormItem {
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

// ── Create stand request / response ──

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStandRequest {
    pub collection: String,
    pub project: String,
    pub version: String,
    pub stage: String,
    pub build_option: String,
    pub build_version: Option<String>,
    pub use_elastic: bool,
    pub use_kafka: bool,
    pub temp_files_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStandResponse {
    pub status: String,
    pub log_info: Option<LogInfo>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LogType {
    Session,
    Operation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogInfo {
    pub log_type: LogType,
    pub log_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateStandError {
    pub error_msg: String,
    pub log_info: Option<LogInfo>,
}

impl CreateStandError {
    pub fn simple(message: impl Into<String>) -> Self {
        Self {
            error_msg: message.into(),
            log_info: None,
        }
    }
}

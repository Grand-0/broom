use serde::{Deserialize, Serialize};

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

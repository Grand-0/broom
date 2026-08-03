use crate::models::CollectionsConfig;
use crate::services::collections_config;
use crate::view_models::{
    CollectionFormItem, CreateStandFormData, ProjectFormItem, StageFormItem, VersionFormItem,
};

#[tauri::command]
pub fn get_create_stand_form_data(app: tauri::AppHandle) -> Result<CreateStandFormData, String> {
    let config: CollectionsConfig = collections_config::load_from_resources(&app)?;

    let collections: Vec<CollectionFormItem> = config
        .collections
        .into_iter()
        .map(|(collection_name, collection)| {
            let projects: Vec<ProjectFormItem> = collection
                .projects
                .into_iter()
                .map(|(project_name, project)| {
                    let versions: Vec<VersionFormItem> = project
                        .versions
                        .into_iter()
                        .map(|(version_key, version)| {
                            let stages: Vec<StageFormItem> = version
                                .stages
                                .into_iter()
                                .map(|(stage_name, _stage)| StageFormItem { stage_name })
                                .collect();

                            VersionFormItem {
                                version: version_key,
                                stages,
                            }
                        })
                        .collect();

                    ProjectFormItem {
                        project_name,
                        versions,
                    }
                })
                .collect();

            CollectionFormItem {
                collection_name,
                projects,
            }
        })
        .collect();

    Ok(CreateStandFormData { collections })
}

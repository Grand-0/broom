use crate::models::CollectionsConfig;
use crate::view_models::{
    CollectionFormItem, CreateStandFormData, ProjectFormItem, StageFormItem, VersionFormItem,
};
use tauri::Manager;

#[tauri::command]
pub fn get_create_stand_form_data(app: tauri::AppHandle) -> Result<CreateStandFormData, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;
    let collections_path = resource_path.join("resources/collections.json");

    let file_content = std::fs::read_to_string(&collections_path)
        .map_err(|e| format!("Ошибка чтения collections.json: {}", e))?;

    let config: CollectionsConfig = serde_json::from_str(&file_content)
        .map_err(|e| format!("Ошибка парсинга collections.json: {}", e))?;

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

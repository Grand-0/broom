use tauri::Manager;

use crate::models::collection::{Collection, CollectionsConfig, ConfigureVm, DeployParams, Project, Stage, Version};

pub struct ResolvedConfig {
    pub configure_vm: ConfigureVm,
    pub deploy: DeployParams,
    pub script_path: String,
    pub temp_files_path: String,
    pub build: String,
}

pub fn load_from_resources(app: &tauri::AppHandle) -> Result<CollectionsConfig, String> {
    let resource_path = app.path().resource_dir().map_err(|e| e.to_string())?;
    let collections_path = resource_path.join("resources/collections.json");

    let file_content = std::fs::read_to_string(&collections_path)
        .map_err(|e| format!("Ошибка чтения collections.json: {}", e))?;

    serde_json::from_str(&file_content)
        .map_err(|e| format!("Ошибка парсинга collections.json: {}", e))
}

pub fn resolve(
    config: &CollectionsConfig,
    collection_name: &str,
    project_name: &str,
    version_key: &str,
    stage_name: &str,
    build: &str,
    use_elastic: bool,
    use_kafka: bool,
    temp_files_path: Option<String>,
) -> Result<ResolvedConfig, String> {
    let collection: &Collection = config
        .collections
        .get(collection_name)
        .ok_or_else(|| format!("Коллекция {} не найдена", collection_name))?;
    let project: &Project = collection
        .projects
        .get(project_name)
        .ok_or_else(|| format!("Проект {} не найден", project_name))?;
    let version: &Version = project
        .versions
        .get(version_key)
        .ok_or_else(|| format!("Версия {} не найдена", version_key))?;
    let stage: &Stage = version
        .stages
        .get(stage_name)
        .ok_or_else(|| format!("Стадия {} не найдена", stage_name))?;

    let mut configure_vm = version.configure_vm.clone();
    if !use_elastic {
        configure_vm.elasticsearch = None;
    }
    if !use_kafka {
        configure_vm.kafka = None;
    }

    let script_path = format!(
        "\\\\tfbd\\storage\\{}\\{}\\{}-{}\\{}\\BuildResult",
        collection.path_prefix,
        project.collection_path_suffix,
        stage.stand_prefix,
        version_key,
        build,
    );

    let temp_files_path = temp_files_path
        .unwrap_or_else(|| config.defaults.temp_files_path.clone());

    Ok(ResolvedConfig {
        configure_vm,
        deploy: version.deploy.clone(),
        script_path,
        temp_files_path,
        build: build.to_string(),
    })
}

use std::process::Command;

use tauri::Manager;

use crate::models::collection::ConfigureVm;
use crate::services::{collections_config, ps_executor, stand_params, vm_operations};
use crate::store::{stand_id_generator, stands_store};
use crate::view_models::{CreateStandRequest, CreateStandResponse, StandInfo};

#[tauri::command]
pub async fn create_stand(
    app: tauri::AppHandle,
    request: CreateStandRequest,
) -> Result<CreateStandResponse, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    // 1. Load collections config
    let config = collections_config::load_from_resources(&app)?;

    // 2. Resolve build version
    let build = resolve_build(&app, &config, &request).await?;

    // 3. Resolve deployment params
    let resolved = collections_config::resolve(
        &config,
        &request.collection,
        &request.project,
        &request.version,
        &request.stage,
        &build,
        request.use_elastic,
        request.use_kafka,
        request.temp_files_path,
    )?;

    // 4. Generate stand ID
    let stand_id = stand_id_generator::generate_id(&app_data_dir)?;

    // 5. Look up model entities for context generation
    let collection = config
        .collections
        .get(&request.collection)
        .ok_or_else(|| format!("Коллекция {} не найдена", request.collection))?;
    let project = collection
        .projects
        .get(&request.project)
        .ok_or_else(|| format!("Проект {} не найден", request.project))?;
    let version = project
        .versions
        .get(&request.version)
        .ok_or_else(|| format!("Версия {} не найдена", request.version))?;
    let stage = version
        .stages
        .get(&request.stage)
        .ok_or_else(|| format!("Стадия {} не найдена", request.stage))?;

    // 6. Generate stand context (names, ports, etc.)
    let ctx = stand_params::generate_stand_context(
        &request.collection,
        collection,
        &request.project,
        project,
        stage,
        &request.version,
        &stand_id,
    );

    // 7. Create VM
    vm_operations::create_vm(&app_data_dir, &ctx.web_server_name)?;

    // 8. Run configure_vm.ps1
    let cfg_path = format!("{}\\configure_vm.ps1", resolved.script_path);
    let cfg_args = build_configure_vm_args(&resolved.configure_vm);
    let result = ps_executor::execute_script(
        &app_data_dir,
        &cfg_path,
        &cfg_args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        "configure_vm",
        &ctx.web_server_name,
    );
    if !result.success {
        return Ok(CreateStandResponse { status: "fail".to_string(), log_path: Some(result.log_path) });
    }

    // 9. Run deploy_linux.ps1
    let deploy_path = format!("{}\\deploy_linux.ps1", resolved.script_path);
    let deploy_args = build_deploy_args(&ctx, &resolved);
    let result = ps_executor::execute_script(
        &app_data_dir,
        &deploy_path,
        &deploy_args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        "deploy",
        &ctx.web_server_name,
    );
    if !result.success {
        return Ok(CreateStandResponse { status: "fail".to_string(), log_path: Some(result.log_path) });
    }

    // 10. Persist stand info
    let stand_info = StandInfo {
        target_server_name: ctx.target_server_name,
        target_account: ctx.target_account,
        instance_dir: ctx.instance_dir,
        app_name: ctx.app_name,
        db_provider: resolved.deploy.db_provider,
        db_server: resolved.deploy.db_server,
        db_owner: resolved.deploy.db_owner,
        db_admin: resolved.deploy.db_admin,
        web_server_name: ctx.web_server_name,
        port_a: ctx.port_a,
        port_b: ctx.port_b,
        smb_server_address: ctx.smb_server_address,
    };

    stands_store::add_stand(&app_data_dir, stand_info)?;

    Ok(CreateStandResponse { status: "ok".to_string(), log_path: None })
}

async fn resolve_build(
    app: &tauri::AppHandle,
    config: &crate::models::collection::CollectionsConfig,
    request: &CreateStandRequest,
) -> Result<String, String> {
    if request.build_option != "latest" {
        return request
            .build_version
            .clone()
            .ok_or_else(|| "build_version обязателен, если build_option не latest".into());
    }

    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    let collection = config
        .collections
        .get(&request.collection)
        .ok_or_else(|| format!("Коллекция {} не найдена", request.collection))?;
    let project = collection
        .projects
        .get(&request.project)
        .ok_or_else(|| format!("Проект {} не найден", request.project))?;
    let version = project
        .versions
        .get(&request.version)
        .ok_or_else(|| format!("Версия {} не найдена", request.version))?;
    let stage = version
        .stages
        .get(&request.stage)
        .ok_or_else(|| format!("Стадия {} не найдена", request.stage))?;

    let ps_command = format!(
        "CI-Get-LastLocation -CollectionUri {} -TeamProject {} -BuildDefinition \\{} -BranchName {}/{}",
        collection.tfvs_collection_uri,
        collection.tfvs_team_project,
        project.collection_path_suffix,
        stage.stand_prefix,
        request.version,
    );

    let logs_dir = app_data_dir.join("logs");
    let _ = std::fs::create_dir_all(&logs_dir);

    let powershell = std::env::var("SystemRoot")
        .map(|root| format!("{}\\System32\\WindowsPowerShell\\v1.0\\powershell.exe", root))
        .unwrap_or_else(|_| "powershell.exe".into());

    let output = Command::new(&powershell)
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&ps_command)
        .output()
        .map_err(|e| format!("Ошибка запуска CI-Get-LastLocation: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("CI-Get-LastLocation завершился с ошибкой: {}", stderr.trim()))
    }
}

fn build_configure_vm_args(cfg: &ConfigureVm) -> Vec<String> {
    let mut args = Vec::new();

    push_arg(&mut args, "-Dotnet", &cfg.dotnet);
    push_arg(&mut args, "-Postgresql", &cfg.postgresql);
    push_arg(&mut args, "-Nginx", &cfg.nginx);

    if let Some(ref v) = cfg.elasticsearch {
        push_arg(&mut args, "-Elasticsearch", v);
    }
    if let Some(ref v) = cfg.kafka {
        push_arg(&mut args, "-Kafka", v);
    }
    if let Some(ref v) = cfg.chromium {
        push_arg(&mut args, "-Chromium", v);
    }
    if let Some(ref v) = cfg.samba {
        push_arg(&mut args, "-Samba", v);
    }
    if cfg.ca_trust {
        args.push("-CaTrust".to_string());
    }
    if let Some(ref v) = cfg.keycloak {
        push_arg(&mut args, "-Keycloak", v);
    }

    args
}

fn build_deploy_args(
    ctx: &stand_params::StandContext,
    resolved: &collections_config::ResolvedConfig,
) -> Vec<String> {
    let mut args = Vec::new();

    push_arg(&mut args, "-TargetServerName", &ctx.target_server_name);
    push_arg(&mut args, "-InstanceDir", &ctx.instance_dir);
    push_arg(&mut args, "-AppName", &ctx.app_name);
    push_arg(&mut args, "-DBProvider", &resolved.deploy.db_provider);
    push_arg(&mut args, "-DBServer", &resolved.deploy.db_server);
    push_arg(&mut args, "-DBAdmin", &resolved.deploy.db_admin);
    push_arg(&mut args, "-DBOwner", &resolved.deploy.db_owner);
    push_arg(&mut args, "-CreateDB", &resolved.deploy.create_db);
    if let Some(ref v) = resolved.deploy.load_initial_data {
        push_arg(&mut args, "-LoadInitialData", v);
    }
    push_arg(&mut args, "-WebServerName", &ctx.web_server_name);
    if resolved.deploy.reg_host {
        args.push("-RegHost".to_string());
    }
    push_arg(&mut args, "-PortA", &ctx.port_a.to_string());
    push_arg(&mut args, "-PortB", &ctx.port_b.to_string());
    push_arg(&mut args, "-SmbServerAddress", &ctx.smb_server_address);
    push_arg(&mut args, "-TempFilesPath", &resolved.temp_files_path);

    args
}

fn push_arg(args: &mut Vec<String>, name: &str, value: &str) {
    args.push(name.to_string());
    args.push(value.to_string());
}

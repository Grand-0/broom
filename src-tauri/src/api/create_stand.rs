use std::path::Path;

use tauri::Manager;

use crate::models::collection::{Collection, ConfigureVm, Project, Stage};
use crate::services::{collections_config, ps_executor, stand_params, vm_operations};
use crate::store::{stand_id_generator, stands_store};
use crate::view_models::{
    CreateStandError, CreateStandRequest, CreateStandResponse, LogInfo, LogType, StandInfo,
};

#[tauri::command]
pub fn create_stand(
    app: tauri::AppHandle,
    request: CreateStandRequest,
) -> Result<CreateStandResponse, CreateStandError> {
    let result = run_create_stand(&app, &request);

    match &result {
        Ok(_) => log::info!("create_stand finished: status=ok"),
        Err(e) => log::warn!(
            "create_stand finished: status=fail error={}",
            e.error_msg
        ),
    }

    result
}

fn run_create_stand(
    app: &tauri::AppHandle,
    request: &CreateStandRequest,
) -> Result<CreateStandResponse, CreateStandError> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| CreateStandError::simple(format!("Не удалось определить каталог данных: {}", e)))?;

    log::info!(
        "create_stand started: collection={} project={} version={} stage={} build_option={}",
        request.collection,
        request.project,
        request.version,
        request.stage,
        request.build_option
    );

    // 1. Load collections config
    let config = collections_config::load_from_resources(app).map_err(CreateStandError::simple)?;

    // 2. Look up model entities
    let collection = config
        .collections
        .get(&request.collection)
        .ok_or_else(|| CreateStandError::simple(format!("Коллекция {} не найдена", request.collection)))?;
    let project = collection
        .projects
        .get(&request.project)
        .ok_or_else(|| CreateStandError::simple(format!("Проект {} не найден", request.project)))?;
    let version = project
        .versions
        .get(&request.version)
        .ok_or_else(|| CreateStandError::simple(format!("Версия {} не найдена", request.version)))?;
    let stage = version
        .stages
        .get(&request.stage)
        .ok_or_else(|| CreateStandError::simple(format!("Стадия {} не найдена", request.stage)))?;

    // 3. Generate stand ID
    let stand_id = stand_id_generator::generate_id(&app_data_dir).map_err(CreateStandError::simple)?;

    // 4. Generate stand context (names, ports, etc.)
    let ctx = stand_params::generate_stand_context(
        &request.collection,
        collection,
        &request.project,
        project,
        stage,
        &request.version,
        &stand_id,
    );
    log::info!(
        "create_stand: stand={} app={} ports={}/{}",
        ctx.web_server_name,
        ctx.app_name,
        ctx.port_a,
        ctx.port_b
    );

    // 5. Create per-call client log (named by the stand being created)
    let client_log =
        ps_executor::session_log_path(&app_data_dir, "create_stand", &ctx.web_server_name);
    ps_executor::append_log_line(
        &client_log,
        &format!(
            "create_stand started: collection={} project={} version={} stage={} build_option={}",
            request.collection,
            request.project,
            request.version,
            request.stage,
            request.build_option
        ),
    );
    ps_executor::append_log_line(
        &client_log,
        &format!(
            "stand id: {} | stand: {} | app: {} | ports: {}/{}",
            stand_id, ctx.web_server_name, ctx.app_name, ctx.port_a, ctx.port_b
        ),
    );

    // 6. Resolve build version
    let build = resolve_build(
        app,
        request,
        collection,
        project,
        stage,
        &ctx.web_server_name,
        &client_log,
    )?;
    log::info!("create_stand: build resolved: {}", build);
    ps_executor::append_log_line(&client_log, &format!("build resolved: {}", build));

    // 7. Resolve deployment params
    let resolved = collections_config::resolve(
        &config,
        &request.collection,
        &request.project,
        &request.version,
        &request.stage,
        &build,
        request.use_elastic,
        request.use_kafka,
        request.temp_files_path.clone(),
    )
    .map_err(CreateStandError::simple)?;

    // 8. Create VM
    ps_executor::append_log_line(&client_log, "step: create_vm (New-VM)");
    match vm_operations::create_vm(&ctx.web_server_name) {
        Ok(_) => ps_executor::append_log_line(&client_log, "create_vm: ok"),
        Err(e) => {
            ps_executor::append_log_line(&client_log, &format!("create_vm: failed: {}", e));
            return Err(CreateStandError {
                error_msg: "Создание виртуальной машины завершилось с ошибкой.".to_string(),
                log_info: Some(session_log_info(&client_log)),
            });
        }
    }

    // 9. Run configure_vm.ps1
    let cfg_path = format!("{}\\configure_vm.ps1", resolved.script_path);
    let cfg_args = build_configure_vm_args(&resolved.configure_vm);
    let configure_result = ps_executor::execute_script(
        &app_data_dir,
        &cfg_path,
        &cfg_args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        "configure_vm",
        &ctx.web_server_name,
        Some(&client_log),
    );
    if !configure_result.success {
        return Err(CreateStandError {
            error_msg: "Настройка виртуальной машины (configure_vm) завершилась с ошибкой.".to_string(),
            log_info: Some(session_log_info(&client_log)),
        });
    }

    // 10. Run deploy_linux.ps1
    let deploy_path = format!("{}\\deploy_linux.ps1", resolved.script_path);
    let deploy_args = build_deploy_args(&ctx, &resolved);
    let deploy_result = ps_executor::execute_script(
        &app_data_dir,
        &deploy_path,
        &deploy_args.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        "deploy",
        &ctx.web_server_name,
        Some(&client_log),
    );
    if !deploy_result.success {
        return Err(CreateStandError {
            error_msg: "Развёртывание приложения (deploy) завершилось с ошибкой.".to_string(),
            log_info: Some(session_log_info(&client_log)),
        });
    }

    // 11. Persist stand info
    let stand_name = ctx.web_server_name.clone();
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

    stands_store::add_stand(&app_data_dir, stand_info).map_err(|e| CreateStandError {
        error_msg: e,
        log_info: Some(session_log_info(&client_log)),
    })?;

    log::info!("create_stand: stand saved: {}", stand_name);
    ps_executor::append_log_line(&client_log, "stand saved");

    Ok(CreateStandResponse {
        status: "ok".to_string(),
        log_info: Some(session_log_info(&client_log)),
    })
}

fn session_log_info(log_path: &Path) -> LogInfo {
    LogInfo {
        log_type: LogType::Session,
        log_name: log_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    }
}

fn resolve_build(
    app: &tauri::AppHandle,
    request: &CreateStandRequest,
    collection: &Collection,
    project: &Project,
    stage: &Stage,
    stand_name: &str,
    client_log: &Path,
) -> Result<String, CreateStandError> {
    if request.build_option != "latest" {
        return request
            .build_version
            .clone()
            .ok_or_else(|| CreateStandError::simple("build_version обязателен, если build_option не latest"));
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| CreateStandError::simple(format!("Не удалось определить каталог данных: {}", e)))?;

    let ps_command = format!(
        "CI-Get-LastLocation -CollectionUri {} -TeamProject {} -BuildDefinition \\{} -BranchName {}/{}",
        collection.tfvs_collection_uri,
        collection.tfvs_team_project,
        project.collection_path_suffix,
        stage.stand_prefix,
        request.version,
    );

    let result = ps_executor::execute_command(
        &app_data_dir,
        &ps_command,
        "ci_get_last_location",
        stand_name,
        Some(client_log),
    );

    if result.success {
        Ok(result.stdout.trim().to_string())
    } else {
        Err(CreateStandError {
            error_msg: "CI-Get-LastLocation завершился с ошибкой.".to_string(),
            log_info: Some(session_log_info(client_log)),
        })
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

#[cfg(test)]
mod tests {
    use super::{build_configure_vm_args, build_deploy_args};
    use crate::models::collection::{
        Collection, CollectionsConfig, ConfigureVm, DeployParams, Defaults, Project, Stage, Version,
    };
    use crate::services::collections_config::{self, ResolvedConfig};
    use crate::services::stand_params::{self, StandContext};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn load_config() -> CollectionsConfig {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/collections.json");
        let content = std::fs::read_to_string(&path).expect("collections.json должен читаться");
        serde_json::from_str(&content).expect("collections.json должен парситься")
    }

    fn delo_entities<'a>(config: &'a CollectionsConfig) -> (&'a Collection, &'a Project, &'a Stage) {
        let collection = config.collections.get("Delo2020").unwrap();
        let project = collection.projects.get("Delo2020").unwrap();
        let stage = project
            .versions
            .get("26.2")
            .unwrap()
            .stages
            .get("dev")
            .unwrap();
        (collection, project, stage)
    }

    fn delo_ctx(config: &CollectionsConfig, stand_id: &str) -> (StandContext, ResolvedConfig) {
        let (collection, project, stage) = delo_entities(config);
        let resolved = collections_config::resolve(
            config,
            "Delo2020",
            "Delo2020",
            "26.2",
            "dev",
            "20230403.53",
            true,
            true,
            None,
        )
        .unwrap();
        let ctx = stand_params::generate_stand_context(
            "Delo2020",
            collection,
            "Delo2020",
            project,
            stage,
            "26.2",
            stand_id,
        );
        (ctx, resolved)
    }

    #[test]
    fn script_path_is_built_from_config_and_build() {
        let config = load_config();
        let resolved = collections_config::resolve(
            &config,
            "Delo2020",
            "Delo2020",
            "26.2",
            "dev",
            "20230403.53",
            true,
            true,
            None,
        )
        .unwrap();

        assert_eq!(
            resolved.script_path,
            r"\\tfbd\storage\_Delo2020-Collection2020\Delo2020\dev-stand-26.2\20230403.53\BuildResult"
        );
        assert_eq!(resolved.temp_files_path, "$env:TEMP\\deploy_stand");
    }

    #[test]
    fn deploy_args_match_resolved_config_and_context() {
        let config = load_config();
        let (ctx, resolved) = delo_ctx(&config, "1");

        assert_eq!(ctx.web_server_name, "dev-stand-26.2-1.mr.loc");
        assert_eq!(ctx.app_name, "delo2020_262-1");
        assert_eq!(ctx.instance_dir, "/opt/delo");
        assert_eq!(ctx.port_a, 10001);
        assert_eq!(ctx.port_b, 10002);

        let args = build_deploy_args(&ctx, &resolved);
        let expected = vec![
            "-TargetServerName".to_string(),
            ctx.target_server_name.clone(),
            "-InstanceDir".to_string(),
            "/opt/delo".to_string(),
            "-AppName".to_string(),
            ctx.app_name.clone(),
            "-DBProvider".to_string(),
            "PostgreSQL".to_string(),
            "-DBServer".to_string(),
            "localhost".to_string(),
            "-DBAdmin".to_string(),
            "postgres".to_string(),
            "-DBOwner".to_string(),
            "X262".to_string(),
            "-CreateDB".to_string(),
            "Recreate".to_string(),
            "-LoadInitialData".to_string(),
            "demo_csv_template".to_string(),
            "-WebServerName".to_string(),
            ctx.web_server_name.clone(),
            "-RegHost".to_string(),
            "-PortA".to_string(),
            "10001".to_string(),
            "-PortB".to_string(),
            "10002".to_string(),
            "-SmbServerAddress".to_string(),
            ctx.smb_server_address.clone(),
            "-TempFilesPath".to_string(),
            "$env:TEMP\\deploy_stand".to_string(),
        ];
        assert_eq!(args, expected);
    }

    #[test]
    fn deploy_args_omit_load_initial_data_when_null() {
        let config = load_config();
        let collection = config.collections.get("Archive2020").unwrap();
        let project = collection.projects.get("Archive2020").unwrap();
        let stage = project
            .versions
            .get("1.0")
            .unwrap()
            .stages
            .get("dev")
            .unwrap();
        let resolved = collections_config::resolve(
            &config,
            "Archive2020",
            "Archive2020",
            "1.0",
            "dev",
            "20230403.53",
            false,
            false,
            None,
        )
        .unwrap();
        let ctx = stand_params::generate_stand_context(
            "Archive2020",
            collection,
            "Archive2020",
            project,
            stage,
            "1.0",
            "1",
        );
        let args = build_deploy_args(&ctx, &resolved);

        assert!(!args.iter().any(|a| a == "-LoadInitialData"));
    }

    #[test]
    fn deploy_args_omit_reg_host_when_false() {
        for (config, expect_host) in [(synthetic_config(true), true), (synthetic_config(false), false)]
        {
            let collection = config.collections.get("Coll").unwrap();
            let project = collection.projects.get("Proj").unwrap();
            let stage = project
                .versions
                .get("1.0")
                .unwrap()
                .stages
                .get("dev")
                .unwrap();
            let resolved = collections_config::resolve(
                &config,
                "Coll",
                "Proj",
                "1.0",
                "dev",
                "b1",
                true,
                true,
                None,
            )
            .unwrap();
            let ctx = stand_params::generate_stand_context(
                "Coll",
                collection,
                "Proj",
                project,
                stage,
                "1.0",
                "1",
            );
            let args = build_deploy_args(&ctx, &resolved);

            assert_eq!(
                args.iter().any(|a| a == "-RegHost"),
                expect_host,
                "ожидалось -RegHost={}",
                expect_host
            );
        }
    }

    #[test]
    fn configure_vm_args_follow_use_elastic_and_use_kafka() {
        let config = load_config();

        let with = collections_config::resolve(
            &config,
            "Delo2020",
            "Delo2020",
            "26.2",
            "dev",
            "b",
            true,
            true,
            None,
        )
        .unwrap();
        let args = build_configure_vm_args(&with.configure_vm);
        assert!(args.iter().any(|a| a == "-Dotnet"));
        assert!(args.iter().any(|a| a == "8-repo 10-repo"));
        assert!(args.iter().any(|a| a == "-Postgresql"));
        assert!(args.iter().any(|a| a == "17-repo"));
        assert!(args.iter().any(|a| a == "-Nginx"));
        assert!(args.iter().any(|a| a == "-Chromium"));
        assert!(args.iter().any(|a| a == "-Samba"));
        assert!(args.iter().any(|a| a == "-Elasticsearch"));
        assert!(args.iter().any(|a| a == "7.15.0"));
        assert!(args.iter().any(|a| a == "-Kafka"));
        assert!(args.iter().any(|a| a == "2.13-3.2.0"));
        assert!(!args.iter().any(|a| a == "-CaTrust"));
        assert!(!args.iter().any(|a| a == "-Keycloak"));

        let without = collections_config::resolve(
            &config,
            "Delo2020",
            "Delo2020",
            "26.2",
            "dev",
            "b",
            false,
            false,
            None,
        )
        .unwrap();
        let args = build_configure_vm_args(&without.configure_vm);
        assert!(!args.iter().any(|a| a == "-Elasticsearch"));
        assert!(!args.iter().any(|a| a == "-Kafka"));
    }

    #[test]
    fn full_command_line_matches_expected_shape() {
        let config = load_config();
        let (ctx, resolved) = delo_ctx(&config, "1");
        let args = build_deploy_args(&ctx, &resolved);

        let line = format!(
            "powershell.exe -NoProfile -File {}\\deploy_linux.ps1 {}",
            resolved.script_path,
            args.join(" ")
        );
        let expected = format!(
            "powershell.exe -NoProfile -File \\\\tfbd\\storage\\_Delo2020-Collection2020\\Delo2020\\dev-stand-26.2\\20230403.53\\BuildResult\\deploy_linux.ps1 -TargetServerName {} -InstanceDir /opt/delo -AppName {} -DBProvider PostgreSQL -DBServer localhost -DBAdmin postgres -DBOwner X262 -CreateDB Recreate -LoadInitialData demo_csv_template -WebServerName {} -RegHost -PortA 10001 -PortB 10002 -SmbServerAddress {} -TempFilesPath $env:TEMP\\deploy_stand",
            ctx.target_server_name,
            ctx.app_name,
            ctx.web_server_name,
            ctx.smb_server_address
        );
        assert_eq!(line, expected);
    }

    fn synthetic_config(reg_host: bool) -> CollectionsConfig {
        let configure_vm = ConfigureVm {
            dotnet: "dotnet-repo".to_string(),
            postgresql: "pg-repo".to_string(),
            nginx: "nginx-repo".to_string(),
            elasticsearch: None,
            kafka: None,
            chromium: None,
            samba: None,
            ca_trust: false,
            keycloak: None,
        };
        let deploy = DeployParams {
            db_provider: "PostgreSQL".to_string(),
            db_server: "localhost".to_string(),
            db_owner: "owner".to_string(),
            db_admin: "admin".to_string(),
            create_db: "Create".to_string(),
            load_initial_data: None,
            reg_host,
        };
        let mut stages = HashMap::new();
        stages.insert(
            "dev".to_string(),
            Stage {
                stand_prefix: "dev-stand".to_string(),
            },
        );
        let mut versions = HashMap::new();
        versions.insert(
            "1.0".to_string(),
            Version {
                configure_vm,
                deploy,
                stages,
            },
        );
        let mut projects = HashMap::new();
        projects.insert(
            "Proj".to_string(),
            Project {
                collection_path_suffix: "Suffix".to_string(),
                default_instance_dir: "/opt/demo".to_string(),
                default_target_account: "root".to_string(),
                versions,
            },
        );
        let mut collections = HashMap::new();
        collections.insert(
            "Coll".to_string(),
            Collection {
                path_prefix: "_Prefix".to_string(),
                tfvs_collection_uri: "https://tfs.example/".to_string(),
                tfvs_team_project: "TP".to_string(),
                stand_domain: "loc.test".to_string(),
                projects,
            },
        );
        CollectionsConfig {
            collections,
            defaults: Defaults {
                temp_files_path: "$env:TEMP\\deploy_test".to_string(),
            },
        }
    }
}

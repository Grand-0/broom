use std::process::Command;

use crate::services::ps_executor;

pub fn create_vm(name: &str) -> Result<String, String> {
    run_hyper_v_command("New-VM", name, &["-Name", name])
}

pub fn remove_vm(name: &str) -> Result<String, String> {
    run_hyper_v_command("Remove-VM", name, &["-Name", name, "-Force"])
}

pub fn get_vm_status(name: &str) -> Result<String, String> {
    run_hyper_v_command("Get-VM", name, &["-Name", name])
}

fn run_hyper_v_command(operation: &str, vm_name: &str, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("powershell.exe");
    command.arg("-NoProfile").arg("-Command").arg(args.join(" "));
    ps_executor::execute_technical(operation, vm_name, command)
}

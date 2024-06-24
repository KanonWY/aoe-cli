use crate::cmd::cli_base::ModuleSubCmd;

/// 处理 module 主函数
pub fn module_cmd(sub_cmd: &ModuleSubCmd) {
    match sub_cmd {
        ModuleSubCmd::Install { name, version } => module_install_cmd(name, version),
        ModuleSubCmd::Uninstall { name, version } => module_uninstall_cmd(name, version),
        ModuleSubCmd::Update { name, version } => module_update_cmd(name, version),
        ModuleSubCmd::Info { name } => module_info_cmd(name),
        ModuleSubCmd::List => module_list_cmd(),
    }
}

fn module_install_cmd(name: &str, version: &Option<String>) {
    println!("安装模块: {} 版本: {:?}", name, version);
}

fn module_uninstall_cmd(name: &str, version: &Option<String>) {
    println!("卸载模块: {} 版本: {:?}", name, version);
}

fn module_update_cmd(name: &str, version: &Option<String>) {
    println!("更新模块: {} 版本: {:?}", name, version);
}

// 打印软件包信息指令处理流程
fn module_info_cmd(name: &str) {
    println!("显示软件包 {} 信息", name);
}

// 打印软件包列表指令处理流程
fn module_list_cmd() {
    println!("打当前系统已安装的软件包列表");
}

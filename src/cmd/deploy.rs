use crate::cmd::cli_base::DeploySubCmd;

/// 处理 module 主函数
pub fn deploy_cmd(sub_cmd: &DeploySubCmd) {
    match sub_cmd {
        DeploySubCmd::Deps { name } => deploy_deps(name),
    }
}

fn deploy_deps(name: &str) {
    println!("依赖部署: {}", name);
}

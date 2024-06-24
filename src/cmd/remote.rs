use crate::cmd::cli_base::RemoteSubCmd;

/// 处理 module 主函数
pub fn remote_cmd(sub_cmd: &RemoteSubCmd) {
    match sub_cmd {
        RemoteSubCmd::List => remote_module_list_cmd(),
    }
}

fn remote_module_list_cmd() {
    println!("显示远端的文件！");
}

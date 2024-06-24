use clap::CommandFactory;
use clap::Parser;

use crate::cmd::cli_base::{Cli, Cmds};
use crate::cmd::{auto_script, deploy, module, remote};

impl Cli {
    pub fn new() -> Self {
        Cli::parse()
    }

    /// 处理所有的指令
    pub fn handle_all_command(&self) {
        match &self.cmd {
            Some(Cmds::Remote { remote_sub_cmd }) => remote::remote_cmd(remote_sub_cmd),
            Some(Cmds::Module { module_sub_cmd }) => module::module_cmd(module_sub_cmd),
            Some(Cmds::Deploy { deploy_sub_cmd }) => deploy::deploy_cmd(deploy_sub_cmd),
            Some(Cmds::AutoComplete {}) => auto_script::generate_completions(),
            None => Cli::command().print_help().unwrap(),
        }
    }
}

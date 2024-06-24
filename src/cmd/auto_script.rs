use crate::cmd::cli_base::Cli;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Zsh};
use std::env;
use std::path::PathBuf;

/// 生成补全脚本函数
pub fn generate_completions() {
    let mut app = Cli::command();
    let out_dir = env::var_os("OUT_DIR").unwrap_or_else(|| PathBuf::from(".").into());

    generate_to(Bash, &mut app, "aoe", &out_dir).expect("Failed to generate bash completions");
    generate_to(Zsh, &mut app, "aoe", &out_dir).expect("Failed to generate zsh completions");
}

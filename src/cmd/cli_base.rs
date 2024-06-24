use clap::{command, Parser};

#[derive(Parser)]
pub enum Cmds {
    #[command(about = "Operations related to the autonomous driving system module")]
    Module {
        #[command(subcommand)]
        module_sub_cmd: ModuleSubCmd,
    },

    #[command(about = "Autonomous deploy")]
    Deploy {
        #[command(subcommand)]
        deploy_sub_cmd: DeploySubCmd,
    },

    #[command(about = "remote package info")]
    Remote {
        #[command(subcommand)]
        remote_sub_cmd: RemoteSubCmd,
    },

    // 生成命令行补全脚本
    #[command(about = "生成自动补全脚本")]
    AutoComplete {},
}

#[derive(Parser)]
pub enum ModuleSubCmd {
    #[command(about = "Install specified package")]
    Install {
        #[arg(short, long, help = "Package name")]
        name: String,

        #[arg(short, long, help = "Package version")]
        version: Option<String>,
    },

    #[command(about = "Uninstall specified package")]
    Uninstall {
        #[arg(short, long, help = "Package name")]
        name: String,

        #[arg(short, long, help = "Package version")]
        version: Option<String>,
    },

    #[command(about = "Update specified package")]
    Update {
        #[arg(short, long, help = "Package name")]
        name: String,

        #[arg(short, long, help = "Package version")]
        version: Option<String>,
    },

    #[command(about = "Show specified package info")]
    Info {
        #[arg(short, long, help = "Package name")]
        name: String,
    },

    #[command(about = "List the packages that have been installed on the current system")]
    List,
}

#[derive(Parser)]
pub enum DeploySubCmd {
    #[command(about = "Autonomous machine basic deps deploy")]
    Deps {
        #[arg(short, long, value_parser(["ubuntu18", "ubuntu20"]), help = "machine system")]
        name: String,
    },
}

#[derive(Parser)]
pub enum RemoteSubCmd {
    #[command(about = "remote module inspect")]
    List,
}

#[derive(Parser)]
#[command(name = "aoe", author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Cmds>,
}

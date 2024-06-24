mod cmd;

use crate::cmd::cli_base::Cli;

fn main() {
    let c = Cli::new();
    c.handle_all_command();
}

use std::process::exit;

use crate::commands::AppCommand;
use colored::Colorize;
use utils::app::App;
use utils::VERSION;
pub mod commands;
#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let app = App::initialize();
    let cmd = AppCommand::current().unwrap_or(AppCommand::Help);

    if app.has_flag(&["--help", "-h"]) {
        println!("{}", cmd.help());
        return Ok(());
    }
    if app.has_flag(&["--version", "-v"]) {
        println!("zetac v{}", VERSION.bright_green().bold());
        exit(0);
    }
    cmd.run(app).await?;
    Ok(())
}

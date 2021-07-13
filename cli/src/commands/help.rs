use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;
use utils::{app::App, VERSION};

use super::Command;

/// Struct implementation for the `Help` command.
pub struct Help;

#[async_trait]
impl Command for Help {
    fn help() -> String {
        format!(
            r#"
Novus Package Manager {}

Usage: {} {} [<options>]

Commands:
  {} {} {} Installs packages.
  {} {} {} Uninstalls packages.
  {} {} {} Updates packages.
  {} {} {} Lists all packages.
  {} {} {} Searches for packages.  
  {} {} {} Provides information on a specific package.
  {} {} {} Clears all cache.
  {} {} {} Quits an application or a list of applications.
  {} {} {} Forcequits an application or a list of applications.
  
Run {} for more info about each command."#,
            VERSION.bright_green().bold(),
            "novus".bright_green(),
            "[command]".white(),
            "*".bright_magenta().bold(),
            "install".bright_cyan(),
            "(i)".yellow(),
            "*".bright_magenta().bold(),
            "uninstall".bright_cyan(),
            "(u)".yellow(),
            "*".bright_magenta().bold(),
            "update".bright_cyan(),
            "(upgrade)".yellow(),
            "*".bright_magenta().bold(),
            "list".bright_cyan(),
            "(show)".yellow(),
            "*".bright_magenta().bold(),
            "search".bright_cyan(),
            "(find)".yellow(),
            "*".bright_magenta().bold(),
            "info".bright_cyan(),
            "(details)".yellow(),
            "*".bright_magenta().bold(),
            "clean".bright_cyan(),
            "(clear)".yellow(),
            "*".bright_magenta().bold(),
            "quit".bright_cyan(),
            "(exit)".yellow(),
            "*".bright_magenta().bold(),
            "forcequit".bright_cyan(),
            "(forcequit)".yellow(),
            "novus [command] --help".bright_green(),
        )
    }

    async fn exec(_app: Arc<App>) -> Result<()> {
        println!("{}", Self::help());
        Ok(())
    }
}

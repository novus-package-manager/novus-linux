pub mod help;

use std::str::FromStr;
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use utils::app::App;

#[derive(Debug)]
pub enum AppCommand {
    Help,
}

impl FromStr for AppCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-?" => Ok(Self::Help),
            _ => Err(()),
        }
    }
}

impl AppCommand {
    pub fn current() -> Option<Self> {
        if std::env::args().len() == 1 {
            return Some(Self::Help);
        }

        match std::env::args().nth(1) {
            Some(cmd) => Self::from_str(cmd.as_str()).ok(),
            None => None,
        }
    }

    pub fn help(&self) -> String {
        match self {
            Self::Help => help::Help::help(),
        }
    }

    pub async fn run(&self, app: App) -> Result<()> {
        let app = Arc::new(app);
        match self {
            Self::Help => help::Help::exec(app).await,
        }
    }
}

#[async_trait]
pub trait Command {
    fn help() -> String;

    async fn exec(app: Arc<App>) -> Result<()>;
}

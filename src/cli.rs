use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    ///
    Launch {
        /// version to launch
        version: String,
        //version: Option<String>
    },

    Install {
        version: String,
    },

    Uninstall {
        version: String,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

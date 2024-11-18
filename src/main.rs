mod commands;

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use crate::commands::{update::UpdateCommand, check::CheckCommand};

#[derive(Parser)]
#[command(name = "neim")]
#[command(version = "0.0.1")]
#[command(author = "seppzer0")]
#[command(about = "neim — Defconfig modder for Kali NetHunter support.")]
struct Arguments {
    /// Subcommand to execute.
    #[command(subcommand)]
    command: Commands,

    /// Path to defconfig.
    #[arg(required = true)]
    defconfig: PathBuf,

    /// Flag for verbose output.
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Update existing defconfig.
    Update {
        /// Sets the mode of operation.
        #[arg(value_enum, default_value_t = Mode::New)]
        mode: Mode,
    },

    /// Check existing defconfig.
    Check,
}

#[derive(ValueEnum, Clone)]
enum Mode {
    Inline,
    New,
}

fn main() {
    let args: Arguments = Arguments::parse();
    let defconfig: PathBuf = args.defconfig;
    let verbose: bool = args.verbose;

    match args.command {
        Commands::Update { mode } => {
            let uc: UpdateCommand = UpdateCommand::new(defconfig, mode, verbose);
            uc.run()
        }
        Commands::Check => {
            let cc: CheckCommand = CheckCommand::new(defconfig, verbose);
            cc.run()
        }
    }
}

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "Orchard CLI",
    version = "0.1.0",
    about = "Command line interface for Orchard application",
    subcommand_required = true,
    arg_required_else_help = true
)]
pub struct OrchardCli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Trigger an action based on an event
    TriggerAction {
        #[arg(value_enum)]
        action: OrchardCommands,
    },
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab-case")]
pub enum OrchardCommands {
    /// secure generate a new key pair
    GenerateKeyPair,
}

impl std::fmt::Display for OrchardCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrchardCommands::GenerateKeyPair => write!(f, "generate-key-pair"),
        }
    }
}

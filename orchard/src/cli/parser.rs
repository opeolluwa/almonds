use crate::{
    cli::{
        actors::generate_key_pair,
        app::{Commands, OrchardCli, OrchardCommands},
    },
    errors::AppError,
};

pub async fn parse_commands(cli: OrchardCli) -> Result<(), AppError> {
    match cli.command {
        Commands::TriggerAction { action } => {
            handle_orchard_command(action).await?;
        }
    }

    Ok(())
}

async fn handle_orchard_command(action: OrchardCommands) -> Result<(), AppError> {
    match action {
        OrchardCommands::GenerateKeyPair => generate_key_pair().await?,
    };

    Ok(())
}

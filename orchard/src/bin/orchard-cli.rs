use clap::Parser;
use orchard_lib::cli::app::OrchardCli;
use orchard_lib::errors::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = OrchardCli::parse();
    Ok(())
}

use clap::Parser;
use orchard_lib::cli::app::OrchardCli;
use orchard_lib::errors::app_error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = OrchardCli::parse();
    Ok(())
}

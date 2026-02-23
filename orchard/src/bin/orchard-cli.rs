use clap::Parser;
use orchard_lib::errors::AppError;
use orchard_lib::cli::app::OrchardCli;

#[tokio::main]
async fn main() -> Result<(), AppError> {

     let cli = OrchardCli::parse();
    Ok(())
}

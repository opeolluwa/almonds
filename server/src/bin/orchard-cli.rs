use clap::{Parser, Subcommand};
use rand::{distr::Alphanumeric, RngExt};

#[derive(Parser)]
#[command(name = "orchard")]
#[command(about = "Orchard CLI utilities")]
struct OrchardCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate API public/private keys
    GenerateKeys,
}

fn generate_keys() {
    let mut rng = rand::rng();

    let public_key: String = (0..64).map(|_| rng.sample(Alphanumeric) as char).collect();

    let private_key: String = (0..64).map(|_| rng.sample(Alphanumeric) as char).collect();
    println!("Public Key : pk_{}", public_key);
    println!("Private Key: sk_{}", private_key);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = OrchardCli::parse();

    match cli.command {
        Commands::GenerateKeys => generate_keys(),
    }

    Ok(())
}

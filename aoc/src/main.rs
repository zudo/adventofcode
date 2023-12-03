use aoc::get;
use aoc::post;
use clap::Parser;
use clap::Subcommand;
use std::error::Error;
#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Get(get::Args),
    Post(post::Args),
}
impl Commands {
    async fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Get(args) => get::run(args).await,
            Commands::Post(args) => post::run(args).await,
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    Cli::parse().command.run().await
}

#[macro_use]
extern crate lazy_static;

use std::env;

use anyhow::Result;
use clap::{Parser, Subcommand};
use reqwest::Url;
use resources::objects;

mod create;

struct AppConfig {
    base_url: Url,
}

lazy_static! {
    static ref CONFIG: AppConfig = AppConfig {
        base_url: match env::var("API_SERVER_URL") {
            Ok(url) => Url::parse(url.as_str()).unwrap(),
            Err(_) => Url::parse("http://127.0.0.1:8080/api/v1/").unwrap(),
        }
    };
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a resource using configuration file.
    Create(create::Arg),
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create(arg) => arg.handle()?,
    }

    Ok(())
}

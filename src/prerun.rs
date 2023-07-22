use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use tokio::fs;

#[derive(Parser, Debug)]
#[command(long_about = None)]
pub struct Args {
    /// Activate debug mode `--debug true`
    #[arg(short = 'D', long, default_value_t = false)]
    pub debug: bool,
    /// Set categories path
    #[arg(short = 'C', long, default_value_t = String::from("category"))]
    pub categories_path: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArgsParsed {
    pub debug: bool,
    pub categories_path: PathBuf,
}

pub async fn run() -> Result<ArgsParsed> {
    let args = Args::parse();

    let categories_path = PathBuf::from(args.categories_path);

    if fs::read_dir(&categories_path).await.is_err() {
        fs::create_dir(&categories_path).await?;
    }

    let args_parsed = ArgsParsed {
        categories_path,
        debug: args.debug,
    };

    Ok(args_parsed)
}

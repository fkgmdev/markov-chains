use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "phonotax")]
#[command(about = "Language detection tool", version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Subcommand)]
pub enum Commands {
    Train {
        #[arg(short, long, default_value = "list.txt")]
        input: String,
        #[arg(short, long, default_value = "profiles.json")]
        output: String,
    },

    Detect {
        #[arg(short, long, conflicts_with = "file")]
        text: Option<String>,
        #[arg(short, long, conflicts_with = "text")]
        file: Option<PathBuf>,
        #[arg(short, long, default_value = "profiles.json")]
        database: String,
    },
}

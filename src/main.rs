use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{add_record, list_records};
use storage::Csv;

mod commands;
mod record;
mod storage;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List all the things you did
    List,
    /// Add something you did
    Add,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let base_dir = dir::home_dir().expect("Cant find home dir").join(".did");
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }

    let csv = Csv {
        path: &base_dir.join("record.csv"),
    };

    match args.command {
        Commands::List => {
            let records = list_records(csv)?;
            for record in records {
                println!("{:?}", record)
            }
        }
        Commands::Add => add_record(csv)?,
    };

    Ok(())
}

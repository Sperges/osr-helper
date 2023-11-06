use std::collections::HashMap;
use std::env::join_paths;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use anyhow::Result;

use crate::dice::Dice;
use crate::table::Tables;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Rolls dice
    Roll {
        /// Expression to roll
        expression: String,

        #[arg(short, long)]
        /// display the results of each die
        verbose: bool,

        #[arg(short, long)]
        /// total the result of the dice
        total: bool,
    },

    /// Rolls on a table
    Table {
        /// The path of the table to roll
        path: PathBuf,

        /// The table to roll on
        table: String,
    },

    /// Rolls on a hex flower, saving the state in an enironment variable
    Flower {
        /// The path of the hex flower
        path: PathBuf,

        #[arg(short, long)]
        /// environment variable override
        env: Option<String>,
    },
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Roll {
            expression,
            verbose,
            total,
        }) => {
            let roll = Dice::parse(&expression)?.roll();
            if *verbose {
                for sub_roll in roll.sub_rolls.iter() {
                    eprintln!("{}", sub_roll);
                }
            }
            if *total {
                println!("{}", roll.sum());
            } else {
                println!("{}", roll);
            }
        }
        Some(Commands::Table { path, table }) => {
            println!("{}", Tables::new(path)?.roll(table)?);
        }
        Some(Commands::Flower { path, env: _ }) => {
            const FLOWERS_FILE: &str = ".flowers";

            let flowers_path = {
                if let Some(parent) = path.parent() { 
					parent.join(PathBuf::from(FLOWERS_FILE))
                } else {
                    PathBuf::from(FLOWERS_FILE)
                }
            };

            let mut flowers: HashMap<String, String> = {
                if let Ok(ok) = fs::read_to_string(&flowers_path) {
                    ron::from_str(&ok)?
                } else {
                    HashMap::new()
                }
            };

            if let Some(stem) = path.file_stem().and_then(|os_str| os_str.to_str()) {
				if flowers.contains_key(stem) {
					let roll = Tables::new(path)?.roll(&flowers[stem])?;
					println!("{}", &roll);
					flowers.insert(stem.to_string(), roll);
				} else {
					let tables = Tables::new(path)?;
					if let Some(name) = tables.any_name() {
						let roll = tables.roll(name)?;
						println!("{}", &roll);
						flowers.insert(stem.to_string(), roll);
					} else {
						println!("Error: the flower file seems to be empty")
					}
				}
				fs::write(flowers_path, ron::ser::to_string::<HashMap<String, String>>(&flowers)?)?;
            } else {
                println!("Error: invalid file {:#?}", path);
            }
        }
        None => {}
    }

    Ok(())
}

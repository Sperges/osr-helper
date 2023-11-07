use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use anyhow::{anyhow, Result};

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
        table: Option<String>,
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
			if let Some(name) = table {
				println!("{}", Tables::new(path)?.roll(name)?);
			} else {
				let tables = Tables::new(path)?;

				if let Some(name) = tables.first_name() {
					println!("{}", tables.roll(name)?);
				} else {
					println!("That's odd, there don't seem to be any tables");
				}
			}
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
            let (stem, roll) =
                if let Some(stem) = path.file_stem().and_then(|os_str| os_str.to_str()) {
                    if flowers.contains_key(stem) {
                        let roll = Tables::new(path)?.roll(&flowers[stem])?;
                        (stem, roll)
                    } else {
                        let tables = Tables::new(path)?;
                        if let Some(name) = tables.any_name() {
                            let roll = tables.roll(name)?;
                            (stem, roll)
                        } else {
                            return Err(anyhow!("the flower file seems to be empty"));
                        }
                    }
                } else {
                    return Err(anyhow!("invalid file {:#?}", path));
                };
			print_flower(&roll);
            println!("{}", &roll);
            flowers.insert(stem.to_string(), roll);
            fs::write(
                flowers_path,
                ron::ser::to_string::<HashMap<String, String>>(&flowers)?,
            )?;
        }
        None => {}
    }
    Ok(())
}

fn print_flower(roll: &String) {
    if let Ok(id) = roll.chars().take(2).collect::<String>().parse::<usize>() {
        let mut ids = vec!["    "; 19];
        ids[id] = "HERE";
        println!(
            r"
                  ______
                 /      \
          ______/   00   \______
         /      \  {}  /      \
  ______/   03   \______/   01   \______
 /      \  {}  /      \  {}  /      \
/   07   \______/   04   \______/   02   \
\  {}  /      \  {}  /      \  {}  /
 \______/   08   \______/   05   \______/
 /      \  {}  /      \  {}  /      \
/   12   \______/   09   \______/   06   \
\  {}  /      \  {}  /      \  {}  /
 \______/   13   \______/   10   \______/
 /      \  {}  /      \  {}  /      \
/   16   \______/   14   \______/   11   \
\  {}  /      \  {}  /      \  {}  /
 \______/   17   \______/   15   \______/
        \  {}  /      \  {}  /
         \______/   18   \______/
                \  {}  /
                 \______/
",
            ids[0],
            ids[3],
            ids[1],
            ids[7],
            ids[4],
            ids[2],
            ids[8],
            ids[5],
            ids[12],
            ids[9],
            ids[6],
            ids[13],
            ids[10],
            ids[16],
            ids[14],
            ids[11],
            ids[17],
            ids[15],
            ids[18],
        );
    }
}

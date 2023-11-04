use std::{path::PathBuf, env};

use clap::{Parser, Subcommand};

use anyhow::{Result, anyhow};

use crate::dice::Dice;

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

		#[arg(short, long)]
		/// Custom roll to make on table
		roll: Option<String>,
	},

	/// Rolls on a hex flower, saving the state in an enironment variable
	Flower {
		/// The path of the hex flower
		path: PathBuf,

		#[arg(short, long)]
		/// environment variable override
		env: Option<String>,
	}
}

pub fn run() -> Result<()> {
	let cli = Cli::parse();

	match &cli.command {
		Some(Commands::Roll { expression, verbose, total }) => {

			let mut dice = Dice::parse(&expression)?;

			let roll = dice.roll();

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
		},
		Some(Commands::Table { path: _, table: _, roll: _ }) => {
			todo!();
			// crate::table::roll(path, table, roll)?;
		},
		Some(Commands::Flower { path, env: _ }) => {
			if let Some(env_name) = path.file_stem() {
				match env::var(env_name) {
					Ok(_) => todo!(),
					Err(_) => todo!(),
				}
			} else {
				return Err(anyhow!("{:?} is not a valid flower file.", path))
			}
		},
		None => {},
	}

	Ok(())
}
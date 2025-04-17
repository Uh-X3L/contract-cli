use clap::{Parser, Subcommand};
use contract_cli::db::run_migrations;
use contract_cli::{Contract, establish_connection, utils}; // from lib.rs
use env_logger; // added import

mod profiler;
mod utils;
mod utils::display;
mod config;

#[derive(Parser)]
#[command(name = "Contract CLI")]
#[command(about = "Interact with your contract", long_about = None)]
struct Cli {
    /// Contract owner (defaults to 'alice')
    #[arg(long, default_value = "alice")]
    owner: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show contract balance and owner
    Status,
    /// Deposit funds into the contract
    Deposit {
        #[arg(long)]
        amount: u64,
    },
    /// Withdraw funds from the contract
    Withdraw {
        #[arg(long)]
        amount: u64,
    },
    /// Show last 5 transactions
    History,

    /// Run a specific database migration
    Migrate {
        /// Name of the migration file to run (e.g., m_20240414_002_data_transform.rs)
        #[arg(long)]
        filename: String,
    },

    /// Profile a CSV file (schema, nulls, count)
    Profile {
        /// Path to CSV file
        #[arg(long, value_name = "FILE")]
        input: std::path::PathBuf,
        /// Delimiter (default: ,)
        #[arg(long, default_value = ",")]
        delimiter: char,
    },
}

fn main() -> anyhow::Result<()> {
    env_logger::init(); // Logging is now enabled
    let cli = Cli::parse();
    let conn = establish_connection()?; // new helper from db.rs

    let contract_id = utils::hash::hash_owner(&cli.owner);

    let mut contract = Contract::load_or_create(&conn, contract_id, &cli.owner)?;

    match cli.command {
        Commands::Status => contract.status(),
        Commands::Deposit { amount } => contract.deposit(&conn, amount)?,
        Commands::Withdraw { amount } => contract.withdraw(&conn, amount)?,
        Commands::History => contract.show_history(&conn)?,
        Commands::Migrate { filename } => {
            run_migrations(&conn, &filename)?;
        }
        Commands::Profile { input, delimiter } => {
            let (rows, prof) = profiler::profile_csv(&input, delimiter as u8)?;
            utils::display::print_profile(rows, prof);
        }
    }

    Ok(())
}

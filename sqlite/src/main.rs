use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::{create_table, read_table, update_record, delete_table, load_data};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a table
    #[command(alias = "c", short_flag = 'c')]
    Create { table_name: String },

    /// Read data from a table
    #[command(alias = "r", short_flag = 'r')]
    Read { table_name: String },

    /// Update a record in a table
    #[command(alias = "u", short_flag = 'u')]
    Update {
        table_name: String,
        column: String,
        new_value: String,
        condition: String,
    },

    /// Delete a table
    #[command(alias = "d", short_flag = 'd')]
    Delete { table_name: String },

    /// Load data from a CSV file into a table
    #[command(alias = "l", short_flag = 'l')]
    Load {
        table_name: String,
        file_path: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("my_database.db")?;

    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table {}", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Read { table_name } => {
            println!("Reading data from Table {}", table_name);
            read_table(&conn, &table_name).expect("Failed to read table data");
        }
        Commands::Update {
            table_name,
            column,
            new_value,
            condition,
        } => {
            println!(
                "Updating table '{}' setting '{}' to '{}' where {}",
                table_name, column, new_value, condition
            );
            update_record(&conn, &table_name, &column, &new_value, &condition)
                .expect("Failed to update record");
        }
        Commands::Delete { table_name } => {
            println!("Deleting Table {}", table_name);
            delete_table(&conn, &table_name).expect("Failed to delete table");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!("Loading data into table '{}' from '{}'", table_name, file_path);
            load_data(&conn, &table_name, &file_path).expect("Failed to load data");
        }
    }
    Ok(())
}


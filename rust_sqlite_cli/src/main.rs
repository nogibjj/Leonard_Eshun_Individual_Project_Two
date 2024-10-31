use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use rust_sqlite_cli::my_lib::crud::{
    delete_data, read_all_data, read_data, save_data, update_data,
};
use rust_sqlite_cli::my_lib::extract::extract;
use rust_sqlite_cli::my_lib::transform::transform_n_load;
use rust_sqlite_cli::my_lib::util::{
    create_borrowed_view_string, create_borrowed_view_usize, create_borrowed_view_vector,
    parse_json_to_map_string, parse_json_to_map_usize, parse_json_to_map_vector,
    parse_json_to_tuple_vec, parse_to_vec,
};

//Here we define a struct (or object) to hold our CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

//This is designed to generate an object out of the CLI inputs
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

//An enum is a type in rust that can have multiple exauhstive and mutually exclusive options
#[derive(Debug, Subcommand)]

enum Commands {
    ///Pass a record to save
    #[command(alias = "s", short_flag = 's')]
    Save {
        database_name: String,
        table_name: String,
        row: String,
    },
    ///Pass a query string to execute Read one record
    #[command(alias = "q", short_flag = 'q')]
    Read_One {
        database_name: String,
        table_name: String,
        data_id: i64,
    },
    #[command(alias = "a", short_flag = 'a')]
    Read_All {
        database_name: String,
        table_name: String,
    },
    ///Pass a record to drop
    #[command(alias = "d", short_flag = 'd')]
    Delete {
        database_name: String,
        table_name: String,
        data_id: i64,
    },
    ///Pass a record to update
    #[command(alias = "u", short_flag = 'u')]
    Update {
        database_name: String,
        table_name: String,
        things_to_update: String,
        data_id: i64,
    },
    ///Pass information to transform and load data
    #[command(alias = "l", short_flag = 'l')]
    Load {
        local_dataset: String,
        database_name: String,
        new_data_tables: String,
        new_lookup_tables: String,
        column_attributes: String,
        column_map: String,
    },
    ///Pass information to extract data
    #[command(alias = "e", short_flag = 'e')]
    Extract { url: String, file_name: String },
}

fn main() -> Result<()> {
    //Here we parse the CLI arguments and store them in the args object
    let args = Cli::parse();

    match args.command {
        Commands::Save {
            database_name,
            table_name,
            row,
        } => {
            println!("Saving record {}", row);
            let result = save_data(&database_name, &table_name, &&parse_to_vec(&row));

            match result {
                Ok(message) => println!("{}", message),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Read_One {
            database_name,
            table_name,
            data_id,
        } => {
            println!("Reading one record with id: {}", data_id);
            let result = read_data(&database_name, &table_name, data_id);

            match result {
                Ok(vec) => println!("{:?}", vec), // Use `{:?}` to print `Vec<String>` in debug format
                Err(e) => eprintln!("Error: {}", e),
                // None => println!("No data available"),    // Handle `None` case
            }
        }
        Commands::Read_All {
            database_name,
            table_name,
        } => {
            println!("Reading all records from: {}", table_name);
            let result = read_all_data(&database_name, &table_name);

            match result {
                Ok(vec) => println!("{:?}", vec), // Use `{:?}` to print `Vec<String>` in debug format
                Err(e) => eprintln!("Error: {}", e),
                // None => println!("No data available"),    // Handle `None` case
            }
        }
        Commands::Delete {
            database_name,
            table_name,
            data_id,
        } => {
            println!("Deleting record with id: {}", data_id);
            let result = delete_data(&database_name, &table_name, data_id);

            match result {
                Ok(message) => println!("{}", message),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Update {
            database_name,
            table_name,
            things_to_update,
            data_id,
        } => {
            println!("Updating: {}", things_to_update);
            let result = update_data(
                &database_name,
                &table_name,
                &&parse_json_to_tuple_vec(&things_to_update),
                data_id,
            );

            match result {
                Ok(message) => println!("{}", message),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Load {
            local_dataset,
            database_name,
            new_data_tables,
            new_lookup_tables,
            column_attributes,
            column_map,
        } => {
            println!(
                "Loading data into sqlite database '{}' from '{}'",
                database_name, local_dataset
            );
            let result = transform_n_load(
                &local_dataset,
                &database_name,
                &create_borrowed_view_vector(&parse_json_to_map_vector(&new_data_tables)),
                &create_borrowed_view_vector(&parse_json_to_map_vector(&new_lookup_tables)),
                &create_borrowed_view_string(&parse_json_to_map_string(&column_attributes)),
                &create_borrowed_view_usize(&parse_json_to_map_usize(&column_map)),
            );

            match result {
                Ok(message) => println!("{}", message),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Extract { url, file_name } => {
            println!("Downloading data from url to '{}'", file_name);
            match extract(&url, &file_name) {
                Ok(message) => println!("{}", message),
                Err(e) => eprintln!("Failed to extract data: {}", e),
            }
        }
    }
    Ok(())
}

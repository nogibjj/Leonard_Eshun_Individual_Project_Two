use csv::ReaderBuilder;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::path::Path;
use super::util::log_tests;

pub fn create_table(
    conn: &Connection,
    table: &str,
    columns: &Vec<&str>,
    column_attributes: &HashMap<&str, &str>,
) -> Result<()> {
    conn.execute(&format!("DROP TABLE IF EXISTS {}", table), [])?;

    let col_attrib_list: Vec<String> = columns
        .iter()
        .map(|&col| format!("{} {}", col, column_attributes[col]))
        .collect();

    let create_table_sql = format!("CREATE TABLE {} ({})", table, col_attrib_list.join(", "));
    conn.execute(&create_table_sql, [])?;

    Ok(())
}


pub fn transform_n_load(
    local_dataset: &str,
    database_name: &str,
    new_data_tables: &HashMap<&str, Vec<&str>>,
    new_lookup_tables: &HashMap<&str, Vec<&str>>,
    column_attributes: &HashMap<&str, &str>,
    column_map: &HashMap<&str, usize>,
) -> Result<String> {
    // Load CSV file
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(Path::new(local_dataset))
        .expect("Failed to open CSV file");

    // Connect to the SQLite database
    let conn = Connection::open(database_name)?;
    println!("{:?}",database_name);
    // Create tables
    for (table_name, columns) in new_data_tables.iter() {
        println!("Creating non-lookup table: {}", table_name);
        let _ = log_tests(&format!("Creating non-lookup table: {}", table_name), false, false, false, false);
        create_table(&conn, table_name, columns, column_attributes)?;
    }

    for (table_name, columns) in new_lookup_tables.iter() {
        println!("Creating lookup table: {}", table_name);
        let _ = log_tests(&format!("Creating lookup table: {}", table_name), false, false, false, false);
        create_table(&conn, table_name, columns, column_attributes)?;
    }
    println!("Tables created.");
    let _ = log_tests("Tables created.", false, false, false, false);


    // Skip the first row (header)
    println!("Skipping the first row...");
    let _ = log_tests("Tables created.", false, false, false, false);

    let mut rows = reader.records();
    rows.next(); // Skipping header row

    println!("Inserting table data...");
    let _ = log_tests("Tables created.", false, false, false, false);

    for result in rows {
        // let row = result;
        let mut skip_data = false;
        // Handle the result from `rows.next()`
        let row = match result {
            Ok(record) => record, // Unwrap the `StringRecord` if successful
            Err(e) => {
                eprintln!("Error reading row: {}", e); // Print error and continue
                continue; // Skip this row on error
            }
        };
        // Insert lookup tables
        for (table_name, columns) in new_lookup_tables.iter() {
            // Now safely index into `row`, which is a `StringRecord`
            if !row[column_map[columns[0]]].parse::<i32>().is_ok() {
                skip_data = true;
                break;
            }

            let check_sql = format!(
                "SELECT COUNT({}) FROM {} WHERE {} = ?1",
                columns[0], table_name, columns[0]
            );

            let id_value: i32 = row[column_map[columns[0]]].parse().unwrap();
            let count: i32 = conn.query_row(&check_sql, &[&id_value], |row| row.get(0))?;

            if count == 0 {
                let data_values: Vec<String> = columns
                    .iter()
                    .map(|&col| format!("{}", row.get(column_map[col]).unwrap_or(""))) // Access safely using .get
                    .collect();
                let insert_sql = format!(
                    "INSERT INTO {} ({}) VALUES ('{}')",
                    table_name,
                    columns.join(", "),
                    data_values.join("', '")
                );
                conn.execute(&insert_sql, [])?;
            }
        }

        // Insert main data tables only if lookup info is valid
        if !skip_data {
            for (table_name, columns) in new_data_tables.iter() {
                let data_values: Vec<String> = columns
                    .iter()
                    .map(|&col| format!("{}", row.get(column_map[col]).unwrap_or(""))) // Access safely using .get
                    .collect();
                let insert_sql = format!(
                    "INSERT INTO {} ({}) VALUES ('{}')",
                    table_name,
                    columns.join(", "),
                    data_values.join("', '")
                );
                conn.execute(&insert_sql, [])?;
            }
        }
    }

    println!("Inserting table data completed");
    let _ = log_tests("Inserting table data completed", false, false, false, false);

    Ok("Transform and load Successful".to_string())
}





#[cfg(test)]
mod tests{
    // use super::super::util::log_tests;
    // use super::transform_n_load;
    // use std::collections::HashMap;
    // use maplit::hashmap;
}
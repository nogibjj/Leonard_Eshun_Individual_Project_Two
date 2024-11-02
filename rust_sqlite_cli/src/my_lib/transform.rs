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

    // Create tables
    for (table_name, columns) in new_data_tables.iter() {
        println!("Creating non-lookup table: {}", table_name);
        log_tests(&format!("Creating non-lookup table: {}", table_name), false, false, false, false);
        create_table(&conn, table_name, columns, column_attributes)?;
    }

    for (table_name, columns) in new_lookup_tables.iter() {
        println!("Creating lookup table: {}", table_name);
        log_tests(&format!("Creating lookup table: {}", table_name), false, false, false, false);
        create_table(&conn, table_name, columns, column_attributes)?;
    }
    println!("Tables created.");
    log_tests("Tables created.", false, false, false, false);


    // Skip the first row (header)
    println!("Skipping the first row...");
    log_tests("Tables created.", false, false, false, false);

    let mut rows = reader.records();
    rows.next(); // Skipping header row

    println!("Inserting table data...");
    log_tests("Tables created.", false, false, false, false);

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
    log_tests("Inserting table data completed", false, false, false, false);

    Ok("Transform and load Successful".to_string())
}

pub fn main() -> Result<()> {
    let local_dataset = "data.csv";
    let database_name = "example.db";

    // Example mappings and configurations for columns and tables
    let new_data_tables: HashMap<&str, Vec<&str>> = HashMap::new(); // Populate as needed
    let new_lookup_tables: HashMap<&str, Vec<&str>> = HashMap::new(); // Populate as needed
    let column_attributes: HashMap<&str, &str> = HashMap::new(); // Define column attributes
    let column_map: HashMap<&str, usize> = HashMap::new(); // Define column indices

    let result = transform_n_load(
        local_dataset,
        database_name,
        &new_data_tables,
        &new_lookup_tables,
        &column_attributes,
        &column_map,
    );

    match result {
        Ok(message) => println!("{}", message),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}





#[cfg(test)]
mod tests{
    use super::super::util::log_tests;
    use clap::error::Result;
    use super::transform_n_load;
    use std::collections::HashMap;
    use maplit::hashmap;



#[test] 
    fn test_transform_and_load() -> Result<(), Box<dyn std::error::Error>> {
        log_tests("Transform and Load Test", true, false, false, false);

        let table_map: HashMap<&str, Vec<&str>> = hashmap! {
            "air_quality" => vec![
                "air_quality_id",
                "fn_indicator_id",
                "fn_geo_id",
                "time_period",
                "start_date",
                "data_value"
            ]
        };
        
        let column_map: HashMap<&str, usize> = hashmap! {
            "air_quality_id" => 0,
            "indicator_id" => 1,
            "indicator_name" => 2,
            "measure" => 3,
            "measure_info" => 4,
            "geo_type_name" => 5,
            "geo_id" => 6,
            "geo_place_name" => 7,
            "time_period" => 8,
            "start_date" => 9,
            "data_value" => 10,
            "fn_geo_id" => 6,
            "fn_indicator_id" => 1,
        };
    
        let lookup_map: HashMap<&str, Vec<&str>> = hashmap! {
            "indicator" => vec![
                "indicator_id",
                "indicator_name",
                "measure",
                "measure_info"
            ],
            "geo_data" => vec![
                "geo_id",
                "geo_place_name",
                "geo_type_name"
            ]
        };
    
        let column_types: HashMap<&str, &str> = hashmap! {
            "air_quality_id" => "INTEGER PRIMARY KEY",
            "indicator_id" => "INTEGER PRIMARY KEY",
            "indicator_name" => "TEXT",
            "measure" => "TEXT",
            "measure_info" => "TEXT",
            "geo_type_name" => "TEXT",
            "geo_id" => "INTEGER PRIMARY KEY",
            "geo_place_name" => "TEXT",
            "time_period" => "TEXT",
            "start_date" => "TEXT",
            "data_value" => "REAL",
            "fn_indicator_id" => "INTEGER",
            "fn_geo_id" => "INTEGER"
        };
    
        // Assuming `column_map` is defined somewhere with the correct type
        // let column_map = hashmap! { /* your column map here */ };
    
        transform_n_load(
            "air_quality.csv",
            "air_quality.db",
            &table_map,
            &lookup_map,
            &column_types,
            &column_map.clone(),
        )?;
        log_tests("Transform and Load Test Successful", true, false, false,  false);

        Ok(())
    }
    
    
    // Stub for transform_n_load function
}
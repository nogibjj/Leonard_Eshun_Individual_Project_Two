use std::fs;
use rust_sqlite_cli::my_lib::util::log_tests;
use rust_sqlite_cli::my_lib::extract::extract;
use rust_sqlite_cli::my_lib::transform::transform_n_load;
use std::collections::HashMap;
use maplit::hashmap;
use rust_sqlite_cli::my_lib::crud::{delete_data, save_data, read_all_data, read_data, update_data, get_table_columns};




// Test Extract
#[test]
fn test_1_extract() -> clap::error::Result<()> {
    let _ = log_tests("Extraction Test", false, true,  false, true);
    let _ = log_tests("Removing existing CSV file if it exists", false, false,  false, false);
    //delete the file if it's there
    let file_path = "air_quality.csv";
    if fs::metadata(file_path).is_ok() {
        fs::remove_file(file_path)?;
    }

    let _ = log_tests("Confirming that CSV file doesn't exist...", false, false,  false, false);
    assert!(!fs::metadata(file_path).is_ok());
    let _ = log_tests("Test Successful", false, false,  false, false);

    let _ = log_tests("Extracting data and saving...", false, false,  false, false);
    let _ = extract("https://data.cityofnewyork.us/resource/c3uy-2p5r.csv?$limit=200000", file_path);

    let _ = log_tests("Testing if CSV file exists...", false, false,  false, false);
    assert!(fs::metadata(file_path).is_ok());
    let _ = log_tests("Extraction Test Successful", false, false, true,  false);

    Ok(())
}



// Transform load tests
#[test] 
fn test_2_transform_and_load() -> Result<(), Box<dyn std::error::Error>> {
    let _ = log_tests("Transform and Load Test", false, true, false, false);

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

    let _ = log_tests("Removing existing sqlite file if it exists", false, false,  false, false);
    //delete the file if it's there
    if fs::metadata("air_quality.db").is_ok() {
        fs::remove_file("air_quality.db")?;
    }
    let _ = log_tests("Confirming that sqlite file doesn't exist...", false, false,  false, false);

    assert!(!fs::metadata("air_quality.db").is_ok());

    transform_n_load(
        "air_quality.csv",
        "air_quality.db",
        &table_map,
        &lookup_map,
        &column_types,
        &column_map.clone(),
    )?;

    let _ = log_tests("Testing if sqlite file exists...", false, false,  false, false);
    assert!(fs::metadata("air_quality.db").is_ok());

    let _ = log_tests("Transform and Load Test Successful", false, false, true,  false);

    Ok(())
}







// CRUD TESTS
// Test read data
#[test]
fn test_3_read_data() -> clap::error::Result<()> {
    let _ = log_tests("One Record Reading Test", false, true, false, false);
    
    let row = read_data("air_quality.db", "air_quality", 740885);
    let _ = log_tests("Asserting that row[0][data_value] == 16.4", false, false, false,  false);
    match row {
        Ok(vec) => {
            // Now you can index into `vec` as it is a `Vec<String>`
            assert_eq!(vec[0][5], "16.4");
        }
        Err(e) => {
            // Handle the error case
            eprintln!("Error occurred: {:?}", e);
            assert_ne!(1, 1);
        }
    }
    let _ = log_tests("Assert Successful", false, false, false,  false);
    
    let _ = log_tests("One Record Reading Test Successful", false, false, true, false);
    
    Ok(())
}


// Test read all data
#[test]
fn test_4_read_all_data() -> rusqlite::Result<()> {
    let _ = log_tests("All Records Reading Test", false, true, false, false);

    let rows = read_all_data("air_quality.db", "air_quality")?;
    let _ = log_tests("Asserting that len(rows) == 18016", false, false, false,  false);
    assert_eq!(rows.len(), 18016);
    let _ = log_tests("All Records Reading Test Successful", false, false, true, false);

    Ok(())
}

// Test save data
#[test]
fn test_5_save_data() -> rusqlite::Result<()> {
    let _ = log_tests("Record Saving Test", false, true, false, false);

    let _ = log_tests("Asserting there's no record in geo_data with ID 100000", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;
    assert!(result.len() == 0);
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Saving new record with ID 100000", false, false, false,  false);
    save_data("air_quality.db", "geo_data", &["100000".to_string(), "Lancaster".to_string(), "UFO".to_string()])?;

    let _ = log_tests("Asserting there's now a record in geo_data with ID 100000", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;

    assert_eq!(result.len(), 1);
    let _ = log_tests("Assert Successful", false, false, false, false);

    let _ = log_tests("Record Saving Test Successful", false, false, true, false);

    Ok(())
}

// Test update data
#[test]
fn test_6_update_data() -> rusqlite::Result<()> {
    let _ = log_tests("Record Update Test", false, true, false, false);

    let _ = log_tests("Asserting 'geo_place_name' in geo_data for row ID 100000 is 'Lancaster'", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;
    assert_eq!(result[0][1], "Lancaster");
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Updating 'geo_place_name' in geo_data for row ID 100000 to 'Duke'", false, false, false,  false);
    let to_update: Vec<(String, String)> = vec![
        ("geo_place_name".to_string(), "Duke".to_string()),
    ];
    update_data("air_quality.db", "geo_data", &to_update, 100000)?;

    let _ = log_tests("Asserting 'geo_place_name' in geo_data for row ID 100000 is now 'Duke'", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;
    assert_eq!(result[0][1], "Duke");
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Record Update Test Successful", false, false, true, false);

    Ok(())
}

// Test delete data
#[test]
fn test_7_delete_data() -> rusqlite::Result<()> {
    let _ = log_tests("Record Deletion Test", false, true, false, false);

    let _ = log_tests("Asserting there's a record in geo_data for row ID 100000", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;
    assert_eq!(result.len(), 1);
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Deleting record with ID 100000", false, false, false,  false);
    delete_data("air_quality.db", "geo_data", 100000)?;

    let _ = log_tests("Asserting there's no record in geo_data with ID 100000", false, false, false,  false);
    let result = read_data("air_quality.db", "geo_data", 100000)?;
    assert!(result.len() == 0);
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Record Deletion Test Successful", false, false, true, false);

    Ok(())
}

// Test read all column names
#[test]
fn test_8_get_table_columns() -> rusqlite::Result<()> {
    let _ = log_tests("Reading All Column Test", false, true, false, false);

    let columns = get_table_columns("air_quality.db", "air_quality")?;
    let _ = log_tests("Asserting the air_quality table has six (6) columns", false, false, false,  false);
    assert_eq!(columns.len(), 6);
    let _ = log_tests("Assert Successful", false, false, false,  false);

    let _ = log_tests("Reading All Column Test Successful", false, false, true, false);

    Ok(())
}   


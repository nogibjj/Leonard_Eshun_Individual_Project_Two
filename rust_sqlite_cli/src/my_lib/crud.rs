use super::util::log_tests;
use rusqlite::{params, Connection, Result};

pub fn get_table_columns(database_name: &str, table_name: &str) -> Result<Vec<String>> {
    let conn = Connection::open(database_name)?;
    let mut stmt = conn.prepare(&format!("PRAGMA table_info('{}')", table_name))?;
    log_tests("Executing query...", false, false, false, false);

    let columns: Result<Vec<String>> = stmt.query_map([], |row| row.get::<_, String>(1))?.collect();

    Ok(columns.unwrap_or_default())
}

fn get_primary_key(conn: &Connection, table_name: &str) -> Result<String> {
    let mut stmt = conn.prepare(&format!(
        "SELECT name FROM pragma_table_info('{}') WHERE pk > 0",
        table_name
    ))?;

    stmt.query_row([], |row| row.get(0))
}

pub fn read_data(
    database_name: &str,
    table_name: &str,
    data_id: i64,
) -> Result<Option<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    let primary_key = get_primary_key(&conn, table_name)?;

    let query = format!("SELECT * FROM {} WHERE {} = ?", table_name, primary_key);
    log_tests("Executing query...", false, false, false, false);

    let mut stmt = conn.prepare(&query)?;
    let result: Result<Vec<String>> = stmt.query_map([data_id], |row| row.get(0))?.collect();

    Ok(result.ok())
}

pub fn read_all_data(database_name: &str, table_name: &str) -> Result<Option<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    let query = format!("SELECT * FROM {}", table_name);
    log_tests("Executing query...", false, false, false, false);

    let mut stmt = conn.prepare(&query)?;
    let result: Result<Vec<String>> = stmt.query_map([], |row| row.get(0))?.collect();

    Ok(result.ok())
}

pub fn save_data(database_name: &str, table_name: &str, row: &[String]) -> Result<String> {
    let conn = Connection::open(database_name)?;
    let columns = get_table_columns(database_name, table_name)?.join(", ");
    let values = row
        .iter()
        .map(|val| format!("'{}'", val))
        .collect::<Vec<_>>()
        .join(", ");
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name, columns, values
    );
    log_tests("Executing query...", false, false, false, false);

    conn.execute(&query, [])?;
    Ok("Save Successful".to_string())
}

pub fn delete_data(database_name: &str, table_name: &str, data_id: i64) -> Result<String> {
    let conn = Connection::open(database_name)?;
    let primary_key = get_primary_key(&conn, table_name)?;
    let query = format!("DELETE FROM {} WHERE {} = ?", table_name, primary_key);
    log_tests("Executing query...", false, false, false, false);

    conn.execute(&query, params![data_id])?;
    Ok("Delete Successful".to_string())
}

pub fn update_data(
    database_name: &str,
    table_name: &str,
    things_to_update: &[(String, String)],
    data_id: i64,
) -> Result<String> {
    let conn = Connection::open(database_name)?;
    let set_values = things_to_update
        .iter()
        .map(|(k, v)| format!("{}='{}'", k, v))
        .collect::<Vec<_>>()
        .join(", ");
    let primary_key = get_primary_key(&conn, table_name)?;
    let query = format!(
        "UPDATE {} SET {} WHERE {} = ?",
        table_name, set_values, primary_key
    );
    log_tests("Executing query...", false, false, false, false);

    conn.execute(&query, params![data_id])?;
    Ok("Update Successful".to_string())
}



#[cfg(test)]
mod tests{

    use super::*;
    use rusqlite::{Connection, Result};
    use super::super::util::log_tests;

    // use rusqlite::{params, Connection, Result};


    // #[test]


    // // Test read data
    // #[test]
    // fn test_read_data() -> Result<()> {
    //     log_tests("One Record Reading Test", true, false);
        
    //     let row = read_data("air_quality.db", "air_quality", 740885)?;
    //     log_tests("Asserting that row[0][data_value] == 16.4");
    //     assert_eq!(row[0]["data_value"], 16.4);
    //     log_tests("Assert Successful");
        
    //     log_tests("One Record Reading Test Successful", true);
        
    //     Ok(())
    // }

    // // Test read all data
    // fn test_read_all_data() -> Result<()> {
    //     log_tests("All Records Reading Test", true, false);

    //     let rows = read_all_data("air_quality.db", "air_quality")?;
    //     log_tests("Asserting that len(rows) == 18016");
    //     assert_eq!(rows.len(), 18016);
    //     log_tests("All Records Reading Test Successful", true);

    //     Ok(())
    // }

    // // Test save data
    // #[test]
    // fn test_save_data() -> Result<()> {
    //     log_tests("Record Saving Test", true, false);

    //     log_tests("Asserting there's no record in geo_data with ID 100000");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert!(result.is_none());
    //     log_tests("Assert Successful");

    //     log_tests("Saving new record with ID 100000");
    //     save_data("air_quality.db", "geo_data", &["100000", "Lancaster", "UFO"])?;

    //     log_tests("Asserting there's now a record in geo_data with ID 100000");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert_eq!(result.unwrap().len(), 1);
    //     log_tests("Assert Successful");

    //     log_tests("Record Saving Test Successful", true);

    //     Ok(())
    // }

    // // Test update data
    // #[test]
    // fn test_update_data() -> Result<()> {
    //     log_tests("Record Update Test", true, false);

    //     log_tests("Asserting 'geo_place_name' in geo_data for row ID 100000 is 'Lancaster'");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert_eq!(result.unwrap()[0]["geo_place_name"], "Lancaster");
    //     log_tests("Assert Successful");

    //     log_tests("Updating 'geo_place_name' in geo_data for row ID 100000 to 'Duke'");
    //     update_data("air_quality.db", "geo_data", hashmap!{"geo_place_name" => "Duke"}, 100000)?;

    //     log_tests("Asserting 'geo_place_name' in geo_data for row ID 100000 is now 'Duke'");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert_eq!(result.unwrap()[0]["geo_place_name"], "Duke");
    //     log_tests("Assert Successful");

    //     log_tests("Record Update Test Successful", true);

    //     Ok(())
    // }

    // // Test delete data
    // #[test]
    // fn test_delete_data() -> Result<()> {
    //     log_tests("Record Deletion Test", true, false);

    //     log_tests("Asserting there's a record in geo_data for row ID 100000");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert!(result.is_some());
    //     log_tests("Assert Successful");

    //     log_tests("Deleting record with ID 100000");
    //     delete_data("air_quality.db", "geo_data", 100000)?;

    //     log_tests("Asserting there's no record in geo_data with ID 100000");
    //     let result = read_data("air_quality.db", "geo_data", 100000)?;
    //     assert!(result.is_none());
    //     log_tests("Assert Successful");

    //     log_tests("Record Deletion Test Successful", true);

    //     Ok(())
    // }

    // // Test read all column names
    // #[test]
    // fn test_get_table_columns() -> Result<()> {
    //     log_tests("Reading All Column Test", true, false);

    //     let columns = get_table_columns("air_quality.db", "air_quality")?;
    //     log_tests("Asserting the air_quality table has six (6) columns");
    //     assert_eq!(columns.len(), 6);
    //     log_tests("Assert Successful");

    //     log_tests("Reading All Column Test Successful", true);

    //     Ok(())
    // }   

}
use super::util::log_tests;
use rusqlite::{params, types::Value, Connection, Result, Row};

pub fn get_table_columns(database_name: &str, table_name: &str) -> Result<Vec<String>> {
    let conn = Connection::open(database_name)?;
    let mut stmt = conn.prepare(&format!("PRAGMA table_info('{}')", table_name))?;
    let _ = log_tests("Executing query...", false, false, false, false);

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


// Helper function to convert each column in a row to a String
fn row_to_string_vec(row: &Row, column_count: usize) -> Result<Vec<String>> {
    let mut values = Vec::new();
    for i in 0..column_count {
        let value: Value = row.get(i)?; // Dynamically get the column as a `Value`
        let value_str = match value {
            Value::Integer(i) => i.to_string(),
            Value::Real(f) => f.to_string(),
            Value::Text(s) => s,
            Value::Blob(b) => format!("{:?}", b), // Convert blob to a formatted string
            Value::Null => "NULL".to_string(),     // Handle NULL values
        };
        values.push(value_str);
    }
    Ok(values)
}


pub fn read_data(
    database_name: &str,
    table_name: &str,
    data_id: i64,
) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    let primary_key = get_primary_key(&conn, table_name)?;
    let query = format!("SELECT * FROM {} WHERE {} = ?", table_name, primary_key);
    let _ = log_tests("Executing query...", false, false, false, false);
    
    let mut stmt = conn.prepare(&query)?;
    let column_count = stmt.column_count(); // Get column count from the statement

    let result = stmt.query_map([data_id], |row| {
        // Convert each column in the row to a String, regardless of type
        row_to_string_vec(row, column_count)
    })?.collect::<Result<Vec<Vec<String>>>>()?;

    Ok(result)
}


pub fn read_all_data(
    database_name: &str,
    table_name: &str,
) -> Result<Vec<Vec<String>>> {
    let conn = Connection::open(database_name)?;
    let query = format!("SELECT * FROM {}", table_name);
    let mut stmt = conn.prepare(&query)?;
    let column_count = stmt.column_count(); // Get column count from the statement

    let result = stmt.query_map([], |row| {
        // Use row_to_string_vec to convert each row into Vec<String>
        row_to_string_vec(row, column_count)
    })?.collect::<Result<Vec<Vec<String>>>>()?;

    Ok(result)
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
    let _ = log_tests("Executing query...", false, false, false, false);

    conn.execute(&query, [])?;
    Ok("Save Successful".to_string())
}

pub fn delete_data(database_name: &str, table_name: &str, data_id: i64) -> Result<String> {
    let conn = Connection::open(database_name)?;
    let primary_key = get_primary_key(&conn, table_name)?;
    let query = format!("DELETE FROM {} WHERE {} = ?", table_name, primary_key);
    let _ = log_tests("Executing query...", false, false, false, false);

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
    let _ = log_tests("Executing query...", false, false, false, false);

    conn.execute(&query, params![data_id])?;
    Ok("Update Successful".to_string())
}



#[cfg(test)]
mod tests{
    // use super::super::util::log_tests;
    // use super::*;
    // use maplit::hashmap;
}
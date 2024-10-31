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

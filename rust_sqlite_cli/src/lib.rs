use rusqlite::{params, Connection, Result};

pub mod my_lib;

//Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        // Assuming the `users` table has an `id` and `name` column
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let age: i32 = row.get(2)?;
        Ok((id, name, age))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (id, name, age) = row?;
        println!("ID: {}, Name: {}, Age: {}", id, name, age);
    }

    Ok(())
}

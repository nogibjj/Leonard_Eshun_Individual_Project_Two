use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Local;

pub fn get_server_time() -> String {
    let now = Local::now();
    format!("{}", now.format("%Y-%m-%d %H:%M:%S")).to_string()
} 

// Function to get the base directory
fn base_dir() -> PathBuf {
    // Get the current directory as the base
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let base_dir = current_dir.join("src"); // Equivalent to __file__, adjust as needed
    base_dir
}

// Function to construct the database path (similar to db_path in Python)
pub fn db_path(file_name: &str) -> PathBuf {
    let data_dir = base_dir().join("../data/");
    data_dir.join(file_name)
}


fn general_log(file_name: &str, log: &str,
    issql: bool,
    header: bool,
    last_in_group: bool,
    new_log_file: bool,
) -> std::io::Result<()> {
    // Remove any leading or trailing whitespace
    let log = log.trim();

    // Open the file in write ("w") or append ("a") mode based on the new_log_file flag
    let file_path = file_name;
    let file = if new_log_file || !Path::new(file_path).exists() {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
    } else {
        OpenOptions::new().append(true).open(file_path)
    };

    // Write to the file based on the flags
    if issql {
        writeln!(file?, "\n```sql\n{}\n```\n", log)?;
    } else if header {
        writeln!(file?, "### {} ### \n", log)?;
    } else if last_in_group {
        writeln!(file?, "{}\n\n", log)?;
    } else {
        writeln!(file?, "{} <br />", log)?;
    }

    Ok(())
}


pub fn log_tests(
    log: &str,
    issql: bool,
    header: bool,
    last_in_group: bool,
    new_log_file: bool,
) -> std::io::Result<()> {
    general_log("Test_Log.md", log, issql, header, last_in_group, new_log_file)
}

pub fn log_speed_tests(
    log: &str,
) -> std::io::Result<()> {
    general_log("../speed_test_data/Speed_Test_Result.md", log,  false, false,  false, false)
}



pub fn parse_json_to_map_vector(input: &str) -> HashMap<String, Vec<String>> {
    // Parse the JSON string to a serde_json::Value
    let json_value: Value = serde_json::from_str(input).expect("Invalid JSON format");

    // Create a HashMap to store the owned_map
    let mut owned_map = HashMap::new();

    // Iterate over the JSON object and insert into the HashMap
    if let Value::Object(map) = json_value {
        for (key, value) in &map {
            if let Value::Array(array) = value {
                let vec: Vec<String> = array
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string())) // Convert each element to `String`
                    .collect();
                owned_map.insert(key.to_string(), vec); // Insert owned `String` key and vector into HashMap
            }
        }
    }

    owned_map
}

pub fn parse_json_to_map_string(input: &str) -> HashMap<String, String> {
    // Parse the JSON string into a `serde_json::Value`
    let json_value: Value = serde_json::from_str(input).expect("Invalid JSON format");

    // Temporary storage for owned data
    let mut owned_map = HashMap::new();

    // Create the owned `HashMap<String, String>`
    if let Value::Object(map) = json_value {
        for (key, value) in map {
            if let Value::String(v) = value {
                owned_map.insert(key, v);
            }
        }
    }

    owned_map
}

pub fn parse_json_to_map_usize(input: &str) -> HashMap<String, usize> {
    // Parse the JSON string into a `serde_json::Value`
    let json_value: Value = serde_json::from_str(input).expect("Invalid JSON format");

    // Create a HashMap to store the result with owned data
    let mut result = HashMap::new();

    // Populate `result` as `HashMap<String, usize>`
    if let Value::Object(map) = json_value {
        for (key, value) in map {
            if let Value::Number(num) = value {
                // Convert the JSON number to usize if possible
                if let Some(u) = num.as_u64() {
                    result.insert(key, u as usize); // Insert owned `String` and `usize`
                }
            }
        }
    }

    result
}

/// Parses the JSON input and returns a `Vec<(String, String)>`
pub fn parse_json_to_tuple_vec(input: &str) -> Vec<(String, String)> {
    // Parse the JSON string into a `serde_json::Value`
    let json_value: Value = serde_json::from_str(input).expect("Invalid JSON format");

    // Create a vector to store the result
    let mut result = Vec::new();

    // Ensure it's a JSON object and extract key-value pairs
    if let Value::Object(map) = json_value {
        for (key, value) in map {
            if let Value::String(v) = value {
                result.push((key, v)); // Insert owned `String` key and value as tuple
            }
        }
    }

    result
}

pub fn parse_json_to_string_vec(input: &str) -> Vec<String> {
    // Parse the JSON string into a `serde_json::Value`
    let json_value: Value = serde_json::from_str(input).expect("Invalid JSON format");

    // Create a vector to store the values
    let mut result = Vec::new();

    // Ensure it's a JSON object and collect values
    if let Value::Object(map) = json_value {
        for value in map.values() {
            if let Value::String(v) = value {
                result.push(v.clone());
            }
        }
    }

    result
}

pub fn parse_to_vec(input: &str) -> Vec<String> {
    // Remove square brackets and whitespace, then split by comma
    input
        .trim_matches(['[', ']']) // Remove square brackets
        .split(",") // Split by comma
        .map(|s| s.trim().trim_matches('\'')) // Trim whitespace and single quotes
        .map(|s| s.to_string()) // Convert each &str to String
        .collect() // Collect into Vec<String>
}

/// Creates a borrowed view of the owned `HashMap<String, usize>`
pub fn create_borrowed_view_usize<'a>(
    owned_map: &'a HashMap<String, usize>,
) -> HashMap<&'a str, usize> {
    let mut borrowed_map = HashMap::new();
    for (key, value) in owned_map {
        borrowed_map.insert(key.as_str(), *value);
    }
    borrowed_map
}

pub fn create_borrowed_view_string<'a>(
    owned_map: &'a HashMap<String, String>,
) -> HashMap<&'a str, &'a str> {
    let mut borrowed_map = HashMap::new();
    for (key, value) in owned_map {
        borrowed_map.insert(key.as_str(), value.as_str());
    }
    borrowed_map
}

/// Creates a borrowed view `HashMap<&String, Vec<&String>>` from an owned `HashMap<String, Vec<String>>`
pub fn create_borrowed_view_vector<'a>(
    owned_map: &'a HashMap<String, Vec<String>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut borrowed_map = HashMap::new();

    for (key, value_vec) in owned_map {
        // Create a vector of references for each `Vec<String>`
        let borrowed_vec: Vec<&str> = value_vec.iter().map(|s| s.as_str()).collect();
        borrowed_map.insert(key.as_str(), borrowed_vec);
    }

    borrowed_map
}

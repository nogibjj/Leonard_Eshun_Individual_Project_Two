use super::util;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use util::db_path;

// Function to extract data from a URL and save it to a file
pub fn extract(url: &str, file_name: &str) -> Result<&'static str, Box<dyn std::error::Error>> {
    // Fetch the URL content
    let response = get(url)?;

    let mut file = File::create(file_name)?;

    // Write the content to the file
    file.write_all(&response.bytes()?)?;

    Ok("Extract Successful")
}

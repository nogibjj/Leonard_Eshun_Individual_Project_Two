use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;


// Function to extract data from a URL and save it to a file
pub fn extract(url: &str, file_name: &str) -> Result<&'static str, Box<dyn std::error::Error>> {
    // Fetch the URL content
    let response = get(url)?;

    let mut file = File::create(file_name)?;

    // Write the content to the file
    file.write_all(&response.bytes()?)?;

    Ok("Extract Successful")
}


#[cfg(test)]
mod tests{
    use std::fs;
    use clap::error::Result;
    use super::super::util::log_tests;
    use super::extract;

    // Test Extract
    #[test]
    fn test_extract() -> Result<()> {
        let _ = log_tests("Extraction Test", true, true,  false, false);
        let _ = log_tests("Removing existing CSV file if it exists", false, false,  false, false);

        let file_path = "air_quality.csv";
        if fs::metadata(file_path).is_ok() {
            fs::remove_file(file_path)?;
        }

        let _ = log_tests("Confirming that CSV file doesn't exist...", false, false,  false, false);
        assert!(!fs::metadata("population_bar.png").is_ok());
        let _ = log_tests("Test Successful", false, false,  false, false);

        let _ = log_tests("Extracting data and saving...", false, false,  false, false);
        let _ = extract("https://data.cityofnewyork.us/resource/c3uy-2p5r.csv?$limit=200000", file_path);

        let _ = log_tests("Testing if CSV file exists...", false, false,  false, false);
        assert!(fs::metadata(file_path).is_ok());
        let _ = log_tests("Extraction Test Successful", true, false, false,  false);

        Ok(())
    }
}
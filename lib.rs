use std::fs::File;
use std::io::Write;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://raw.githubusercontent.com/MainakRepositor/Datasets/master/Chennai%20rain/chennai_reservoir_rainfall.csv";
    let filepath = "rainfall.csv";
    extract(url, filepath).await?;
    Ok(())
}

async fn extract(url: &str, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Download the data from the URL
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Get the bytes of the response
        let bytes = response.bytes().await?;

        // Open a file at the specified path for writing
        let mut file = File::create(Path::new(filepath))?;

        // Write the downloaded bytes to the file
        file.write_all(&bytes)?;

        println!("Data downloaded and saved to {}", filepath);
    } else {
        println!("Failed to download data.");
    }

    Ok(())
}

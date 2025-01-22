use threat_modeling::utils::helpers::*;
use std::path::Path;

#[tokio::main]
async fn main() {
    // User input file path
    println!("Please enter the path to the JSON file:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let file_path = Path::new(input.trim());
    if !file_path.exists() {
        eprintln!("File does not exist");
    }

    // Read JSON file
    let json = read_json(file_path).unwrap();

    // Parse the JSON file into the internal data structure
    let assets = parse_asset_inventory_into_asset_model(json);
    println!("Parsed assets: {:?}", assets);
}

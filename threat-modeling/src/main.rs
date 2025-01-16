use threat_modeling::utils::helpers::*;
use std::path::Path;
use auto_test_macro::auto_test_args;

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

#[auto_test_args(4, 5)]
pub fn sample_function_with_args(a: i32, b: i32) -> Result<(), &'static str> {
    if a + b > 5 {
        Ok(())
    } else {
        Err("Sum is too small")
    }
}

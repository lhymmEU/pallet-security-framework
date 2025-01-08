use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() {
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
    println!("JSON file read successfully: {:?}", json["assets"][0]["name"]);
}

// -----------------------------------------------Helper Functions----------------------------------------------

// Read JSON file into a JSON object
fn read_json(file_path: &Path) -> Result<serde_json::Value, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader)?;
    Ok(json)
}

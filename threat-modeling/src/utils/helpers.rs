use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::utils::assets::*;
// -----------------------------------------------Helper Functions----------------------------------------------

// Read JSON file into a JSON object
pub fn read_json(file_path: &Path) -> Result<serde_json::Value, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader)?;
    Ok(json)
}

// Parse the JSON file into the internal data structure
pub fn parse_asset_inventory_into_asset_model(assets: serde_json::Value) -> Vec<Asset> {
    let mut result = Vec::new();
    
    // Extract the assets array
    let assets_array = assets["assets"].as_array().unwrap();
    
    for asset in assets_array {
        // Parse visibility
        let visibility = match asset["visibility"].as_str().unwrap_or("none") {
            "public" => Visibility::Public,
            "private" => Visibility::Private,
            _ => Visibility::None,
        };

        let name = asset["name"].as_str().unwrap_or("").to_string();
        
        // Parse category
        let category = match asset["category"].as_object().and_then(|c| c.keys().next()).map(String::as_str) {
            Some("PublicFunction") => {
                let params = asset["category"]["PublicFunction"][1]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|p| Parameter {
                        name: p[0].as_str().unwrap_or("").to_string(),
                        param_type: p[1].as_str().unwrap_or("").to_string(),
                    })
                    .collect();
                
                AssetCategory::PublicFunction {
                    parameters: params,
                    return_type: None,
                }
            },
            Some("Helper") => {
                let params = asset["category"]["Helper"][1]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|p| Parameter {
                        name: p[0].as_str().unwrap_or("").to_string(),
                        param_type: p[1].as_str().unwrap_or("").to_string(),
                    })
                    .collect();
                
                AssetCategory::Helper {
                    parameters: params,
                    return_type: None,
                }
            },
            Some("Storage") => {
                AssetCategory::Storage(StorageConfig {
                    visibility: visibility.clone(),
                    name: name.clone(),
                })
            },
            Some("Constant") => AssetCategory::Constant {
                value_type: "".to_string(), // Type information not provided in JSON
                name: name.clone(),
            },
            Some("Events") => AssetCategory::Event {
                name: name.clone(),
                fields: Vec::new(), // Event fields not provided in JSON
            },
            Some("Error") => AssetCategory::Error {
                name: name.clone(),
                fields: Vec::new(), // Error fields not provided in JSON
            },
            _ => continue, // Skip invalid categories
        };

        // Create asset with default properties
        let asset = Asset {
            name,
            visibility,
            category,
            properties: Properties::default(),
        };

        result.push(asset);
    }

    result
}
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use threat_modeling::utils::llm::query_llm;

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

// ----------------------------------------Pallet Model Data Structures-------------------------------------
// This is the internal model for pallets

/// Asset visibility classification
#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    None,
}

/// Asset category classification
#[derive(Debug, Clone)]
pub enum AssetCategory {
    PublicFunction {
        parameters: Vec<Parameter>,
        return_type: Option<String>,
    },
    Helper {
        parameters: Vec<Parameter>,
        return_type: Option<String>,
    },
    Storage(StorageConfig),
    Constant {
        name: String,
        value_type: String,
    },
    Event {
        name: String,
        fields: Vec<Parameter>,
    },
    Error {
        name: String,
        fields: Vec<Parameter>,
    },
}

/// Parameter definition for functions
#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
    param_type: String,
}

// Supporting struct for Storage variant
#[derive(Debug, Clone)]
pub struct StorageConfig {
    visibility: Visibility,
    name: String,
}

/// Core asset representation
#[derive(Debug, Clone)]
pub struct Asset {
    name: String,
    visibility: Visibility,
    category: AssetCategory,
    properties: Properties,
}

/// Risk level is for the asset itself, not the threats mapped to it
/// This is used to prioritize assets for security analysis
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    #[default]
    Low,
}

// -----------------------------------------------Threat Model Data Structures-------------------------------------

/// Threat model data structure
/// This structure will be used to:
/// 1. Store known threats
/// 2. Store translated threats from natural language using LLM
/// 
/// Basically, this represent the question "what needs to be checked and how to check it"
#[derive(Debug, Clone)]
pub struct Threat {
    // The name of the threat, e.g. "User controled input"
    name: ThreatType,
    // How to check the threat, e.g. "Input Sanitization".
    // This will only be the category of checking, the actual checking will be involved during symbolic execution, maybe through LLM tool calling and etc.
    how_to_check: SecurityCheck,
    // TODO: Add other fields

}

#[derive(Debug, Clone)]
enum SecurityCheck {
    InputSanitization,
}

#[derive(Debug, Clone)]
enum ThreatType {
    UserControlledInput,
}

// Dummy implementation for input sanitization
fn input_sanitization() {
    println!("Check this input for length and character set");
}

/// Extensible security properties
/// This structure should provide the threat mapping function enough information to map threats to assets
#[derive(Debug, Clone, Default)]
pub struct Properties {
    // Threats mapped to this asset
    threats: Vec<Threat>,
    // Risk level is for the asset itself, not the threats mapped to it
    risk_level: RiskLevel,
    // Input validation rules for this asset, prepared by internal functions
    validation_rules: Vec<ValidationRule>,
    // Intended state transitions caused by this asset, prepared by internal functions
    state_transitions: Vec<StateTransition>,
    // Value constraints for this asset, prepared by internal functions
    value_constraints: Vec<ValueConstraint>,
}

#[derive(Debug, Clone)]
pub struct ValidationRule {
    field: String,
    rule_type: String,
    constraint: String,
}

#[derive(Debug, Clone)]
pub struct StateTransition {
    from_state: String,
    to_state: String,
    constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValueConstraint {
    field: String,
    constraint_type: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct MitigationStrategy {
    strategy_type: String,
    description: String,
    implementation_status: bool,
}

/// Central asset registry for security analysis
pub struct SecurityAssetRegistry {
    assets: HashMap<String, Asset>,
    function_dependencies: HashMap<String, Vec<String>>,
    storage_access_map: HashMap<String, Vec<String>>,
    vulnerability_index: HashMap<String, Vec<Threat>>,
}

impl SecurityAssetRegistry {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            function_dependencies: HashMap::new(),
            storage_access_map: HashMap::new(),
            vulnerability_index: HashMap::new(),
        }
    }

    pub fn register_asset(&mut self, asset: Asset) {
        let asset_name = asset.name.clone();
        
        // Update dependency mappings
        if let AssetCategory::PublicFunction { .. } = &asset.category {
            self.function_dependencies.insert(
                asset_name.clone(),
                vec![]
            );
        }
        
        // Update storage access patterns
        if let AssetCategory::Storage(_) = &asset.category {
            self.storage_access_map.insert(
                asset_name.clone(),
                vec![]
            );
        }
        
        // Register asset
        self.assets.insert(asset_name, asset);
    }

    pub fn get_asset_properties(&self, name: &str) -> Option<&Properties> {
        self.assets.get(name).map(|asset| &asset.properties)
    }

    pub fn update_security_properties(
        &mut self,
        asset_name: &str,
        properties: Properties,
    ) -> bool {
        if let Some(asset) = self.assets.get_mut(asset_name) {
            asset.properties = properties;
            true
        } else {
            false
        }
    }

    pub fn get_dependent_assets(&self, asset_name: &str) -> Vec<String> {
        self.function_dependencies
            .get(asset_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_storage_access_pattern(&self, storage_name: &str) -> Vec<String> {
        self.storage_access_map
            .get(storage_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn register_vulnerability(
        &mut self,
        asset_name: &str,
        threat: Threat,
    ) {
        self.vulnerability_index
            .entry(asset_name.to_string())
            .or_insert_with(Vec::new)
            .push(threat);
    }

    pub fn get_asset_vulnerabilities(&self, asset_name: &str) -> Vec<Threat> {
        self.vulnerability_index
            .get(asset_name)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get_assets_by_risk_level(&self, risk_level: RiskLevel) -> Vec<&Asset> {
        self.assets
            .values()
            .filter(|asset| asset.properties.risk_level == risk_level)
            .collect()
    }

    pub fn get_public_interfaces(&self) -> Vec<&Asset> {
        self.assets
            .values()
            .filter(|asset| {
                matches!(asset.category, AssetCategory::PublicFunction { .. }) &&
                asset.visibility == Visibility::Public
            })
            .collect()
    }
}

// -----------------------------------------------Helper Functions----------------------------------------------

// Read JSON file into a JSON object
fn read_json(file_path: &Path) -> Result<serde_json::Value, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader)?;
    Ok(json)
}

// Parse the JSON file into the internal data structure
fn parse_asset_inventory_into_asset_model(assets: serde_json::Value) -> Vec<Asset> {
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
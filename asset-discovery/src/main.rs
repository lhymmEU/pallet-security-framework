use quote::quote;
use serde::Serialize;
use std::{collections::HashMap, error::Error, fs, path::Path};
use syn::{visit::Visit, Attribute, File};

fn main() -> Result<(), AppError> {
    // Read source code
    let file_content = loop {
        match source_code_reader() {
            // If we successfully read the file content, exit the loop and return the content.
            // This breaks out of the loop with the value 'content' which becomes the value of file_content
            Ok(content) => break content,
            Err(e) => {
                eprintln!("Error reading source code: {}", e);
                if let Some(source) = e.source() {
                    eprintln!("Caused by: {}", source);
                }
                continue;
            }
        }
    };

    // Parse source code into self-defined format [AssetInventory]
    let output = parser(file_content).map_err(|e| {
        eprintln!("Error parsing source code: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        e
    })?;

    // Write result to file
    result_writer(output).map_err(|e| {
        eprintln!("Error writing results: {}", e);
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        e
    })?;
    
    Ok(())
}

/// Helper function to read the Rust source code file
fn source_code_reader() -> Result<String, AppError> {
    println!("Please enter the path to the source code:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| AppError::IoError(e))?;
    
    let file_path = Path::new(input.trim());
    if !file_path.exists() {
        return Err(AppError::InvalidInput(format!(
            "File does not exist: {}", 
            file_path.display()
        )));
    }
    
    if !file_path.extension().map_or(false, |ext| ext == "rs") {
        return Err(AppError::InvalidInput(format!(
            "Invalid file extension for {}, expected .rs file", 
            file_path.display()
        )));
    }

    fs::read_to_string(file_path).map_err(|e| AppError::IoError(e))
}

/// Helper function to write the result to user specified location
fn result_writer(result: AssetInventory) -> Result<(), AppError> {
    let result_string = result.to_json()
        .map_err(|e| AppError::SerializationError(e))?;
    let output_path = Path::new("./asset-inventory.JSON");
    
    fs::write(output_path, result_string)
        .map_err(|e| AppError::IoError(e))
}

/// Unified visitor to collect all relevant pallet items
struct PalletVisitor {
    functions: HashMap<String, String>,                   // (function name, visibility)
    params: Vec<(String, Vec<(String, String)>)>,         // (function name, [(param name, param type)])
    storage_items: HashMap<String, String>,               // (storage name, visibility)
    constants: Vec<String>,                               // constant names
    events: Vec<String>,                                  // event names
    errors: Vec<String>,                                  // error names
}

impl<'ast> Visit<'ast> for PalletVisitor {
    // Extract function information from impl blocks
    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        for item in &node.items {
            if let syn::ImplItem::Fn(method) = item {
                let fn_name = method.sig.ident.to_string();
                let visibility = match &method.vis {
                    syn::Visibility::Public(_) => "public",
                    _ => "private",
                };

                let mut param_info = Vec::new();
                for param in method.sig.inputs.iter() {
                    if let syn::FnArg::Typed(pat_type) = param {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            let param_name = pat_ident.ident.to_string();
                            let param_type = quote!(#pat_type).to_string();
                            param_info.push((param_name, param_type));
                        }
                    }
                }

                self.functions.insert(fn_name.clone(), visibility.to_string());
                self.params.push((fn_name, param_info));
            }
        }
        syn::visit::visit_item_impl(self, node);
    }

    // Extract storage items from pallet module
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.ident == "pallet" {
            if let Some((_, items)) = &node.content {
                for item in items {
                    if let syn::Item::Type(storage_type) = item {
                        if storage_type.attrs.iter().any(|_attr| true) {
                            let storage_name = storage_type.ident.to_string();
                            let visibility = match storage_type.vis {
                                syn::Visibility::Public(_) => "public",
                                _ => "private",
                            };
                            self.storage_items.insert(storage_name, visibility.to_string());
                        }
                    }
                }
            }
        }
        syn::visit::visit_item_mod(self, node);
    }

    // Extract constants from traits
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        for item in &node.items {
            if let syn::TraitItem::Type(constant) = item {
                if constant.attrs.iter().any(|attr| has_pallet_constant("pallet::constant".to_string(), attr)) {
                    self.constants.push(constant.ident.to_string());
                }
            }
        }
    }

    // Extract events and errors from enums
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        let is_event = node.attrs.iter().any(|attr| has_pallet_constant("pallet::event".to_string(), attr));
        let is_error = node.attrs.iter().any(|attr| has_pallet_constant("pallet::error".to_string(), attr));

        for variant in &node.variants {
            let name = variant.ident.to_string();
            if is_event {
                self.events.push(name);
            } else if is_error {
                self.errors.push(name);
            }
        }
    }
}

/// Parse source code with unified visitor
fn parser(code: String) -> Result<AssetInventory, AppError> {
    let syntax_tree: File = syn::parse_file(&code)?;

    // Initialize unified visitor
    let mut visitor = PalletVisitor {
        functions: HashMap::new(),
        params: Vec::new(),
        storage_items: HashMap::new(),
        constants: Vec::new(),
        events: Vec::new(),
        errors: Vec::new(),
    };

    // Visit all items in the file
    visitor.visit_file(&syntax_tree);

    let mut asset_inventory = AssetInventory { assets: Vec::new() };

    // Convert visitor data into assets
    for (function, params) in visitor.params {
        let visibility = visitor.functions.get(&function).unwrap();
        let category = if visibility == "public" {
            AssetCategory::PublicFunction(function.clone(), params)
        } else {
            AssetCategory::Helper(function.clone(), params)
        };
        asset_inventory.assets.push(Asset {
            visibility: visibility.to_string(),
            name: function.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for (storage_item, visibility) in visitor.storage_items {
        let category = AssetCategory::Storage(storage_item.clone(), visibility.clone());
        asset_inventory.assets.push(Asset {
            visibility: visibility.to_string(),
            name: storage_item.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for constant in visitor.constants {
        let category = AssetCategory::Constant(constant.clone());
        asset_inventory.assets.push(Asset {
            visibility: "none".to_string(),
            name: constant.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for event in visitor.events {
        let category = AssetCategory::Events(event.clone());
        asset_inventory.assets.push(Asset {
            visibility: "public".to_string(),
            name: event.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for error in visitor.errors {
        let category = AssetCategory::Error(error.clone());
        asset_inventory.assets.push(Asset {
            visibility: "public".to_string(),
            name: error.clone(),
            category,
        });
    }

    Ok(asset_inventory)
}

/// Asset Category
#[derive(Debug, Serialize)]
enum AssetCategory {
    /// Point of interest:
    /// 1. Sensitive data handling (e.g. balances, access control lists, etc.)
    /// 2. System configuration (e.g. system parameters, configurations, etc.)
    ///
    /// # Arguments
    /// * `String` - The name of the storage item
    Storage(String, String),
    /// Point of interest:
    /// 1. Sensitive information leak through event definition
    /// 2. Internal state leak through event parameters
    /// 3. System behavior leak through event emission patterns
    ///
    /// # Arguments
    /// * `String` - The name of the event
    Events(String),
    /// Point of interest:
    /// 1. Custom types that handle sensitive data
    /// 2. Enums that determine state transitions
    /// 3. Composite types containing priviledged information
    ///
    /// # Arguments
    /// * `String` - The name of the custom type
    /// * `String` - The purpose of the custom type
    // CustomType(String, String),
    /// Point of interest:
    /// 1. Constants that define security thresholds
    /// 2. These constants are defined by runtime implementation, it also could cause security issues
    ///
    /// # Arguments
    /// * `String` - The name of the constant
    Constant(String),
    /// Point of interest:
    /// 1. Weight calculations and resource limits
    ///
    /// # Arguments
    /// * `String` - The name of the function for which the weight is defined
    //Weight(String),
    /// Point of interest:
    /// 1. Internal state leak through error handling
    ///
    /// # Arguments
    /// * `String` - The name of the error
    Error(String),
    /// # Arguments
    /// * `String` - The name of the public function
    /// * `Vec<(String, String)>` - The parameters of the public function,
    ///   where the first string is the parameter name, and the second string is the parameter type
    PublicFunction(String, Vec<(String, String)>),
    /// Point of interest:
    /// 1. Internal helper functions handling priviledged operations
    /// 2. Validation logic
    ///
    /// # Arguments
    /// * `String` - The name of the helper function
    /// * `Vec<(String, String)>` - The parameters of the helper function,
    ///   where the first string is the parameter name, and the second string is the parameter type
    Helper(String, Vec<(String, String)>),
}

/// Asset Data Structure
#[derive(Debug, Serialize)]
struct Asset {
    visibility: String,
    name: String,
    category: AssetCategory,
}

/// Asset Inventory Data Structure
#[derive(Debug, Serialize)]
struct AssetInventory {
    assets: Vec<Asset>,
}

impl AssetInventory {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

// ----------------------------------------------Helper Functions--------------------------------------------------

fn has_pallet_constant(name: String, attrs: &Attribute) -> bool {
    let path_str = attrs
        .path()
        .segments
        .iter()
        .map(|seg| seg.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");

    path_str == name
}

// ----------------------------------------------Robust Error Handling----------------------------------------------

// Custom error type for the application
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(syn::Error),
    SerializationError(serde_json::Error),
    InvalidInput(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IoError(err) => write!(f, "IO Error: {}", err),
            AppError::ParseError(err) => write!(f, "Parse Error: {}", err),
            AppError::SerializationError(err) => write!(f, "Serialization Error: {}", err),
            AppError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

// Implement std::error::Error trait for better error handling
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::IoError(err) => Some(err),
            AppError::ParseError(err) => Some(err),
            AppError::SerializationError(err) => Some(err),
            AppError::InvalidInput(_) => None,
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<syn::Error> for AppError {
    fn from(err: syn::Error) -> Self {
        AppError::ParseError(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err)
    }
}

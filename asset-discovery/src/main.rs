use std::{collections::HashMap, fs, path::Path};
use syn::{visit::Visit, Error, File, ItemFn};
use quote::quote;
use serde::Serialize;

fn main() {
    // Read source code
    let file_content = source_code_reader().expect("Failed to read the Rust file.");

    // Parse source code
    let output = parser(file_content).expect("Failed to parse the Rust file.");

    // Write result to file
    result_writer(output).expect("Failed to write output file");
}

/// Helper function to read the Rust source code file
fn source_code_reader() -> Result<String, std::io::Error> {
    // Path to the source code
    println!("Please enter the path to the source code:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read output path");
    let file_path = Path::new(input.trim());

    let file_content = fs::read_to_string(file_path).expect("Failed to read the Rust file.");

    // Return the content as string
    return Ok(file_content);
}

/// Helper function to write the result to user specified location
fn result_writer(result: AssetInventory) -> Result<(), std::io::Error> {
    // Transform AssetInventory into JSON
    let result_string = result.to_json();

    let output_path = Path::new("./asset-inventory.JSON");

    fs::write(output_path, result_string).expect("Failed to write output file");
    Ok(())
}

/// Helper function to parse the source code
/// It needs to:
/// 1. Find public pallet calls ✅
/// 2. Extract function signatures of found public pallet calls ✅
/// 3. Find helper functions
/// 4. Extract storage items
/// 5. Find cross-pallet interaction vectors (e.g. AssetsFungibles and etc.)
/// 6. Write results to AssetInventory
fn parser(code: String) -> Result<AssetInventory, Error> {
    // Parse the file into a syn::File
    let syntax_tree: File = syn::parse_file(&code).expect("Failed to parse the Rust file.");

    // Visit the file to extract functions
    let mut fn_visitor = FunctionVisitor {
        functions: HashMap::new(),
        params: Vec::new(),
    };
    
    // Visit all items in the file
    fn_visitor.visit_file(&syntax_tree);

    let mut asset_inventory = AssetInventory {
        assets: Vec::new(),
    };

    // Parse visitor type into Asset type
    for (function, params) in fn_visitor.params {
        asset_inventory.assets.push(Asset {
            visibility: fn_visitor.functions.get(&function).unwrap().to_string(),
            name: function.clone(),
            category: AssetCategory::PublicFunction(function, params),
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
    Storage(String),
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
    CustomType(String, String),
    /// Point of interest:
    /// 1. Constants that define security thresholds
    ///
    /// # Arguments
    /// * `String` - The name of the constant
    /// * `String` - The purpose of the constant
    Constant(String, String),
    /// Point of interest:
    /// 1. Weight calculations and resource limits
    ///
    /// # Arguments
    /// * `String` - The name of the function for which the weight is defined
    Weight(String),
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
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Failed to serialize AssetInventory to JSON")
    }
}

/// Visitor to find functions in the Rust file.
struct FunctionVisitor {
    functions: HashMap<String, String>,             // Store function name and their visibility
    params: Vec<(String, Vec<(String, String)>)>, // (function name, parameter names and types)
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    // Substrate Pallet functions are nested in impl blocks
    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        // Visit all items in the impl block
        for item in &node.items {
            if let syn::ImplItem::Fn(method) = item {
                let fn_name = method.sig.ident.to_string();
                
                // Determine function visibility
                let visibility = match &method.vis {
                    syn::Visibility::Public(_) => "public",
                    _ => "private"
                };

                // Extract parameter names and types
                let mut param_info = Vec::new();
                for param in method.sig.inputs.iter() {
                    if let syn::FnArg::Typed(pat_type) = param {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            let param_name = pat_ident.ident.to_string();
                            let param_type = quote!(#pat_type.ty).to_string();
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
}
use quote::quote;
use serde::Serialize;
use std::{collections::HashMap, fs, path::Path};
use syn::{visit::Visit, Attribute, Error, File};

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

    // Visit the file to extract storage items
    let mut storage_visitor = StorageVisitor {
        storage_items: HashMap::new(),
    };

    // Visit the file to extract constants
    let mut constant_visitor = ConstantVisitor {
        constants: Vec::new(),
    };

    // Visit the file to extract events
    let mut event_visitor = EventVisitor {
        events: Vec::new(),
    };

    // Visit the file to extract errors 
    let mut error_visitor = ErrorVisitor {
        errors: Vec::new(),
    };

    // Visit all items in the file
    fn_visitor.visit_file(&syntax_tree);
    storage_visitor.visit_file(&syntax_tree);
    constant_visitor.visit_file(&syntax_tree);
    event_visitor.visit_file(&syntax_tree);
    error_visitor.visit_file(&syntax_tree);

    let mut asset_inventory = AssetInventory { assets: Vec::new() };

    // Parse visitor type into Asset type
    for (function, params) in fn_visitor.params {
        let visibility = fn_visitor.functions.get(&function).unwrap();
        let category: AssetCategory;
        if visibility == "public" {
            category = AssetCategory::PublicFunction(function.clone(), params);
        } else {
            category = AssetCategory::Helper(function.clone(), params);
        }
        asset_inventory.assets.push(Asset {
            visibility: visibility.to_string(),
            name: function.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for (storage_item, visibility) in storage_visitor.storage_items {
        let category = AssetCategory::Storage(storage_item.clone(), visibility.clone());
        asset_inventory.assets.push(Asset {
            visibility: visibility.to_string(),
            name: storage_item.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for constant in constant_visitor.constants {
        let category = AssetCategory::Constant(constant.clone());
        asset_inventory.assets.push(Asset {
            visibility: "none".to_string(),
            name: constant.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for event in event_visitor.events {
        let category = AssetCategory::Events(event.clone());
        asset_inventory.assets.push(Asset {
            visibility: "public".to_string(),
            name: event.clone(),
            category,
        });
    }

    // Parse visitor type into Asset type
    for error in error_visitor.errors {
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
    CustomType(String, String),
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
    functions: HashMap<String, String>, // Store function name and their visibility
    params: Vec<(String, Vec<(String, String)>)>, // (function name, parameter names and types)
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    // Substrate Pallet functions are nested in impl blocks
    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        // Visit all items in the impl block
        for item in &node.items {
            // Extract function information
            if let syn::ImplItem::Fn(method) = item {
                let fn_name = method.sig.ident.to_string();

                // Determine function visibility
                let visibility = match &method.vis {
                    syn::Visibility::Public(_) => "public",
                    _ => "private",
                };

                // Extract parameter names and types
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

                self.functions
                    .insert(fn_name.clone(), visibility.to_string());
                self.params.push((fn_name, param_info));
            }
        }
        syn::visit::visit_item_impl(self, node);
    }
}

/// Visitor to find storage items in the Rust file.
struct StorageVisitor {
    storage_items: HashMap<String, String>, // (storage item name, storage item visibility)
}

impl<'ast> Visit<'ast> for StorageVisitor {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        // Check if this is the pallet module
        if node.ident == "pallet" {
            // Visit the module contents if available
            if let Some((_, items)) = &node.content {
                for item in items {
                    // Look for storage items marked with #[pallet::storage]
                    if let syn::Item::Type(storage_type) = item {
                        // Check if the type has the #[pallet::storage] attribute
                        //println!("Attributes for {} is {:#?}", storage_type.ident.to_string(), storage_type.attrs);
                        if storage_type.attrs.iter().any(|_attr| true) {
                            let storage_name = storage_type.ident.to_string();
                            match storage_type.vis {
                                syn::Visibility::Public(_) => {
                                    self.storage_items
                                        .insert(storage_name, "public".to_string());
                                }
                                _ => {
                                    self.storage_items
                                        .insert(storage_name, "private".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        // Continue visiting other modules
        syn::visit::visit_item_mod(self, node);
    }
}

/// Visitor to find constants in the Rust file.
struct ConstantVisitor {
    constants: Vec<String>,
}

impl<'ast> Visit<'ast> for ConstantVisitor {
    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        // Then process the current trait's items
        for item in &node.items {
            if let syn::TraitItem::Type(constant) = item {
                // Check if any attribute matches #[pallet::constant]
                // One interesting thing to notice is that, doc comments are considered as attributes in the form of #[doc = "..."]
                // Which means, in the eyes of syn crate a constant with comments looks like this:
                //  #[doc = "..."]
                //  #[pallet::constant]
                //  type StringLimit: Get<u32>;
                let res = constant
                    .attrs
                    .iter()
                    .any(|attr| has_pallet_constant("pallet::constant".to_string(), attr));

                if res {
                    let constant_name = constant.ident.to_string();
                    self.constants.push(constant_name);
                }
            }
        }
    }
}

// Visitor to find events in the Rust file.
struct EventVisitor {
    events: Vec<String>,
}

impl<'ast> Visit<'ast> for EventVisitor {
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        // First check if this enum has the #[pallet::event] attribute
        let res = node.attrs.iter().any(|attr| {
            has_pallet_constant("pallet::event".to_string(), attr)
        });

        // If the enum has the #[pallet::event] attribute, then process all variants
        if res {
            for variant in &node.variants {
                let event_name = variant.ident.to_string();
                self.events.push(event_name);
            }
        }
    }
}

// Visitor to find errors in the Rust file.
struct ErrorVisitor {
    errors: Vec<String>,
}

impl<'ast> Visit<'ast> for ErrorVisitor {
    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        // First check if this enum has the #[pallet::error] attribute
        let res = node.attrs.iter().any(|attr| {
            has_pallet_constant("pallet::error".to_string(), attr)
        });

        // If the enum has the #[pallet::error] attribute, then process all variants
        if res {
            for variant in &node.variants {
                let error_name = variant.ident.to_string();
                self.errors.push(error_name);
            }
        }
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

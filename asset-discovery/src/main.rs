use std::{fs, path::Path};
use syn::{visit::Visit, File, ImplItem, ItemImpl};

/// Asset Category
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
struct Asset {
    name: String,
    category: AssetCategory,
}

/// Asset Inventory Data Structure
struct AssetInventory {
    assets: Vec<Asset>,
}

/// Visitor to find functions in the Rust file.
struct FunctionVisitor {
    functions: Vec<String>,             // Store function names
    params: Vec<(String, Vec<String>)>, // (function_name, parameter_names)
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        // Visit methods inside the impl block, looking for permissionless calls
        for item in &node.items {
            if let ImplItem::Fn(method) = item {
                let fn_name = method.sig.ident.to_string();

                // Extract parameter names
                let mut param_names = Vec::new();
                for param in method.sig.inputs.iter() {
                    if let syn::FnArg::Typed(pat_type) = param {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            param_names.push(pat_ident.ident.to_string());
                        }
                    }
                }

                self.functions.push(fn_name.clone());
                self.params.push((fn_name, param_names));
            }
        }

        // Continue visiting the implementation block
        syn::visit::visit_item_impl(self, node);
    }
}

fn main() {
    let file_content = source_code_reader().expect("Failed to read the Rust file.");

    // Parse the file into a syn::File
    let syntax_tree: File = syn::parse_file(&file_content).expect("Failed to parse the Rust file.");

    // Visit the file to extract functions
    let mut visitor = FunctionVisitor {
        functions: Vec::new(),
        params: Vec::new(),
    };
    visitor.visit_file(&syntax_tree);

    // Write results to file
    let mut output = String::from("Parsed Functions:\n");
    for (function, params) in visitor.params {
        output.push_str(&format!("\n=== Function: {} ===\n", function));
        if params.is_empty() {
            output.push_str("  No parameters\n");
        } else {
            output.push_str("  Parameters:\n");
            for param in params {
                output.push_str(&format!("    • {}\n", param));
            }
        }
        output.push_str(&format!("{}\n", "=".repeat(function.len() + 14)));
    }

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
fn result_writer(result: String) -> Result<(), std::io::Error> {
    // Path to ghe generated result file
    println!("Please enter the path to the result file:");
    let mut output_path = String::new();
    std::io::stdin()
        .read_line(&mut output_path)
        .expect("Failed to read output path");

    fs::write(output_path, result).expect("Failed to write output file");
    Ok(())
}

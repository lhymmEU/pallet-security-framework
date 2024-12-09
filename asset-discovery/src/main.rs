use std::{fs, path::Path};
use syn::{visit::Visit, File, ImplItem, ItemImpl};

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
    // Path to the Rust source file
    println!("Please enter the path to the Rust file:");
    let mut input = String::new();
    // Input the location to the pallet file you want to analyze
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let file_path = Path::new(input.trim());

    // Path to ghe generated result file
    println!("Please enter the path to the result file:");
    let mut output_path = String::new();
    std::io::stdin()
        .read_line(&mut output_path)
        .expect("Failed to read output path");

    // Read the file content
    let file_content = fs::read_to_string(file_path).expect("Failed to read the Rust file.");

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
    fs::write(output_path.trim(), output).expect("Failed to write output file");
}

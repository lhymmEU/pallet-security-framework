use std::collections::HashMap;

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
    pub name: ThreatType,
    // How to check the threat, e.g. "Input Sanitization".
    // This will only be the category of checking, the actual checking happens via symbolic execution, maybe through LLM tool calling and etc.
    pub how_to_check: SecurityCheck,
    // TODO: Add other fields
}

#[derive(Debug, Clone)]
pub enum SecurityCheck {
    InputSanitization,
}

impl SecurityCheck {
    pub fn validate_input(&self) -> Option<()> {
        match self {
            SecurityCheck::InputSanitization => {
                println!("Validating input.");
                Some(())
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ThreatType {
    UserControlledInput,
}

#[derive(Debug, Clone, Default)]
pub struct ForSymbolicExecution {
    // Pallet-specific semantic meaning of the asset, e.g. "control staking operation"
    semantic_meaning: String,
    // Compliance requirements for the asset, e.g. "must be compliant with xxx regulations"
    compliance_requirements: String,
    // Business requirements for the asset, e.g. "only owner can call this function"
    // This field also includes domain-specific error behavior, e.g. "receipient must be a valid ss58 address"
    business_requirements: Vec<String>,
    // Valid assumptions regarding what preconditions should lead to what postconditions
    // e.g. "If sender is deducted 1 token, then the receipient must be credited with 1 token"
    // TODO: The type for pre- and post-conditions should not be String
    // If the asset is an external function, this represents the abstract behavior of the function, e.g. "given A the function will return B"
    valid_assumptions: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MitigationStrategy {
    strategy_type: String,
    description: String,
    implementation_status: bool,
}
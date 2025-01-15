use crate::utils::threats::*;

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
    pub name: String,
    pub param_type: String,
}

// Supporting struct for Storage variant
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub visibility: Visibility,
    pub name: String,
}

/// Core asset representation
#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub visibility: Visibility,
    pub category: AssetCategory,
    pub properties: Properties,
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

/// Extensible security properties
/// This structure is generated during the asset <-> threat mapping process and added to the asset model
#[derive(Debug, Clone, Default)]
pub struct Properties {
    // Threats mapped to this asset
    pub threats: Vec<Threat>,
    // Risk level is for the asset itself, not the threats mapped to it
    pub risk_level: RiskLevel,
    // Information will be used for symbolic execution and targeted unit tests generation
    pub for_symbolic_execution: ForSymbolicExecution
}

use serde::{Deserialize, Serialize};

/// Represents a function parameter that could be security-critical
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    name: String,
    param_type: String,
}

/// Represents a function signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    name: String,
    parameters: Vec<Parameter>,
    return_type: Option<String>,
}

/// Core threat definition with specific patterns to match
/// This approach reduce the threat modeling problem to a pattern matching problem
/// The success of this approach depends on the quality of the patterns and the ability to match them
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatPattern {
    // Patterns that indicate potential threat in function names
    function_patterns: Vec<String>,
    // Patterns that indicate dangerous parameter types
    parameter_patterns: Vec<String>,
    // Risk score for this threat (0-100)
    risk_score: u8,
}

/// Analysis result for a specific function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionThreatAnalysis {
    function: FunctionSignature,
    risk_score: u8,
    dangerous_params: Vec<Parameter>,
    matched_patterns: Vec<String>,
}

/// Main threat oracle implementation
pub struct ThreatOracle {
    patterns: Vec<ThreatPattern>,
}

impl ThreatOracle {
    pub fn new(patterns: Vec<ThreatPattern>) -> Self {
        Self { patterns }
    }

    /// Analyze a function signature against known threat patterns
    pub fn analyze_function(&self, func: &FunctionSignature) -> FunctionThreatAnalysis {
        let mut risk_score = 0;
        let mut dangerous_params = Vec::new();
        let mut matched_patterns = Vec::new();

        for pattern in &self.patterns {
            // Check function name against patterns
            for func_pattern in &pattern.function_patterns {
                if func.name.contains(func_pattern) {
                    risk_score = risk_score.max(pattern.risk_score);
                    matched_patterns.push(format!("Function name matches pattern: {}", func_pattern));
                }
            }

            // Check parameters against patterns
            for param in &func.parameters {
                for param_pattern in &pattern.parameter_patterns {
                    if param.param_type.contains(param_pattern) {
                        dangerous_params.push(param.clone());
                        matched_patterns.push(format!("Parameter {} matches pattern: {}", 
                            param.name, param_pattern));
                    }
                }
            }
        }

        FunctionThreatAnalysis {
            function: func.clone(),
            risk_score,
            dangerous_params,
            matched_patterns,
        }
    }

    /// Analyze a list of functions and return them sorted by risk
    pub fn analyze_functions(&self, functions: Vec<FunctionSignature>) 
        -> Vec<FunctionThreatAnalysis> 
    {
        let mut results: Vec<FunctionThreatAnalysis> = functions.iter()
            .map(|f| self.analyze_function(f))
            .collect();

        // Sort by risk score in descending order
        results.sort_by(|a, b| b.risk_score.cmp(&a.risk_score));
        results
    }
}

/// Function to parse function signatures from a text file
pub fn parse_function_file(content: &str) -> Vec<FunctionSignature> {
    content.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                let name = parts[0].trim().to_string();
                let params: Vec<Parameter> = parts[1].split(',')
                    .filter_map(|p| {
                        let param_parts: Vec<&str> = p.trim().split(':').collect();
                        if param_parts.len() == 2 {
                            Some(Parameter {
                                name: param_parts[0].trim().to_string(),
                                param_type: param_parts[1].trim().to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                
                let return_type = parts.get(2)
                    .map(|rt| rt.trim().to_string());

                Some(FunctionSignature {
                    name,
                    parameters,
                    return_type,
                })
            } else {
                None
            }
        })
        .collect()
}

// Example usage
pub fn create_default_patterns() -> Vec<ThreatPattern> {
    vec![
        ThreatPattern {
            function_patterns: vec![
                "transfer".to_string(),
                "send".to_string(),
                "mint".to_string(),
                "burn".to_string(),
            ],
            parameter_patterns: vec![
                "Amount".to_string(),
                "Balance".to_string(),
                "AccountId".to_string(),
            ],
            risk_score: 80,
        },
        ThreatPattern {
            function_patterns: vec![
                "set".to_string(),
                "update".to_string(),
                "modify".to_string(),
            ],
            parameter_patterns: vec![
                "Config".to_string(),
                "Storage".to_string(),
            ],
            risk_score: 60,
        },
    ]
}
use rmcp::schemars;
use serde::Deserialize;

/// Paramètres du tool `calculate`
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CalculateParams {
    /// Arithmetic expression to evaluate — e.g. "2 + 3 * 4" or "(10 - 2) / 4"
    pub expression: String,
}

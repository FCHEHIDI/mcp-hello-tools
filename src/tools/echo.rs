use rmcp::schemars;
use serde::Deserialize;

/// Paramètres du tool `echo`
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EchoParams {
    /// The message to echo back unchanged
    pub message: String,
}

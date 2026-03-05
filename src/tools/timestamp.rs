use rmcp::schemars;
use serde::Deserialize;

/// Paramètres du tool `get_timestamp`
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TimestampParams {
    /// IANA timezone name (e.g. "UTC", "Europe/Paris", "America/New_York").
    /// Defaults to UTC when absent.
    pub timezone: Option<String>,
}

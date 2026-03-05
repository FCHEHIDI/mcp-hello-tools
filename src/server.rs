use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
};

use crate::tools::{
    calculate::CalculateParams,
    echo::EchoParams,
    timestamp::TimestampParams,
};

#[derive(Clone)]
pub struct McpServer {
    tool_router: ToolRouter<McpServer>,
}

#[tool_router]
impl McpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Returns the message unchanged — verifies that the MCP connection works
    #[tool(description = "Returns the message unchanged — verifies that the MCP connection works")]
    fn echo(
        &self,
        Parameters(params): Parameters<EchoParams>,
    ) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(params.message)]))
    }

    /// Returns the current time, optionally converted to the requested timezone
    #[tool(description = "Returns the current timestamp; accepts an optional IANA timezone (e.g. 'UTC', 'Europe/Paris')")]
    fn get_timestamp(
        &self,
        Parameters(params): Parameters<TimestampParams>,
    ) -> Result<CallToolResult, McpError> {
        use chrono::Utc;
        use chrono_tz::Tz;

        let now_utc = Utc::now();

        let formatted = match params.timezone.as_deref() {
            None | Some("UTC") | Some("utc") => now_utc.to_rfc3339(),
            Some(tz_str) => match tz_str.parse::<Tz>() {
                Ok(tz) => now_utc.with_timezone(&tz).to_rfc3339(),
                Err(_) => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Unknown timezone: '{tz_str}'. Use an IANA name like 'Europe/Paris'."
                    ))]));
                }
            },
        };

        Ok(CallToolResult::success(vec![Content::text(formatted)]))
    }

    /// Evaluates a simple arithmetic expression and returns the result
    #[tool(description = "Evaluates a simple arithmetic expression — e.g. '2 + 3 * 4' or '(10 - 2) / 4'")]
    fn calculate(
        &self,
        Parameters(params): Parameters<CalculateParams>,
    ) -> Result<CallToolResult, McpError> {
        match meval::eval_str(&params.expression) {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(result.to_string())])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "Could not evaluate '{}': {}",
                params.expression, e
            ))])),
        }
    }
}

#[tool_handler]
impl ServerHandler for McpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "MCP Hello Tools — fondation MCP en Rust. \
                 Tools disponibles : echo, get_timestamp, calculate."
                    .to_string(),
            )
    }
}

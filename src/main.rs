mod server;
mod tools;

use rmcp::{ServiceExt, transport::stdio};
use server::McpServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = McpServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

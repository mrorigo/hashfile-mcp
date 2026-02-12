mod hashline;
mod tools;

use rmcp::{model::*, tool_handler, transport::stdio, ServerHandler, ServiceExt};

use crate::tools::HashfileServer;

#[tool_handler]
impl ServerHandler for HashfileServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Hashfile MCP Server - provides reliable file editing using hash-anchored operations.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = HashfileServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

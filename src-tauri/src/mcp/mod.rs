// MCP (Model Context Protocol) integration

pub mod client;
pub mod server;
pub mod tools;
pub mod types;

pub use server::McpServer;
pub use tools::{get_tool_definitions, ToolHandler};
pub use types::*;

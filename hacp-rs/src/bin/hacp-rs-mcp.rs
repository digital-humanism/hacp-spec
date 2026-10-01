//! hacp-rs-mcp — MCP transport adapter (JSON-RPC over stdio).
//!
//! Same evaluate as runner and proxy. Fail-closed.

use hacp_rs::mcp;

const TEST_PUBKEY_HEX: &str = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

fn main() {
    if let Err(e) = mcp::run(TEST_PUBKEY_HEX) {
        eprintln!("{{\"error\":\"MCP_ERROR\",\"message\":\"{}\"}}", e);
        std::process::exit(1);
    }
}

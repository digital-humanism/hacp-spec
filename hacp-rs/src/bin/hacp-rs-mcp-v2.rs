//! hacp-rs-mcp-v2 — MCP adapter v2 (Model B: deterministic transport synthesis).

use hacp_rs::mcp_v2;

const TEST_PUBKEY_HEX: &str = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

fn main() {
    if let Err(e) = mcp_v2::run_v2(TEST_PUBKEY_HEX) {
        eprintln!("{{\"error\":\"MCP_V2_ERROR\",\"message\":\"{}\"}}", e);
        std::process::exit(1);
    }
}

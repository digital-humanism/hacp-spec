//! hacp-rs-runner — Protocol v1 conformance runner binary.
//!
//! Usage: hacp-rs-runner
//! Reads JSON requests from stdin, writes JSON responses to stdout.

use hacp_rs::runner;

/// Embedded conformance test public key (harness/keys/KEYS.md). TEST ONLY.
const TEST_PUBKEY_HEX: &str = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

fn main() {
    if let Err(e) = runner::run(TEST_PUBKEY_HEX) {
        eprintln!("{{\"error\":\"RUNNER_ERROR\",\"message\":\"{}\"}}", e);
        std::process::exit(1);
    }
}

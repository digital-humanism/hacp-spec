//! hacp-rs-controlplane — Gate E control plane (JSON-RPC over stdio).
//!
//! In-memory revocation journal. No tonic/protoc required.

use hacp_rs::controlplane::{journal::Journal, server};
use std::sync::Arc;

fn main() {
    let journal = Arc::new(Journal::new());

    if let Err(e) = server::run(journal) {
        eprintln!("{{\"error\":\"CONTROL_ERROR\",\"message\":\"{}\"}}", e);
        std::process::exit(1);
    }
}

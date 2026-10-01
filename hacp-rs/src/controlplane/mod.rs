//! Gate E — distributed control plane.
//!
//! Components:
//! - `ControlState` — freshness tracker
//! - `Journal` — in-memory revocation journal
//! - `RevocationStore` — local revocation state with evaluator-context projection helpers
//! - JSON-RPC stdio server for control plane operations
//! - gRPC server (tonic) — wire-compat with Go sidecar proto

pub mod adapter;
pub mod grpc;
pub mod journal;
pub mod server;
pub mod state;

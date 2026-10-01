pub mod controlplane;
pub mod ed25519;
pub mod ed25519_v2;
pub mod evaluate;
pub mod sha256;
pub mod jcs;
pub mod jcs_v2;
pub mod jcs_v2_r2;
pub mod json_ingress_v2;
pub mod mcp;
pub mod mcp_v2;
pub mod proxy;
pub mod runner;

// Proto module at crate root for tonic::include_proto!
#[allow(clippy::result_large_err)]
pub mod proto {
    tonic::include_proto!("hacp.control.v1");
}

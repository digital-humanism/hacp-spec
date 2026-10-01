//! hacp-rs-grpc-server — gRPC ControlPlane server (wire-compat with Go proto).

use hacp_rs::controlplane::{grpc::ControlPlaneGrpc, journal::Journal};
use hacp_rs::proto::control_plane_server;
use std::sync::Arc;
use std::time::Duration;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port: u16 = std::env::var("HACP_GRPC_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(9090);

    let addr = format!("127.0.0.1:{}", port).parse()?;
    let journal = Arc::new(Journal::new());

    // Seed with test revocations for demonstration
    journal.revoke(
        hacp_rs::controlplane::journal::RevocationKind::Key,
        "test-revoked-key-001",
    )?;

    let service = ControlPlaneGrpc::new(journal, Duration::from_secs(5));

    eprintln!("hacp-rs-grpc-server listening on {}", addr);

    Server::builder()
        .add_service(control_plane_server::ControlPlaneServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

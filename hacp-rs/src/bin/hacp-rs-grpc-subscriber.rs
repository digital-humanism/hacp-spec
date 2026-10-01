//! hacp-rs-grpc-subscriber — gRPC subscriber that connects to a ControlPlane server.
//!
//! Usage: hacp-rs-grpc-subscriber [server_addr]
//! Default: http://127.0.0.1:9090

use hacp_rs::proto::control_plane_client::ControlPlaneClient;
use hacp_rs::proto::{
    watch_revocations_response, GetRevocationSnapshotRequest, WatchRevocationsRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://127.0.0.1:9090".to_string());

    eprintln!("Connecting to {}", addr);
    let mut client = ControlPlaneClient::connect(addr.clone()).await?;

    // 1. Get initial snapshot
    eprintln!("Getting initial snapshot...");
    let snapshot = client
        .get_revocation_snapshot(GetRevocationSnapshotRequest {
            sidecar_id: "hacp-rs-subscriber".to_string(),
        })
        .await?;

    let snap = snapshot.into_inner();
    eprintln!(
        "Snapshot: revision={}, entries={}",
        snap.revision,
        snap.entries.len()
    );
    for entry in &snap.entries {
        eprintln!("  kind={} subject_id={}", entry.kind, entry.subject_id);
    }

    // 2. Watch for revocations
    eprintln!(
        "Watching for revocations after revision={}...",
        snap.revision
    );
    let mut stream = client
        .watch_revocations(WatchRevocationsRequest {
            sidecar_id: "hacp-rs-subscriber".to_string(),
            after_revision: snap.revision,
        })
        .await?
        .into_inner();

    let mut last_revision = snap.revision;
    let mut event_count = 0u64;

    while let Some(response) = stream.message().await? {
        match response.payload {
            Some(watch_revocations_response::Payload::Event(event)) => {
                event_count += 1;
                last_revision = event.revision;
                eprintln!(
                    "Event: revision={} kind={} subject_id={}",
                    event.revision, event.kind, event.subject_id
                );
                println!(
                    "{}",
                    serde_json::json!({
                        "type": "event",
                        "revision": event.revision,
                        "kind": event.kind,
                        "subject_id": event.subject_id,
                    })
                );
            }
            Some(watch_revocations_response::Payload::Heartbeat(hb)) => {
                eprintln!(
                    "Heartbeat: current_revision={} server_time={}",
                    hb.current_revision, hb.server_time_ms
                );
                println!(
                    "{}",
                    serde_json::json!({
                        "type": "heartbeat",
                        "current_revision": hb.current_revision,
                    })
                );
            }
            Some(watch_revocations_response::Payload::ResetRequired(reset)) => {
                eprintln!(
                    "ResetRequired: oldest_available={} current={} reason={}",
                    reset.oldest_available_revision, reset.current_revision, reset.reason
                );
                println!(
                    "{}",
                    serde_json::json!({
                        "type": "reset_required",
                        "oldest_available_revision": reset.oldest_available_revision,
                        "current_revision": reset.current_revision,
                    })
                );
                // In a real subscriber, we would call GetRevocationSnapshot here
                break;
            }
            None => {
                eprintln!("Empty payload");
            }
        }
    }

    eprintln!(
        "Stream ended. Events received: {}, last_revision: {}",
        event_count, last_revision
    );

    // Output summary
    println!(
        "{}",
        serde_json::json!({
            "type": "summary",
            "events_received": event_count,
            "last_revision": last_revision,
        })
    );

    Ok(())
}

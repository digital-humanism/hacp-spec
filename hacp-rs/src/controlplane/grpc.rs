//! gRPC server for HACP ControlPlane — same proto as Go sidecar.
//!
//! Uses tonic + prost, proto from hacp-spec/proto/hacp/control/v1/.

use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio_stream::{wrappers::ReceiverStream, Stream};
use tonic::{Request, Response, Status};

use crate::controlplane::journal::Journal;
use crate::proto::control_plane_server;
use crate::proto::*;

pub struct ControlPlaneGrpc {
    journal: Arc<Journal>,
    heartbeat_interval: Duration,
}

impl ControlPlaneGrpc {
    pub fn new(journal: Arc<Journal>, heartbeat_interval: Duration) -> Self {
        Self {
            journal,
            heartbeat_interval,
        }
    }
}

#[tonic::async_trait]
impl control_plane_server::ControlPlane for ControlPlaneGrpc {
    async fn get_revocation_snapshot(
        &self,
        _request: Request<GetRevocationSnapshotRequest>,
    ) -> Result<Response<RevocationSnapshot>, Status> {
        let snap = self.journal.snapshot();
        Ok(Response::new(RevocationSnapshot {
            revision: snap.revision,
            entries: snap
                .entries
                .into_iter()
                .map(|e| RevocationEntry {
                    kind: e.kind,
                    subject_id: e.subject_id,
                })
                .collect(),
            generated_at_ms: snap.generated_at_ms,
        }))
    }

    type WatchRevocationsStream =
        Pin<Box<dyn Stream<Item = Result<WatchRevocationsResponse, Status>> + Send>>;

    async fn watch_revocations(
        &self,
        request: Request<WatchRevocationsRequest>,
    ) -> Result<Response<Self::WatchRevocationsStream>, Status> {
        let req = request.into_inner();
        let journal = self.journal.clone();
        let heartbeat_interval = self.heartbeat_interval;
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        tokio::spawn(async move {
            let mut last_revision = req.after_revision;
            let mut sequence: u64 = 0;

            loop {
                match journal.events_after(last_revision) {
                    Ok(events) => {
                        if events.is_empty() {
                            tokio::select! {
                                _ = tokio::time::sleep(heartbeat_interval) => {
                                    let current = journal.revision();
                                    if current == last_revision && heartbeat_interval.as_millis() > 0 {
                                        sequence += 1;
                                        let resp = WatchRevocationsResponse {
                                            sequence,
                                            payload: Some(
                                                watch_revocations_response::Payload::Heartbeat(
                                                    Heartbeat {
                                                        current_revision: last_revision,
                                                        server_time_ms: chrono_ms(),
                                                    },
                                                ),
                                            ),
                                        };
                                        if tx.send(Ok(resp)).await.is_err() { return; }
                                    }
                                }
                            }
                        } else {
                            for event in &events {
                                sequence += 1;
                                let resp = WatchRevocationsResponse {
                                    sequence,
                                    payload: Some(watch_revocations_response::Payload::Event(
                                        RevocationEvent {
                                            revision: event.revision,
                                            event_id: event.event_id.clone(),
                                            kind: event.kind,
                                            subject_id: event.subject_id.clone(),
                                            issued_at_ms: event.issued_at_ms,
                                        },
                                    )),
                                };
                                if tx.send(Ok(resp)).await.is_err() {
                                    return;
                                }
                                last_revision = event.revision;
                            }
                        }
                    }
                    Err(e) => {
                        if e.contains("replay unavailable") {
                            sequence += 1;
                            let resp = WatchRevocationsResponse {
                                sequence,
                                payload: Some(watch_revocations_response::Payload::ResetRequired(
                                    ResetRequired {
                                        oldest_available_revision: journal
                                            .oldest_available_revision(),
                                        current_revision: journal.revision(),
                                        reason: "requested revision is outside replay retention"
                                            .to_string(),
                                    },
                                )),
                            };
                            let _ = tx.send(Ok(resp)).await;
                            return;
                        }
                        let _ = tx.send(Err(Status::internal(e))).await;
                        return;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}

fn chrono_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

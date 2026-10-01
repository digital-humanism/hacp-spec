//! hacp-rs-proxy — HTTP_PROXY transport adapter.
//!
//! One evaluate, transport only. Fail-closed.
//! Headers: X-HACP-Intent-Envelope, X-HACP-Decision-Token (base64url).

use hacp_rs::proxy;

use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::TcpListener;

const TEST_PUBKEY_HEX: &str = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";

async fn handle(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("");
    let request_target = if query.is_empty() {
        path.clone()
    } else {
        format!("{}?{}", path, query)
    };

    // Collect headers
    let headers: Vec<(String, String)> = req
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    // Read body
    let body = match http_body_util::BodyExt::collect(req.into_body()).await {
        Ok(c) => c.to_bytes().to_vec(),
        Err(_) => {
            let body = proxy::deny_response_body("INVALID_ACTION", "err");
            return Ok(Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("X-HACP-Decision", "DENY")
                .header("X-HACP-Reason", "INVALID_ACTION")
                .body(Full::new(Bytes::from(body)))
                .unwrap());
        }
    };

    let clock = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    // Call evaluate (SAME function as runner)
    let (decision, reason_codes, _action_hash) = proxy::proxy_evaluate(
        &headers,
        &method,
        &request_target,
        &body,
        TEST_PUBKEY_HEX,
        clock,
    );

    let request_id = "proxy-001";

    if decision == "ALLOW" {
        let body = proxy::allow_response_body(request_id);
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("X-HACP-Decision", "ALLOW")
            .header("X-HACP-Request-Id", request_id)
            .body(Full::new(Bytes::from(body)))
            .unwrap())
    } else {
        let reason = reason_codes.first().map(|s| s.as_str()).unwrap_or("DENIED");
        let body = proxy::deny_response_body(reason, request_id);
        Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .header("X-HACP-Decision", "DENY")
            .header("X-HACP-Reason", reason)
            .header("X-HACP-Request-Id", request_id)
            .body(Full::new(Bytes::from(body)))
            .unwrap())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port: u16 = env::var("HACP_SIDECAR_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;
    eprintln!("hacp-rs-proxy listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
            {
                eprintln!("proxy connection error: {}", err);
            }
        });
    }
}

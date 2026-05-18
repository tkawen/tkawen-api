//! tkawen-api — Unified API gateway for TKAWEN Sovereign Cloud.
//!
//! Status: alpha scaffold. Routes accept requests under /v1/<pillar>/*,
//! validate auth shape, and return 503 with explicit JSON for pillar
//! routes that don't have a wired upstream yet. /v1/health and /v1/usage
//! return real (mock) responses.
//!
//! AGPL-3.0-or-later.

use axum::{
    body::Body,
    extract::{Path, Request},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{any, get},
    Json, Router,
};
use serde::Serialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_BANNER: &str = concat!("tkawen-api/", env!("CARGO_PKG_VERSION"), " (axum)");

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root_landing))
        .route("/v1/health", get(health))
        .route("/v1/usage", get(usage_mock).layer(middleware::from_fn(require_auth)))
        .route("/v1/keys", get(keys_list_mock).post(keys_create_mock).layer(middleware::from_fn(require_auth)))
        // 7 pillar wildcard routes — all return 503 with honest message until
        // upstream services are wired.
        .route("/v1/identity/*path", any(pillar_503))
        .route("/v1/connect/*path", any(pillar_503))
        .route("/v1/pay/*path", any(pillar_503))
        .route("/v1/commerce/*path", any(pillar_503))
        .route("/v1/knowledge/*path", any(pillar_503))
        .route("/v1/logistics/*path", any(pillar_503))
        .fallback(not_found)
        .layer(cors)
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(SetResponseHeaderLayer::overriding(
            header::SERVER,
            HeaderValue::from_static(SERVER_BANNER),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-powered-by"),
            HeaderValue::from_static("Rust + Axum (AGPL-3.0)"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = std::env::var("TKAWEN_API_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:9099".to_string())
        .parse()
        .expect("TKAWEN_API_ADDR must be a valid SocketAddr");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    tracing::info!("tkawen-api v{} listening on http://{}", VERSION, addr);
    tracing::info!("alpha scaffold: pillar routes return 503 until upstreams wired");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}

// ─────────────────────────────────────────────────────────────────────
// AUTH MIDDLEWARE — skeleton only
// ─────────────────────────────────────────────────────────────────────

/// Validates the shape of the Authorization header. Does NOT verify
/// against a real key store yet. Real implementation will:
///   1. Look up key in Postgres (sk_live_... or sk_sandbox_...)
///   2. Check it hasn't been revoked
///   3. Check the requested pillar is in the key's scope
///   4. Rate-limit via Redis (per key, per pillar, per minute)
///   5. Log usage event for billing
async fn require_auth(req: Request, next: Next) -> Result<Response, Response> {
    let auth = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    if !auth.starts_with("Bearer ") {
        return Err(error_response(
            StatusCode::UNAUTHORIZED,
            "missing_authorization",
            "Authorization header required: 'Bearer sk_...'",
        ));
    }

    let token = &auth[7..];
    if !is_valid_token_shape(token) {
        return Err(error_response(
            StatusCode::UNAUTHORIZED,
            "invalid_token_shape",
            "Token must start with sk_live_ or sk_sandbox_ followed by 24+ chars.",
        ));
    }

    // TODO: real verification against Postgres-backed key store
    Ok(next.run(req).await)
}

fn is_valid_token_shape(token: &str) -> bool {
    (token.starts_with("sk_live_") || token.starts_with("sk_sandbox_"))
        && token.len() >= 32
        && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

// ─────────────────────────────────────────────────────────────────────
// ROUTES
// ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    gateway: &'static str,
    upstream_status: serde_json::Value,
}

async fn root_landing() -> impl IntoResponse {
    let html = format!(r##"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><title>api.tkawen.com</title>
<style>body{{font-family:system-ui,sans-serif;background:#0a0e1a;color:#e2e8f0;padding:60px 24px;max-width:680px;margin:0 auto;line-height:1.7}}
h1{{color:#fff;letter-spacing:-.02em}}code{{background:#151b2e;padding:2px 8px;border-radius:4px;color:#fbbf24;font-family:ui-monospace,monospace}}
a{{color:#3b82f6}}.banner{{background:rgba(245,158,11,.1);border:1px solid rgba(245,158,11,.3);padding:14px 18px;border-radius:8px;margin-bottom:24px;font-size:14px}}</style></head>
<body>
<div class="banner"><strong>alpha</strong> — this gateway is a scaffold. Pillar routes return <code>503</code> until backends wire up.</div>
<h1>TKAWEN API Gateway</h1>
<p>The unified API for TKAWEN Sovereign Cloud. All 7 pillars (Identity, Connect, Pay, Commerce, Knowledge, Logistics, Developer) accessible under <code>/v1/&lt;pillar&gt;/*</code>.</p>
<p>Documentation: <a href="https://developer.tkawen.com">developer.tkawen.com</a></p>
<p>Source: <a href="https://github.com/hartemyaakoub/tkawen-api">github.com/hartemyaakoub/tkawen-api</a></p>
<p>Status: <a href="https://status.tkawen.com">status.tkawen.com</a></p>
<p><strong>Version</strong>: <code>{}</code></p>
</body></html>"##, VERSION);

    ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], html)
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: VERSION,
        gateway: "tkawen-api",
        upstream_status: json!({
            "identity": "scaffold",
            "connect": "scaffold",
            "pay": "scaffold",
            "commerce": "scaffold",
            "knowledge": "scaffold",
            "logistics": "scaffold"
        }),
    })
}

async fn usage_mock() -> Json<serde_json::Value> {
    Json(json!({
        "period": "2026-05",
        "by_pillar": {
            "identity":  { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" },
            "connect":   { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" },
            "pay":       { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" },
            "commerce":  { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" },
            "knowledge": { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" },
            "logistics": { "calls": 0, "cost_dzd": 0.0, "status": "scaffold" }
        },
        "total_dzd": 0.0,
        "plan": "sandbox",
        "next_invoice_date": "2026-06-01",
        "note": "Usage tracking is not yet wired to a real billing system. This response is mock data for client SDK integration testing."
    }))
}

async fn keys_list_mock() -> Json<serde_json::Value> {
    Json(json!({
        "data": [],
        "note": "Key store is not yet implemented. Real implementation will return your sk_live_*/sk_sandbox_* keys here."
    }))
}

async fn keys_create_mock() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "error": "key_creation_not_implemented",
            "message": "Key creation requires the Postgres-backed key store. Coming in Phase 1 of the roadmap (see github.com/hartemyaakoub/tkawen-api#roadmap)."
        })),
    )
}

/// Returns 503 with honest JSON for any pillar route. This is the
/// scaffold behaviour — real implementation will reverse-proxy to the
/// appropriate upstream and stream the response back.
async fn pillar_503(req: Request) -> (StatusCode, Json<serde_json::Value>) {
    let path = req.uri().path();
    // /v1/<pillar>/<rest> → extract pillar
    let pillar = path
        .strip_prefix("/v1/")
        .and_then(|s| s.split('/').next())
        .unwrap_or("unknown");

    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({
            "error": "upstream_not_yet_implemented",
            "pillar": pillar,
            "path": path,
            "method": req.method().as_str(),
            "message": "This gateway is in alpha. The upstream backend for this pillar has not been wired yet. See https://github.com/hartemyaakoub/tkawen-api#roadmap for the implementation plan.",
            "developer_docs": format!("https://developer.tkawen.com/pillars/{}/", pillar)
        })),
    )
}

async fn not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "not_found",
            "message": "Unknown route. Available routes are under /v1/<pillar>/*. See https://developer.tkawen.com/"
        })),
    )
}

fn error_response(status: StatusCode, code: &str, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            json!({ "error": code, "message": message }).to_string(),
        ))
        .unwrap()
}

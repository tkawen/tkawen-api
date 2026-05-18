<div align="center">

<img src="https://raw.githubusercontent.com/tkawen/tkawen-com/main/assets/og.png" width="640" alt="TKAWEN — Seven APIs. One Platform." />

# tkawen-api

**Unified API gateway for TKAWEN — one Bearer key, seven pillars, OpenAPI 3.1.**

[![ci](https://github.com/tkawen/tkawen-api/actions/workflows/ci.yml/badge.svg)](https://github.com/tkawen/tkawen-api/actions/workflows/ci.yml)
[![Status: alpha](https://img.shields.io/badge/status-alpha-f59e0b)](https://github.com/tkawen/tkawen-api)
[![License: AGPL-3.0](https://img.shields.io/badge/license-AGPL--3.0-blue.svg)](LICENSE)
[![Discord](https://img.shields.io/badge/community-discord-5865f2)](https://discord.gg/tkawen)

</div>

---

> **Status: alpha scaffold — not production-functional yet.** Read [Roadmap](#roadmap-to-production-functional) before integrating.

## What this is

The planned **unified API gateway** for TKAWEN Sovereign Cloud — the `api.tkawen.com` endpoint that fronts seven cloud APIs (Identity, Connect, Pay, Commerce, Knowledge, Logistics, Developer) behind one authentication scheme.

```
api.tkawen.com/v1/identity/*   →  Authentik
api.tkawen.com/v1/connect/*    →  LIQAA Cloud
api.tkawen.com/v1/pay/*        →  TKAWEN Pay
api.tkawen.com/v1/commerce/*   →  MyStoq backend
api.tkawen.com/v1/knowledge/*  →  Certify backend
api.tkawen.com/v1/logistics/*  →  Traccar + carriers
api.tkawen.com/v1/usage        →  this service (billing layer)
```

## What works today

This scaffold deliberately ships **honest** behaviour so SDK developers can integrate against it without waiting for full backends:

| Endpoint | Behaviour |
|----------|-----------|
| `GET /v1/health` | Real JSON listing all 7 pillars as `scaffold` |
| `GET /v1/usage` (auth) | Mock usage data for SDK integration testing |
| `GET /v1/keys` (auth) | Empty list placeholder |
| `POST /v1/keys` (auth) | `503` with implementation roadmap pointer |
| `* /v1/<pillar>/*` | `503` with JSON pointing to developer docs — explicit, never silent |
| `GET /` | Small HTML landing explaining alpha status |
| `GET /healthz` | `ok` for load balancers |

Auth middleware validates the **shape** of the Bearer token (`sk_live_*` or `sk_sandbox_*`, 32+ chars) but does not yet verify against a real key store.

## Quick start

```bash
git clone https://github.com/tkawen/tkawen-api.git
cd tkawen-api
cargo build --release
./target/release/tkawen-api
# → http://127.0.0.1:9099

curl http://127.0.0.1:9099/v1/health
# {"status":"ok","version":"0.1.0-alpha","gateway":"tkawen-api","upstream_status":{...}}

curl -H "Authorization: Bearer sk_sandbox_test_xxxxxxxxxxxxxxxx" \
     http://127.0.0.1:9099/v1/connect/rooms
# 503 Service Unavailable
# {"error":"upstream_not_yet_implemented","pillar":"connect", ...}
```

## Stack

| Layer | Choice |
|-------|--------|
| HTTP server | [Axum](https://github.com/tokio-rs/axum) 0.7 |
| HTTP client (upstreams) | [reqwest](https://github.com/seanmonstar/reqwest) 0.12 (rustls) |
| Middleware | tower + tower-http (compression, CORS, security headers) |
| Auth verification | HMAC-SHA256 for webhook sig; future: JWKs against Authentik |
| Runtime | Tokio multi-threaded |

## Roadmap to production-functional

### Phase 1 — Real auth (week 1-2)
- Replace skeleton auth with Authentik OIDC token verification (JWKs)
- API key model in Postgres — issue, rotate, revoke, scope per pillar
- Redis-backed rate limiting (per key, per pillar)

### Phase 2 — First real pillar (week 3-4)
- Wire `/v1/connect/*` to LIQAA Cloud with token translation
- Request/response logging to Postgres
- Integration tests against real LIQAA backend

### Phase 3 — Remaining pillars (week 5-8)
- Identity, Pay, Commerce, Knowledge, Logistics
- Each requires the upstream backend to expose a stable `/v1/...` surface

### Phase 4 — Billing + observability (week 9-10)
- Real-time usage tracking with billing recompute every 10 minutes
- Stripe-style "next invoice" endpoint
- Tie into payment processor for actual charging

### Phase 5 — OpenAPI + SDK regeneration (week 11-12)
- OpenAPI 3.1 spec auto-generated from routes
- Trigger SDK rebuild for all 4 languages on spec change
- Publish at `api.tkawen.com/openapi.json` + `.yaml`

## Architecture

```
                ┌───────────────────────┐
                │    api.tkawen.com     │
                │       (TLS edge)      │
                └───────────┬───────────┘
                            │
                ┌───────────▼───────────┐
                │   tkawen-api (Rust)   │
                │  Auth · Rate limit ·  │
                │   Usage · Routing     │
                └───────────┬───────────┘
                            │
        ┌───────┬───────────┼───────────┬───────────┐
        ▼       ▼           ▼           ▼           ▼
   Authentik LIQAA       Chargily   MyStoq      Traccar
   (Identity) (Connect)  (Pay)      (Commerce)  (Logistics)
```

## Why open source?

Sovereign infrastructure means **inspectable infrastructure**. If a regulated buyer wants to fork this and run their own gateway, they should be able to.

Commercial value is in the **operated service** — the SLA, the data residency story, the support — not in the source being secret.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Especially welcome at this stage: code review on the auth middleware, OpenAPI spec contributions, test infrastructure.

## License

[AGPL-3.0-or-later](./LICENSE).

## Security

[SECURITY.md](./SECURITY.md) — please do not open public issues for security vulnerabilities.

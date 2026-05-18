# tkawen-api

**Status: alpha · scaffold only · not production-functional yet.**

This is the planned unified API gateway for TKAWEN Sovereign Cloud — the
`api.tkawen.com` endpoint that fronts all 7 pillars under one authentication
scheme.

## What this is

A **reverse proxy + auth layer** in Rust (Axum + reqwest) that routes
incoming requests under `/v1/<pillar>/*` to the appropriate backend:

```
api.tkawen.com/v1/identity/*   →  Authentik (id.tkawen.com)
api.tkawen.com/v1/connect/*    →  LIQAA Cloud (api.liqaa.io)
api.tkawen.com/v1/pay/*        →  TKAWEN Pay (Chargily-backed)
api.tkawen.com/v1/commerce/*   →  MyStoq backend
api.tkawen.com/v1/knowledge/*  →  Algeria Certify + Academy
api.tkawen.com/v1/logistics/*  →  Traccar + carrier integrations
api.tkawen.com/v1/usage        →  this service (billing layer)
```

## What this is NOT (yet)

- **Not a functional gateway.** The backends advertised at `*.tkawen.com`
  subdomains in the marketing site don't exist with `/v1/...` URLs yet.
  This gateway will need actual upstream services with stable API surfaces
  before it can route to them.
- **Not authenticated against TKAWEN ID.** The auth middleware is a
  skeleton — it validates the `Bearer` token format but does not verify
  against the real Authentik OIDC issuer.
- **Not rate-limited.** Real implementation needs Redis-backed counters.
- **Not billed.** Usage tracking writes to stdout, not to a billing DB.
- **No OpenAPI spec yet.** Will be added once routes are stable.

## What this IS today

A working Rust server that:

1. Boots on `127.0.0.1:9099` (or `$TKAWEN_API_ADDR`)
2. Accepts requests under `/v1/*`
3. Validates the `Authorization: Bearer ...` header format
4. Returns `503 Service Unavailable` with a JSON `{ "error": "upstream
   not yet implemented" }` for any pillar route — explicit and honest
5. Returns `200 { "status": "ok" }` for `/v1/health`
6. Returns mock `/v1/usage` JSON for testing client SDKs against

This is enough to:
- **Develop SDKs in parallel** (JS/PHP/Python/Go can integrate against
  this skeleton — they'll get 503 but their integration code is ready)
- **Write integration tests** that verify routing + auth shape
- **Document the planned API surface** (OpenAPI spec lives here)
- **Deploy a "coming soon" landing** at api.tkawen.com that responds
  honestly instead of 404

## Quick start

```bash
# Set up PATH for LLVM-MinGW + cargo
$env:Path = "D:\F\.toolchain\llvm-mingw-20251007-ucrt-x86_64\bin;$env:USERPROFILE\.cargo\bin;$env:Path"

cd D:\F\tkawen-api
cargo build --release
.\target\release\tkawen-api.exe
# → http://127.0.0.1:9099

curl http://127.0.0.1:9099/v1/health
# → {"status":"ok","version":"0.1.0-alpha","gateway":"tkawen-api"}

curl -H "Authorization: Bearer sk_sandbox_test" \
     http://127.0.0.1:9099/v1/connect/rooms
# → 503 Service Unavailable
# {"error":"upstream not yet implemented","pillar":"connect","path":"/rooms"}
```

## Roadmap to production-functional

### Phase 1 — Real auth (week 1-2)
- Replace skeleton auth with Authentik OIDC token verification (JWKs)
- Add API key model (Postgres) — issue, rotate, revoke, scope per pillar
- Add Redis-backed rate limiting (per key, per pillar)

### Phase 2 — First real pillar wired (week 3-4)
- Wire `/v1/connect/*` to api.liqaa.io with token translation
- Add request/response logging to Postgres
- Write integration tests using real LIQAA backend

### Phase 3 — Remaining pillars (week 5-8)
- Identity, Pay, Commerce, Knowledge, Logistics
- Each requires backend to expose stable `/v1/...` surface
- May require breaking changes in existing platforms (MyStoq, Certify)

### Phase 4 — Billing + observability (week 9-10)
- Real-time usage tracking with billing recompute every 10 minutes
- Stripe-style "next invoice" endpoint
- Tie into Chargily for actual charging

### Phase 5 — OpenAPI + SDK regeneration (week 11-12)
- OpenAPI 3.1 spec auto-generated from routes
- Trigger SDK rebuild for all 4 languages on spec change
- Publish at api.tkawen.com/openapi.json + openapi.yaml

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  api.tkawen.com (TLS)                   │
│                          │                              │
│              ┌───────────┴───────────┐                  │
│              │    tkawen-api (Rust)  │                  │
│              │  ──────────────────   │                  │
│              │  Auth middleware       │                  │
│              │  Rate limiter         │                  │
│              │  Usage tracker        │                  │
│              │  Reverse proxy router │                  │
│              └───────────┬───────────┘                  │
│              ┌───────────┼───────────┐                  │
│              │           │           │                  │
│      identity.tkawen     │     connect.tkawen           │
│      (Authentik)         │     (LIQAA Cloud — Go)       │
│                          │                              │
│      pay.tkawen          │     commerce.tkawen          │
│      (Chargily proxy)    │     (MyStoq — Laravel)       │
│                          │                              │
│      knowledge.tkawen    │     logistics.tkawen         │
│      (Certify — Laravel) │     (Traccar — Java)         │
└─────────────────────────────────────────────────────────┘
```

## License

AGPL-3.0-or-later. See LICENSE.

## Why open source?

Because sovereign cloud means **inspectable cloud**. If anyone wants to
fork this and run their own Algerian API gateway, they should be able to.
The commercial value is in the operated service, the data residency, the
support — not in the source being secret.

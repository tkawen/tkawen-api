# Contributing to tkawen-api

`tkawen-api` is the **unified API gateway** for TKAWEN — the planned `api.tkawen.com` endpoint that routes `/v1/<pillar>/*` requests to the appropriate backend services.

## Status: alpha scaffold

This repository is **not yet production-functional**. Pillar routes deliberately return `503 Service Unavailable` with honest JSON pointing to the developer docs, so SDK integration work can proceed in parallel.

See [README.md](./README.md) for the 5-phase roadmap.

## Quick start

```bash
git clone https://github.com/tkawen/tkawen-api.git
cd tkawen-api
cargo build --release
./target/release/tkawen-api
# → http://127.0.0.1:9099

curl http://127.0.0.1:9099/v1/health
# {"status":"ok","version":"0.1.0-alpha","gateway":"tkawen-api"}
```

## What kind of contributions are welcome

**Now (alpha phase):**

- Code review on the auth middleware skeleton (`require_auth` in `src/main.rs`)
- Discussion of the routing pattern in [GitHub Discussions](https://github.com/tkawen/tkawen-api/discussions)
- OpenAPI 3.1 spec contributions (the planned single source of truth)
- Test infrastructure (currently zero tests — bring your own framework opinions)

**Later (post-alpha):**

- Real upstream backend integrations (pillar-by-pillar)
- Rate limiting (Redis-backed)
- Usage tracking + billing pipeline
- SDK generators that consume the OpenAPI spec
- Metrics + tracing (OpenTelemetry)

## How to contribute

1. Open an issue first for non-trivial changes — direction matters more than code at this stage
2. Fork the repo and create a feature branch off `main`
3. Run `cargo fmt`, `cargo clippy --all-targets`, `cargo check`
4. Add a smoke test if you touch routing
5. Open a PR using the template

## Project structure

```
src/
└── main.rs              # Axum router + auth middleware + pillar handlers (all in one file for alpha)
```

Future expansion:

```
src/
├── main.rs
├── auth.rs              # Real key store + Authentik integration
├── proxy.rs             # Reverse proxy to upstream pillars
├── rate_limit.rs        # Redis-backed limiter
├── usage.rs             # Usage tracking + billing events
└── pillars/
    ├── identity.rs
    ├── connect.rs
    └── ...
```

## Code of Conduct

This project adheres to a [Contributor Covenant Code of Conduct](./CODE_OF_CONDUCT.md).

## Security

Do **not** open public issues for security vulnerabilities. See [SECURITY.md](./SECURITY.md).

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0-or-later license. See [LICENSE](./LICENSE).

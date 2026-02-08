# Repository Guidelines

## Project Structure & Module Organization
Intern-trial is an Axum microservice that proxies account lookups against a Mosaic Chain testnet. `src/main.rs` wires the router, tracing, and shared `AccountService` state. HTTP handlers plus OpenAPI glue live in `src/api`, DTOs in `src/dto`, and blockchain access code under `src/services`. Polkadot metadata for Subxt is committed as `metadata.scale`; refresh it whenever the target runtime changes. Build artifacts and scratch space stay in `target/` and should never be committed.

## Build, Test, and Development Commands
- `cargo run` — start the server on `0.0.0.0:4500`; ping it with `curl localhost:4500/accounts/<address>`.
- `cargo check` — fast validation during feature work; run before staging files.
- `cargo fmt` — applies the repository style; required pre-commit.
- `cargo clippy -- -D warnings` — enforces lint clean builds, especially around async error handling.
- `cargo test` — executes unit/integration suites; keep it green even if only smoke tests exist.

## Coding Style & Naming Conventions
Honor rustfmt defaults (4-space indent, trailing commas, grouped imports). Modules/files stay snake_case, types camel case, trait names describe capabilities (`DtoConvertible`). Prefer explicit `use` statements per module, limit handler functions to one responsibility, and document tricky logic with `///` comments. Run `fmt` and `clippy` before sending code for review.

## Testing Guidelines
Place unit tests next to the code inside `#[cfg(test)]` modules, using `#[tokio::test]` for async contexts. Mock `AccountService` behind an `Arc<Mutex<_>>` to avoid hitting the real node unless doing tagged integration tests. Name tests `function_under_test_condition_expected` for readable failures. Cover DTO conversions and error branches (invalid address, RPC failure) before adding new endpoints.

## Commit & Pull Request Guidelines
Git history favors short present-tense summaries (`Started reworking the code...`). Follow the same style, aim for one logical change per commit, and mention why when the diff is non-obvious. PRs should describe behavior changes, list validation (`cargo test`, sample `curl` output), and link issues or tickets. Include screenshots/log snippets when adjusting Swagger or HTTP responses, and call out follow-up work if network dependencies changed.

## Security & Configuration Notes
The websocket endpoint string in `AccountService` is the only remote secret; override it via env/config when testing other chains and avoid committing experimental URLs. Treat `metadata.scale` as part of the contract—regenerate with `subxt metadata --url <endpoint> > metadata.scale`, review the diff, and note the source network in your PR description.

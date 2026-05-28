# Investment Intelligence Platform (IIP)

Rust workspace for the Investment Intelligence Platform MVP, with:

- `crates/api`: Axum API serving map and heatmap endpoints
- `crates/webapp`: Rust + WebAssembly frontend
- `crates/domain`: shared domain model types
- `crates/dataset-sim`: synthetic dataset generator

## Prerequisites

1. Install the WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Install Trunk once:
   ```bash
   cargo install trunk
   ```

## Developer workflow (host + WASM parity)

Always validate both targets:

```bash
# Host compilation and tests
cargo check --workspace
cargo test --workspace

# Browser target compilation
cargo check -p webapp --target wasm32-unknown-unknown
```

Equivalent cargo aliases:

```bash
cargo all-check
cargo all-test
cargo webapp-check
```

## Run locally

Run the API in one terminal:

```bash
cargo api
```

Run the web frontend in another terminal:

```bash
cd crates/webapp
trunk serve --address 127.0.0.1 --port 8082 --open
```

The webapp calls the API at `http://127.0.0.1:3000`.

## Copilot execution contract

For implementation tasks, Copilot should handle the runtime loop end-to-end:

1. kill stale listeners on app ports (`3000` API, `8082` webapp),
2. rebuild/restart services,
3. verify both compile and serve before declaring completion.

Verification commands:

```bash
cargo check --workspace
cargo check -p webapp --target wasm32-unknown-unknown
curl -s http://127.0.0.1:3000/heatmap | head -c 200
curl -sI http://127.0.0.1:8082 | head -n 1
```

## Python-free setup constraint

Project setup and run workflow are intentionally Rust/Cargo/Trunk/shell only, with no Python dependency.

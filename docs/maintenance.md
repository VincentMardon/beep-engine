# Maintenance and resuming development

The project is intentionally on hold after satisfying its first concrete need. Start future work by reading the [architecture and decisions](architecture.md), the [API guide](api.md), and `AGENTS.md`. The local French learning roadmap is in `notes/roadmap.md`; `notes/` is not versioned.

## Tools

- Rust and Cargo, installed through rustup
- rustfmt and Clippy
- The `wasm32-unknown-unknown` compilation target
- `wasm-bindgen-cli` matching the exact `wasm-bindgen` dependency in `Cargo.toml` (currently 0.2.129)
- Node.js and npm for local package creation
- Python for the optional standalone browser demo server

For a fresh environment:

```sh
rustup component add rustfmt clippy
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked
```

Commands below run from the BeepEngine repository root unless stated otherwise.

## Native checks

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --all-targets
cargo test --locked --all-targets
cargo run --locked
```

The final command plays sound; the others do not. The native listening demo needs a working default audio output.

## Browser build and checks

```sh
cargo clippy --locked --lib --target wasm32-unknown-unknown -- -D warnings
cargo build --locked --lib --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir web/wasm target/wasm32-unknown-unknown/release/beep_engine.wasm
```

Use `--lib` for Wasm: the native executable uses Rodio. Do not hand-edit generated bindings. Rebuild them after changing the Rust generators or Wasm exports. If updating `wasm-bindgen`, also install the matching CLI version.

Serve the repository and open `http://localhost:8000/web/`:

```sh
python -m http.server 8000 --bind 127.0.0.1
```

Listen to each sound at both rates, including the first playback after reloading. The browser API should reject unsupported rates. These are manual checks; there is no automated browser test suite in this repository yet.

## Build and distribute a package

First rebuild the browser files using the commands above, then:

```sh
npm pack --dry-run
npm pack --pack-destination target
```

Check that the archive includes `web/beep-engine.js`, `web/beep-engine.d.ts`, and the Wasm bindings and binary under `web/wasm/`. It also includes the root README and package metadata. The current archive name is `beep-engine-0.1.0.tgz`.

For future distributed changes, choose a new package version so consumers can identify the artifact. Keep Rust and npm version metadata aligned when releasing the project together. Do not assume rebuilding a local archive updates an already installed consumer: copy the new artifact and install it there explicitly, updating its package lock.

PHW's Next.js application lives in `C:/dev/professional-hello-world/professional-hello-world/frontend/`. Its local package is stored in `frontend/vendor/` and installed from that relative path so the artifact is available to CI. Consumer application changes and commits belong to PHW's own workflow.

BeepEngine's `docs/` guides are not currently included in the npm archive's explicit `files` list. The repository is the source of the full documentation; the root README provides package-level instructions.

## What is versioned

Keep Rust sources, browser interface sources, TypeScript declarations, package manifests, `Cargo.lock`, CI configuration, and Markdown documentation in Git.

Keep generated Rust outputs and npm archives under `target/`, and browser bindings under `web/wasm/`, ignored. Personal learning notes stay under the ignored `notes/` directory. Review `.gitignore` when introducing new tools or generated output.

## Restart from a concrete need

Identify which consumer behavior is missing before extending the API. Preserve the existing sounds unless a change is intentional. Re-run relevant tests and listening checks, update the documentation, and let the user apply code changes and make commits as required by `AGENTS.md`.

Potential future topics include audio-context lifecycle, playback completion, scheduling, more sounds, or game-engine use. None is an active implementation commitment.

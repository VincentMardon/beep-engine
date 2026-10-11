# BeepEngine

A small independent audio engine written in Rust, developed progressively as a learning project.

The first milestone is complete. Development is intentionally on hold until a concrete new need appears.

## Documentation

- [API guide](docs/api.md): Rust generation, browser playback, and initialization contracts.
- [Architecture and decisions](docs/architecture.md): responsibilities, sound design, and known boundaries.
- [Maintenance guide](docs/maintenance.md): setup, rebuilding, packaging, and resuming development.

## Current sounds

- Startup beep: a square wave at approximately 880 Hz, lasting 100 ms.
- Crash sound: a synthesized arcade-inspired noise burst, lasting 700 ms, with a fading amplitude.

Both sounds are generated as mono `f32` samples at either 44,100 or 48,000 Hz. Rodio handles playback through the default audio output. No audio files are required.

## Run the demo

Install Rust with rustup, then run from the repository root:

```sh
cargo run --locked
```

The demo queues 200 ms of silence, plays the startup beep, waits 500 ms, then plays the crash sound. A working default audio output is required.

The leading silence resolved a first-beep playback stutter observed locally at 44,100 Hz. It is a workaround in the demo, not part of either generated sound; the underlying cause has not been established.

Choose `SampleRate::Hz44100` or `SampleRate::Hz48000` in `src/main.rs`. The demo currently uses `Hz48000`. This selects the generated sample rate, not necessarily the physical device's output rate.

The listening sequence belongs to the demo. The sound generation functions are independent of Professional Hello World (PHW) and do not open an audio device.

## Project structure

- `src/lib.rs` contains the public `startup_beep(sample_rate)` and `crash_sound(sample_rate)` functions, along with their unit tests. Each function returns mono samples as a `Vec<f32>`.
- `src/main.rs` uses the library and Rodio to play the two sounds in sequence.

Both generation functions accept a `SampleRate` enum with two supported variants: `Hz44100` and `Hz48000`. Its `hz()` method returns the numeric rate. Unit tests cover durations, beep amplitude and polarity, and crash amplitude and final silence at both rates.

The crash noise updates 1,000 times per second at either rate to preserve its intended texture. The library generates samples without opening an audio device.

## Browser demo and local npm package

Install the WebAssembly target and the CLI version matching the pinned Rust dependency:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked
```

Build the browser bindings from the repository root:

```sh
cargo build --locked --lib --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir web/wasm target/wasm32-unknown-unknown/release/beep_engine.wasm
```

Serve the repository locally and open `http://localhost:8000/web/`:

```sh
python -m http.server 8000 --bind 127.0.0.1
```

The demo has separate buttons for both sounds and a choice of sample rates. Browser audio is activated through user interaction.

To inspect and create the local npm archive:

```sh
npm pack --dry-run
npm pack --pack-destination target
```

The archive is `target/beep-engine-0.1.0.tgz`. It includes the JavaScript playback interface, TypeScript declarations, and generated WebAssembly files. Generated files remain ignored by Git. Rebuild and repack after changes to the distributed code or documentation. The package is marked private and is not published to the npm registry.

Install the archive in a consuming project using its path:

```sh
npm install C:/dev/beep-engine/target/beep-engine-0.1.0.tgz
```

The package exports `createBeepEngine()`. After initialization, its returned object offers `startupBeep(sampleRate)` and `crashSound(sampleRate)`, with 48,000 Hz as the default. Both methods return promises that resolve after starting playback, not after the sound finishes.

Playback requires a browser. The user confirmed loading and playback in PHW's dedicated Next.js test page and locally served static export. PHW's main page remains unchanged; deployed GitHub Pages verification and main-page integration belong to PHW's separate workflow.

## Quality checks

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --all-targets
cargo test --locked --all-targets
```

GitHub Actions runs these checks on Windows for pushes and pull requests. Automated checks do not assess how the sounds feel; listening remains part of sound design.

## Scope

The initial objective is to generate and play two sounds while understanding the Rust code behind them. PHW is an initial consumer, with browser consumption validated on a separate test page. Integration into its main sequence and into a game engine remains outside this milestone.

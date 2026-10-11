# Architecture and decisions

BeepEngine's first milestone is complete. Development is intentionally on hold until a concrete consumer need appears. The current scope is two synthesized sounds, usable from Rust and from a browser, with no audio files.

## Responsibilities

| File | Responsibility |
| --- | --- |
| `src/lib.rs` | Sample-rate choices, sound generation, unit tests, and Wasm exports |
| `src/main.rs` | Native listening demo using Rodio |
| `web/beep-engine.js` | Browser initialization and playback using Web Audio |
| `web/beep-engine.d.ts` | TypeScript declarations for the playback interface |
| `web/index.html` | Standalone browser listening demo |
| `Cargo.toml` | Rust package, library outputs, and target-specific dependencies |
| `package.json` | Local npm package entry point and included files |
| `.github/workflows/ci.yml` | Native Rust quality checks on Windows |

The library produces mono `Vec<f32>` samples without opening an audio device. Playback and consumer-specific timing belong outside the generators. PHW is an initial consumer, not an architectural dependency.

## Sound design

The startup beep is a square wave at approximately 880 Hz, lasting 100 ms, with amplitude 0.2. A floating-point phase advances through the cycle so that a period does not need to occupy an integer number of samples.

The crash is a 700 ms arcade-inspired noise burst. A deterministic pseudo-random generator chooses between positive and negative values. An integer clock updates the noise 1,000 times per second at either supported sample rate. A squared descending envelope reduces the amplitude to zero.

A descending-pitch crash was explored and replaced by this noise burst after listening. The startup beep remains a separate sound. The crash is an original interpretation of an arcade explosion, not a verified reproduction of Space Invaders hardware.

`SampleRate` restricts the Rust API to 44,100 and 48,000 Hz. Both choices preserve the durations and intended noise update rate. Browser numbers are validated before conversion to this enum.

## Native and browser paths

The native demo uses Rodio. It queues 200 ms of silence before the beep and waits 500 ms between the two sounds. The leading silence resolved a locally observed first-beep stutter; its underlying cause remains unknown. This workaround is confined to the native demo.

The browser build compiles the same generators to WebAssembly. A target-specific module uses `wasm-bindgen` to export `startupBeep` and `crashSound`. These low-level exports return `Float32Array` samples.

The handwritten JavaScript interface loads the Wasm, generates samples, copies them into an `AudioBuffer`, and plays them through Web Audio. Each engine object creates its audio context lazily on the first playback call and reuses it for subsequent calls. Each playback creates a new buffer source.

No native silence workaround is included in browser playback. The generated sample rate may differ from the device rate; the playback backend handles the conversion.

## Packaging

Cargo produces both `rlib` for Rust consumers and `cdylib` for the Wasm build. Rodio is a non-Wasm dependency; `wasm-bindgen` is a Wasm-only dependency.

The npm archive includes the playback interface, its TypeScript declarations, and the generated Wasm bindings. It is private and distributed locally, not published to the npm registry. Generated files are ignored by Git and must be rebuilt before packing.

## Verification and boundaries

Unit tests cover sample counts, beep polarity and amplitude, and crash amplitude and final silence at both rates. They do not verify beep frequency, spectral quality, or device playback.

The current CI checks Rust formatting, native Clippy, native builds, and native tests on Windows. It does not build Wasm, check JavaScript, or automate browser playback. Wasm compilation and listening have been checked manually during development.

The user confirmed browser playback in the standalone demo, a dedicated PHW Next.js page, and PHW's locally served static export. The main PHW page was not changed. GitHub Pages verification, PHW commits, and integration into its main sequence were delegated to PHW's other agent; no deployed verification is claimed here.

There is no mixing API, music playback, spatial audio, effects chain, streaming synthesis, or game-engine integration. The browser API does not yet expose stop, scheduling, completion notifications, or audio-context disposal. Add capabilities when a concrete use requires them.

See [the API guide](api.md) for callers and [the maintenance guide](maintenance.md) for rebuilding and resuming work.

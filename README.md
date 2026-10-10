# BeepEngine

A small independent audio engine written in Rust, developed progressively as a learning project.

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

## Quality checks

```sh
cargo fmt --all --check
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --all-targets
cargo test --locked --all-targets
```

GitHub Actions runs these checks on Windows for pushes and pull requests. Automated checks do not assess how the sounds feel; listening remains part of sound design.

## Scope

The initial objective is to generate and play two sounds while understanding the Rust code behind them. PHW is an initial consumer; integration with PHW or a game engine is future work.

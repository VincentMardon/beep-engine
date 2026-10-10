# BeepEngine

A small independent audio engine written in Rust, developed progressively as a learning project.

## Current sounds

- Startup beep: a square wave at approximately 880 Hz, lasting 100 ms.
- Crash sound: a synthesized arcade-inspired noise burst, lasting 700 ms, with a fading amplitude.

Both sounds are generated as mono `f32` samples at 48,000 Hz in the current demo. Rodio handles playback through the default audio output. No audio files are required.

## Run the demo

Install Rust with rustup, then run from the repository root:

```sh
cargo run --locked
```

The demo plays the startup beep, waits 500 ms, then plays the crash sound. A working default audio output is required.

The listening sequence belongs to the demo. The sound generation functions are independent of Professional Hello World (PHW) and do not open an audio device.

## Project structure

- `src/lib.rs` contains the public `startup_beep(sample_rate)` and `crash_sound(sample_rate)` functions, along with their unit tests. Each function returns mono samples as a `Vec<f32>`.
- `src/main.rs` uses the library and Rodio to play the two sounds in sequence.

The current demo and tests use a sample rate of 48,000 Hz. Other rates and invalid inputs have not been validated yet. The library generates samples without opening an audio device.

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

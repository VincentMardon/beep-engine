# API guide

## Rust sample generation

```rust
use beep_engine::{SampleRate, crash_sound, startup_beep};

let rate = SampleRate::Hz48000;
let beep = startup_beep(rate);
let crash = crash_sound(rate);
```

| Item | Contract |
| --- | --- |
| `SampleRate::Hz44100` | Generate at 44,100 Hz |
| `SampleRate::Hz48000` | Generate at 48,000 Hz |
| `SampleRate::hz()` | Return the numeric rate as `u32` |
| `startup_beep(rate)` | Return 100 ms of mono `Vec<f32>` samples |
| `crash_sound(rate)` | Return 700 ms of mono `Vec<f32>` samples |

The returned vector belongs to the caller. These functions generate samples only: they do not play sound, sleep, open devices, or create files. Pass the same rate to the playback buffer as was used to generate its samples.

## Browser playback package

The consumer installs the local npm archive, then imports:

```typescript
import { createBeepEngine } from "beep-engine";

const beepEngine = await createBeepEngine();
```

Initialize once in browser-side code, and retain the returned object for later calls. Initialization loads the Wasm module; it does not play a sound.

Call playback from a user interaction to activate browser audio:

```typescript
async function onStartupButtonClick() {
  await beepEngine.startupBeep(48000);
}

async function onCrashButtonClick() {
  await beepEngine.crashSound(44100);
}
```

| Method | Contract |
| --- | --- |
| `createBeepEngine()` | Return `Promise<BeepEngine>` after Wasm initialization |
| `startupBeep(sampleRate?)` | Generate and start playing the beep |
| `crashSound(sampleRate?)` | Generate and start playing the crash |

The sample rate defaults to 48,000 Hz. Supported explicit values are `44100` and `48000`. The TypeScript declarations express this as a union type. Unsupported numeric values are rejected by the Wasm adapter. Catch initialization and playback failures in the consumer UI.

Playback methods return `Promise<void>`. They resolve after playback has been started, not when the sound has finished. Calling the methods consecutively with `await` does not create a sequential listening queue. Consumer timing remains the consumer's responsibility. Repeated calls can overlap.

The package requires a browser for initialization and playback. In Next.js, initialize in browser-side lifecycle code rather than during server rendering. A client component can initialize in `useEffect` and invoke playback from event handlers. The tested PHW page is a dedicated audio test route, separate from the main experience.

## Low-level Wasm exports

The generated `web/wasm/beep_engine.js` module also exports `startupBeep(number)` and `crashSound(number)`. After its default initialization function has completed, these functions return `Float32Array` samples synchronously and do not play them.

The npm package's public entry point wraps these generators with Web Audio. Keep this distinction when debugging: generated exports produce data; the handwritten playback methods produce sound.

## Documentation tools

Rust's `rustdoc` is included with the toolchain. Generate the Rust library reference with:

```sh
cargo doc --locked --lib --no-deps --open
```

The output is under `target/doc/`, already ignored by Git. The library includes `//!` crate documentation and `///` comments describing its public sample-rate type and sound generators. Under the learning rules, changes to `.rs` files, including documentation comments, are made by the user.

The Markdown guides are the current narrative documentation. MkDocs can turn them into a site later, but no MkDocs configuration, dependency, or documentation hosting has been added.

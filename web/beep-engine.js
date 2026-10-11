import init, {
    startupBeep as generateStartupBeep,
    crashSound as generateCrashSound,
} from "./wasm/beep_engine.js";

/**
 * Loads the Wasm generators and returns a browser playback interface.
 * 
 * Retain the returned engine for subsequent playback calls.
 * Activate audio from a user interaction.
 * Playback promises resolve after starting a sound, not after it ends.
 */
export async function createBeepEngine() {
    await init();

    let context;

    async function play(generate, sampleRate) {
        const samples = generate(sampleRate);

        // Create the context lazily so the first playback can activate audio.
        context ??= new AudioContext();
        await context.resume();

        const buffer = context.createBuffer(1, samples.length, sampleRate);
        buffer.copyToChannel(samples, 0);

        // Buffer sources are single-use; create a new one for each playback.
        const source = context.createBufferSource();
        source.buffer = buffer;
        source.connect(context.destination);
        source.onended = () => source.disconnect();
        source.start();
    }

    return {
        startupBeep(sampleRate = 48000) {
            return play(generateStartupBeep, sampleRate);
        },

        crashSound(sampleRate = 48000) {
            return play(generateCrashSound, sampleRate);
        },
    };
}

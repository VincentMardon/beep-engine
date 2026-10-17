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
    let outputGain;
    let volume = 1;

    function setVolume(value) {
        if (!Number.isFinite(value) || value < 0 || value > 1) {
            throw new RangeError("Volume must be a finite number between 0 and 1.")
        }

        volume = value;

        if (outputGain) {
            const now = context.currentTime;

            outputGain.gain.cancelScheduledValues(now);

            if (volume === 0) {
                outputGain.gain.setValueAtTime(0, now);
            } else {
                outputGain.gain.setValueAtTime(volume, now, 0.01);
            }
        }
    }

    async function play(generate, sampleRate) {
        const samples = generate(sampleRate);

        // Create the context lazily so the first playback can activate audio.
        if (!context) {
            context = new AudioContext();
            outputGain = context.createGain();
            outputGain.gain.value = volume;
            outputGain.connect(context.destination);
        }

        await context.resume();

        //Start each new sound at the currently selected volume.
        const now = context.currentTime;
        outputGain.gain.cancelScheduledValues(now);
        outputGain.gain.setValueAtTime(volume, now);

        const buffer = context.createBuffer(1, samples.length, sampleRate);
        buffer.copyToChannel(samples, 0);

        // Buffer sources are single-use; create a new one for each playback.
        const source = context.createBufferSource();
        source.buffer = buffer;
        source.connect(outputGain);
        source.onended = () => source.disconnect();
        source.start();
    }

    return {
        setVolume,

        startupBeep(sampleRate = 48000) {
            return play(generateStartupBeep, sampleRate);
        },

        crashSound(sampleRate = 48000) {
            return play(generateCrashSound, sampleRate);
        },
    };
}

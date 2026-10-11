import init, {
    startupBeep as generateStartupBeep,
    crashSound as generateCrashSound,
} from "../target/web/beep_engine.js";

export async function createBeepEngine() {
    await init();

    let context;

    async function play(generate, sampleRate) {
        const samples = generate(sampleRate);

        context ??= new AudioContext();
        await context.resume();

        const buffer = context.createBuffer(1, samples.length, sampleRate);
        buffer.copyToChannel(samples, 0);

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

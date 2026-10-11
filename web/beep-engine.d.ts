export type SampleRate = 44100 | 48000;

export interface BeepEngine {
    /** Starts the beep. Defaults to 48000 Hz; does not await its end. */
    startupBeep(sampleRate?: SampleRate): Promise<void>;

    /** Starts the crash. Defaults to 48000 Hz; does not await its end. */
    crashSound(sampleRate?: SampleRate): Promise<void>;
}

/** Initializes the Wasm module for browser playback. */
export function createBeepEngine(): Promise<BeepEngine>;
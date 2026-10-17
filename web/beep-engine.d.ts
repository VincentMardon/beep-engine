export type SampleRate = 44100 | 48000;

export interface BeepEngine {
    /**
     * Sets the shared playback volume between 0 and 1.
     * Defaults to 1 and also affects sounds already playing.
     * Throws RangeError for invalid values.
     */
    setVolume(volume: number): void;
    
    /** Starts the beep. Defaults to 48000 Hz; does not await its end. */
    startupBeep(sampleRate?: SampleRate): Promise<void>;

    /** Starts the crash. Defaults to 48000 Hz; does not await its end. */
    crashSound(sampleRate?: SampleRate): Promise<void>;
}

/** Initializes the Wasm module for browser playback. */
export function createBeepEngine(): Promise<BeepEngine>;
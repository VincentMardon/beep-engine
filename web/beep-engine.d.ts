export type SampleRate = 44100 | 48000;

export interface BeepEngine {
    startupBeep(sampleRate?: SampleRate): Promise<void>;
    crashSound(sampleRate?: SampleRate): Promise<void>;
}

export function createBeepEngine(): Promise<BeepEngine>;
//! Small synthesized sounds for native and browser consumers.
//!
//! Sound generators return mono samples without opening an audio device.
//! Playback and application-specific timing belong to the consumer.

/// Supported sample rates for sound generation.
#[derive(Clone, Copy)]
pub enum SampleRate {
    /// Generate 44,100 samples per second.
    Hz44100,
    /// Generate 48,000 samples per second.
    Hz48000,
}

impl SampleRate {
    /// Returns the sample rate in hertz.
    pub fn hz(self) -> u32 {
        match self {
            Self::Hz44100 => 44_100,
            Self::Hz48000 => 48_000,
        }
    }
}

/// Generates a 100 ms square-wave beep at approximately 880 Hz.
///
/// Returns mono samples with an amplitude of 0.2.
/// The caller must use the same sample rate for playback.
/// This function does not play the sound.
pub fn startup_beep(sample_rate: SampleRate) -> Vec<f32> {
    let sample_rate = sample_rate.hz();
    let frequency: f32 = 880.0;
    let duration_ms: u32 = 100;
    let amplitude: f32 = 0.2;

    let sample_count = sample_rate * duration_ms / 1_000;
    let phase_step = frequency / sample_rate as f32;
    let mut phase: f32 = 0.0;
    let mut samples = Vec::new();

    for _ in 0..sample_count {
        let sample = if phase < 0.5 { amplitude } else { -amplitude };

        samples.push(sample);

        phase += phase_step;

        if phase >= 1.0 {
            // Preserve the fractional overshoot when starting the next cycle.
            phase -= 1.0;
        }
    }

    samples
}

/// Generates a 700 ms arcade-inspired noise burst.
///
/// Returns deterministic mono samples with a peak amplitude of 0.2
/// and an envelope that fades to silence.
/// The caller must use the same sample rate for playback.
/// This function does not play the sound.
pub fn crash_sound(sample_rate: SampleRate) -> Vec<f32> {
    let sample_rate = sample_rate.hz();
    let duration_ms: u32 = 700;
    let amplitude: f32 = 0.2;
    let sample_count = sample_rate * duration_ms / 1_000;

    let mut samples = Vec::new();
    // A fixed seed produces the same noise sequence on each call.
    let mut random_state: u32 = 123_456_789;
    let mut noise: f32 = 0.0;

    let noise_update_rate: u32 = 1_000;
    // Update immediately on the first sample, then 1,000 times per second.
    let mut noise_clock = sample_rate;

    for sample_index in 0..sample_count {
        if noise_clock >= sample_rate {
            // Preserve the remainder to avoid timing drift at 44,100 Hz.
            noise_clock -= sample_rate;

            // Wrapping arithmetic implements the generator modul 2^32.
            random_state = random_state
                .wrapping_mul(1_664_525)
                .wrapping_add(1_013_904_223);

            noise = if random_state >= 2_147_483_648 {
                1.0
            } else {
                -1.0
            };
        }

        let progress = sample_index as f32 / (sample_count - 1) as f32;
        // A squared decay gives a strong onset and a quiet tail.
        let envelope = (1.0 - progress) * (1.0 - progress);

        samples.push(noise * amplitude * envelope);

        noise_clock += noise_update_rate;
    }

    samples
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::{SampleRate, crash_sound, startup_beep};
    use wasm_bindgen::prelude::*;

    fn parse_sample_rate(value: f64) -> Result<SampleRate, JsError> {
        if value == 44_100.0 {
            Ok(SampleRate::Hz44100)
        } else if value == 48_000.0 {
            Ok(SampleRate::Hz48000)
        } else {
            Err(JsError::new(
                "Unsupported sample rate: expected 44100 or 48000",
            ))
        }
    }

    /// Returns startup beep samples as a JavaScript Float32Array.
    ///
    /// Throws a JavaScript Error if the sample rate is unsupported.
    /// Does not perform playback.
    #[wasm_bindgen(js_name = startupBeep)]
    pub fn generate_startup_beep(sample_rate: f64) -> Result<Vec<f32>, JsError> {
        let sample_rate = parse_sample_rate(sample_rate)?;
        Ok(startup_beep(sample_rate))
    }

    /// Returns crash sound samples as a JavaScript Float32Array.
    ///
    /// Throws a JavaScript Error if the sample rate is unsupported.
    /// Does not perform playback.
    #[wasm_bindgen(js_name = crashSound)]
    pub fn generate_crash_sound(sample_rate: f64) -> Result<Vec<f32>, JsError> {
        let sample_rate = parse_sample_rate(sample_rate)?;
        Ok(crash_sound(sample_rate))
    }
}

#[cfg(test)]
mod tests {
    use super::{SampleRate, crash_sound, startup_beep};

    #[test]
    fn sounds_have_the_expected_duration() {
        for (sample_rate, beep_count, crash_count) in [
            (SampleRate::Hz44100, 4_410, 30_870),
            (SampleRate::Hz48000, 4_800, 33_600),
        ] {
            assert_eq!(startup_beep(sample_rate).len(), beep_count);
            assert_eq!(crash_sound(sample_rate).len(), crash_count);
        }
    }

    #[test]
    fn startup_beep_has_both_polarities_and_expected_amplitude() {
        for sample_rate in [SampleRate::Hz44100, SampleRate::Hz48000] {
            let samples = startup_beep(sample_rate);

            assert!(samples.iter().any(|sample| *sample > 0.0));
            assert!(samples.iter().any(|sample| *sample < 0.0));
            assert!(
                samples
                    .iter()
                    .all(|sample| (sample.abs() - 0.2).abs() < 0.000_001)
            );
        }
    }

    #[test]
    fn crash_sound_starts_audibly_and_ends_in_silence() {
        for sample_rate in [SampleRate::Hz44100, SampleRate::Hz48000] {
            let samples = crash_sound(sample_rate);

            assert!(!samples.is_empty());
            assert!(samples.iter().all(|sample| sample.abs() <= 0.2));
            assert!(samples[0].abs() > 0.1);
            assert!(samples.last().unwrap().abs() < 0.000_001);
        }
    }
}

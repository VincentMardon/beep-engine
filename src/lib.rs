#[derive(Clone, Copy)]
pub enum SampleRate {
    Hz44100,
    Hz48000,
}

impl SampleRate {
    pub fn hz(self) -> u32 {
        match self {
            Self::Hz44100 => 44_100,
            Self::Hz48000 => 48_000,
        }
    }
}

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
            phase -= 1.0;
        }
    }

    samples
}

pub fn crash_sound(sample_rate: SampleRate) -> Vec<f32> {
    let sample_rate = sample_rate.hz();
    let duration_ms: u32 = 700;
    let amplitude: f32 = 0.2;
    let sample_count = sample_rate * duration_ms / 1_000;

    let mut samples = Vec::new();
    let mut random_state: u32 = 123_456_789;
    let mut noise: f32 = 0.0;

    let noise_update_rate: u32 = 1_000;
    let mut noise_clock = sample_rate;

    for sample_index in 0..sample_count {
        if noise_clock >= sample_rate {
            noise_clock -= sample_rate;

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
        let envelope = (1.0 - progress) * (1.0 - progress);

        samples.push(noise * amplitude * envelope);

        noise_clock += noise_update_rate;
    }

    samples
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

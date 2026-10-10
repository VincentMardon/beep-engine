pub fn startup_beep(sample_rate: u32) -> Vec<f32> {
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

pub fn crash_sound(sample_rate: u32) -> Vec<f32> {
    let duration_ms: u32 = 700;
    let amplitude: f32 = 0.2;
    let sample_count = sample_rate * duration_ms / 1_000;

    let mut samples = Vec::new();
    let mut random_state: u32 = 123_456_789;
    let mut noise: f32 = 0.0;

    for sample_index in 0..sample_count {
        if sample_index % 48 == 0 {
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
    }

    samples
}

#[cfg(test)]
mod tests {
    use super::{crash_sound, startup_beep};

    #[test]
    fn sounds_have_the_expected_duration() {
        assert_eq!(startup_beep(48_000).len(), 4_800);
        assert_eq!(crash_sound(48_000).len(), 33_600);
    }

    #[test]
    fn startup_beep_has_both_polarities_and_expected_amplitude() {
        let samples = startup_beep(48_000);

        assert!(samples.iter().any(|sample| *sample > 0.0));
        assert!(samples.iter().any(|sample| *sample < 0.0));
        assert!(
            samples
                .iter()
                .all(|sample| (sample.abs() - 0.2).abs() < 0.000_001)
        );
    }

    #[test]
    fn crash_sound_starts_audibly_and_ends_in_silence() {
        let samples = crash_sound(48_000);

        assert!(!samples.is_empty());
        assert!(samples.iter().all(|sample| sample.abs() <= 0.2));
        assert!(samples[0].abs() > 0.1);
        assert!(samples.last().unwrap().abs() < 0.000_001);
    }
}

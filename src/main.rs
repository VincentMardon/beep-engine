use rodio::{DeviceSinkBuilder, Player, buffer::SamplesBuffer};
use std::{
    num::{NonZeroU16, NonZeroU32},
    thread,
    time::Duration,
};

fn startup_beep(sample_rate: u32) -> Vec<f32> {
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

fn crash_sound(sample_rate: u32) -> Vec<f32> {
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

fn main() {
    let sample_rate: u32 = 48_000;

    let output =
        DeviceSinkBuilder::open_default_sink().expect("Failed to open the default audio output");

    let player = Player::connect_new(output.mixer());

    let channels = NonZeroU16::new(1).expect("Channel count must be nonzero");
    let playback_sample_rate = NonZeroU32::new(sample_rate).expect("Sample rate must be nonzero");

    println!("Playing startup beep.");
    player.append(SamplesBuffer::new(
        channels,
        playback_sample_rate,
        startup_beep(sample_rate),
    ));
    player.sleep_until_end();

    thread::sleep(Duration::from_millis(500));

    println!("Playing crash sound.");
    player.append(SamplesBuffer::new(
        channels,
        playback_sample_rate,
        crash_sound(sample_rate),
    ));
    player.sleep_until_end();
}

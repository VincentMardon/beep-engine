use rodio::{DeviceSinkBuilder, Player, buffer::SamplesBuffer};
use std::num::{NonZeroU16, NonZeroU32};

fn main() {
    let sample_rate: u32 = 48_000;
    let frequency: u32 = 880;
    let duration_ms: u32 = 100;

    let samples_per_period = sample_rate / frequency;
    let sample_count = sample_rate * duration_ms / 1_000;

    let mut samples: Vec<f32> = Vec::new();

    for sample_index in 0..sample_count {
        let position_in_period = sample_index % samples_per_period;

        let amplitude: f32 = if position_in_period < samples_per_period / 2 {
            0.2
        } else {
            -0.2
        };

        samples.push(amplitude);
    }

    println!("Generated {} samples.", samples.len());

    let output =
        DeviceSinkBuilder::open_default_sink().expect("Failed to open the default audio output");

    let player = Player::connect_new(output.mixer());

    let channels = NonZeroU16::new(1).expect("Channel count must be nonzero");
    let playback_sample_rate = NonZeroU32::new(sample_rate).expect("Sample rate must be nonzero");

    let source = SamplesBuffer::new(channels, playback_sample_rate, samples);

    player.append(source);
    player.sleep_until_end();
}

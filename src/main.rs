use beep_engine::{crash_sound, startup_beep};

use rodio::{DeviceSinkBuilder, Player, buffer::SamplesBuffer};
use std::{
    num::{NonZeroU16, NonZeroU32},
    thread,
    time::Duration,
};

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

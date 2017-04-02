extern crate ears;

use ears::{Sound, AudioController};

pub fn play_sound(chunk: char) {
    let mut sound: Sound;
    match chunk {
        '·' =>  {
            sound = Sound::new("short.ogg").unwrap()
        },
        '−' =>  {
            sound = Sound::new("long.ogg").unwrap()
        },
        _ => panic!("Unknown character: {}", chunk)
    };
    sound.play();
    while sound.is_playing() {};
}

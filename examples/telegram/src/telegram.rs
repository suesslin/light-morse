extern crate ears;

use ears::{Sound, AudioController};

const DIR: &'static str = "resrc/";
const EXT: &'static str = "ogg";

pub fn play_sound(chunk: char) {
    let mut sound = match chunk {
        '·' =>  {
            Sound::new(&format!("{}{}.{}", DIR, "short", EXT)).unwrap()
        },
        '−' =>  {
            Sound::new(&format!("{}{}.{}", DIR, "long", EXT)).unwrap()
        },
        ' ' => {
            Sound::new(&format!("{}{}.{}", DIR, "break", EXT)).unwrap()
        },
        _ => panic!("Unknown character: {}", chunk)
    };
    sound.play();
    while sound.is_playing() {};
}

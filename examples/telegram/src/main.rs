mod telegram;

extern crate light_morse;
extern crate clap;
extern crate ears;
extern crate itertools;

use light_morse::*;
use clap::*;
use ears::{Sound, AudioController};
use telegram::*;
use itertools::*;

fn main() {
    let telegram = App::new("Telegram")
                    .arg(Arg::with_name("message")
                        .short("msg")
                        .long("message")
                        .takes_value(true)
                        .required(true))
                    .arg(Arg::with_name("type")
                        .short("type")
                        .long("morse-type")
                        .required(true)
                        .takes_value(true)
                        .possible_values(&["ITU", "Gerke", "Morse"]));
    let matches = telegram.get_matches();
    let message = matches.value_of("message").unwrap().to_string();

    let msg = match matches.value_of("type").unwrap().to_lowercase().trim() {
            "itu" => message.to_morse(MorseType::ITU),
            "gerke" => message.to_morse(MorseType::Gerke),
            "morse" => message.to_morse(MorseType::Morse),
            _ => panic!("An unknown type has been found")
    };
     msg.chars().into_iter().foreach(|chr|{ play_sound(chr) })
}

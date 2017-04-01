extern crate light_morse;
extern crate clap;

use light_morse::*;
use clap::*;

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

    println!("Result is: {}", {
        match matches.value_of("type").unwrap().to_lowercase().trim() {
            "itu" => message.to_morse(MorseType::ITU),
            "gerke" => message.to_morse(MorseType::Gerke),
            "morse" => message.to_morse(MorseType::Morse),
            _ => panic!("An unknown type has been found")
        }
    });

}

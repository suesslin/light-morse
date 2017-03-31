extern crate light_morse;

use light_morse::*;

fn main() {
    let hehe: Morse = "Samuel Morse".to_string().to_morse(MorseType::ITU);
    println!("{}", hehe);
}

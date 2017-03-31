extern crate morse;

use morse::*;

fn main() {
    let hehe: Morse = "Samuel Morse".to_string().to_morse(MorseType::ITU);
    println!("{}", hehe);
}

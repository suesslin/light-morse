extern crate light_morse;

use light_morse::*;

fn main() {
    let hehe: Morse = "Morse".to_string().to_morse(MorseType::Gerke);
    println!("{}", hehe);
}

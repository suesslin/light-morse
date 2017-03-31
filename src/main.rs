extern crate morse;

use morse::*;

fn main() {
    let hehe: Morse = "ABBA".to_string().to_morse(MorseSelection::ITU);
    println!("{}", hehe);
}

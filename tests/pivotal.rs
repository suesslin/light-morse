extern crate morse;

use morse::{Morse, MorseType, MorseSubstiution};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn itu_morse_correct() {
        let starting_word: Morse = "Samuel Morse".to_string().to_morse(MorseType::ITU);
        println!("{}", starting_word);
        assert_eq!(starting_word, "····−−−··−··−·· −−−−−·−·····")
    }
    #[test]
    fn itu_differs() {
        let word1: Morse = "Samuel Morse".to_string();
        let word2: Morse = "Alex Crichton".to_string();
        assert_ne!(word1.to_morse(MorseType::ITU), word2.to_morse(MorseType::ITU))
    }
    // TODO:
    // Test that proves, substitutions aren't the same.
}

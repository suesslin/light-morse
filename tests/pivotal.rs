extern crate light_morse;

use light_morse::{Morse, MorseSubstitution, MorseType};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn itu_morse_correct() {
        let starting_word: Morse = "Morse".to_string().to_morse(MorseType::ITU);
        assert_eq!(starting_word, "−−−−−·−·····")
    }
    #[test]
    fn itu_differs() {
        let word1: Morse = "Morse".to_string();
        let word2: Morse = "Telegram".to_string();
        assert_ne!(
            word1.to_morse(MorseType::ITU),
            word2.to_morse(MorseType::ITU)
        )
    }
    // TODO:
    // Test that proves, substitutions aren't the same.
    #[test]
    fn itu_and_gerke_differ() {
        let word1: Morse = "Morse".to_string();
        assert_ne!(
            word1.to_morse(MorseType::ITU),
            word1.to_morse(MorseType::Gerke)
        )
    }

    #[test]
    fn gerke_morse_correct() {
        let starting_word: Morse = "Morse".to_string().to_morse(MorseType::Gerke);
        assert_eq!(starting_word, "−−·−····−·····")
    }

    #[test]
    fn gerke_differs() {
        let word1: Morse = "Morse".to_string();
        let word2: Morse = "Telegram".to_string();
        assert_ne!(
            word1.to_morse(MorseType::Gerke),
            word2.to_morse(MorseType::Gerke)
        )
    }
}

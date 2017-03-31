mod substitution;

use substitution::{MorseChunk, itu_substitution};

pub type Plaintext = String;
pub type Morse = String;

// MORSE TYPES
pub enum MorseType {
    ITU,
    American,
    Continental
}

// MORSE FUNCTIONS
fn with_substitute(word: &Plaintext, substitue_method: fn(char) -> MorseChunk) -> Morse {
    word.chars().into_iter().map(|chr|{ substitue_method(chr) }).collect::<Vec<MorseChunk>>().join("")
}

pub trait MorseSubstiution {
    fn to_morse(&self, morse_type: MorseType) -> Morse;
}

impl MorseSubstiution for Plaintext {
    fn to_morse(&self, morse_type: MorseType) -> Morse {
        match morse_type {
            MorseType::ITU => with_substitute(&self, itu_substitution),
            _ => panic!("Other methods than ITU not implemented.")
        }
    }
}

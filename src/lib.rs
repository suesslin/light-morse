mod substitution;

use substitution::{gerke_substitution, itu_substitution, MorseChunk};

pub type Plaintext = String;
pub type Morse = String;

// MORSE TYPES
pub enum MorseType {
    ITU,   // International
    Morse, //American
    Gerke, // Continental
}

// MORSE FUNCTIONS
fn with_substitute(word: &Plaintext, substitue_method: fn(char) -> MorseChunk) -> Morse {
    word.to_uppercase()
        .chars()
        .into_iter()
        .map(substitue_method)
        .collect::<Vec<MorseChunk>>()
        .join("")
}

pub trait MorseSubstitution {
    fn to_morse(&self, morse_type: MorseType) -> Morse;
}

impl MorseSubstitution for Plaintext {
    fn to_morse(&self, morse_type: MorseType) -> Morse {
        match morse_type {
            MorseType::ITU => with_substitute(&self, itu_substitution),
            MorseType::Gerke => with_substitute(&self, gerke_substitution),
            _ => panic!("Other methods than ITU not implemented."),
        }
    }
}

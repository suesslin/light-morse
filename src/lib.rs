pub type Plaintext = String;
pub type Morse = String;
pub type MorseChunk = String;

// MORSE TYPES
pub enum MorseType {
    ITU,
    American,
    Continental
}

// SUBSTITUTION
//// Chars for substitution: ·−
// International
fn itu_substitution(chr: char) -> MorseChunk {
    let morse_chunk: &str = {
        match chr {
            'A' => "·−",
            'B' => "−···",
            'C' => "−·−·",
            'D' => "−··",
            _ => panic!("{} couldn't be used in substitution.", chr)
        }
    };
    morse_chunk.to_string()
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

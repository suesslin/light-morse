pub type Plaintext = String;
pub type Morse = String;
pub type MorseChunk = String;

// MORSE TYPES
pub enum MorseSelection {
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
// International
fn to_itu_morse(word: &Plaintext) -> Morse {
    word.chars().into_iter().map(|chr|{ itu_substitution(chr) }).collect::<Vec<MorseChunk>>().join("")
}

// MAIN FUNCTION
// Takes Morse functions as argument
fn to_cetain_morse(word: &Plaintext, func_type: fn(&Plaintext) -> Morse) -> Morse {
    // IDEA: Create closure that takes different substitution fn's.
    func_type(word)
}

pub trait MorseSubstiution {
    fn to_morse(&self, m_type: MorseSelection) -> Morse;
}

impl MorseSubstiution for Plaintext {
    fn to_morse(&self, m_type: MorseSelection) -> Morse {
        match m_type {
            MorseSelection::ITU => to_cetain_morse(&self.to_uppercase(), to_itu_morse),
            _ => panic!("Not implemented yet.")
        }
    }
}

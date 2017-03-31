pub type Plaintext = String;
pub type Morse = String;

pub trait MorseSubstiution {
    fn to_inter_morse(&self) -> Morse;
}

impl MorseSubstiution for Plaintext {
    fn to_inter_morse(&self) -> Morse {
        "Hello".to_string()
    }
}

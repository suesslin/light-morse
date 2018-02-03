pub type MorseChunk = String;
pub type PlainChunk = char;

//// Chars for substitution: ·−

// International (ITU)
pub fn itu_substitution(chr: PlainChunk) -> MorseChunk {
    let morse_chunk: &str = {
        match chr {
            'A' => "·−",
            'B' => "−···",
            'C' => "−·−·",
            'D' => "−··",
            'E' => "·",
            'F' => "··−·",
            'G' => "−−·",
            'H' => "····",
            'I' => "··",
            'J' => "·−−−",
            'K' => "−·−",
            'L' => "·−··",
            'M' => "−−",
            'N' => "−·",
            'O' => "−−−",
            'P' => "·−−·",
            'Q' => "−−·−",
            'R' => "·−·",
            'S' => "···",
            'T' => "−",
            'U' => "··−",
            'V' => "···−",
            'W' => "·−−",
            'X' => "−··−",
            'Y' => "−·−−",
            'Z' => "−−··",
            ' ' => " ",
            _ => panic!("{} couldn't be used in substitution.", chr),
        }
    };
    morse_chunk.to_string()
}

// Continental (Gerke)
pub fn gerke_substitution(chr: PlainChunk) -> MorseChunk {
    let morse_chunk: &str = {
        match chr {
            'A' => "·−",
            'B' => "·−·−",
            'C' => "−···",
            'D' => "−··",
            'E' => "·",
            'F' => "··−·",
            'G' => "−−·",
            'H' => "····",
            'I' => "··",
            'J' => "·−−−",
            'K' => "−·−",
            'L' => "·−··",
            'M' => "−−",
            'N' => "−·",
            'O' => "·−···",
            'P' => "·····",
            'Q' => "−−·−",
            'R' => "·−·",
            'S' => "···",
            'T' => "−",
            'U' => "··−",
            'V' => "···−",
            'W' => "·−−",
            'X' => "··−···",
            'Y' => "−−···",
            'Z' => "·−−··",
            ' ' => " ",
            _ => panic!("{} couldn't be used in substitution.", chr),
        }
    };
    morse_chunk.to_string()
}

// // American (Morse)
// pub fn morse_substitution(chr: char) -> MorseChunk {
//     let morse_chunk: &str = {
//         match chr {
//             'A' => "·−",
//             'B' => "−···",
//             'C' => "·· ·",
//             'D' => "−··",
//             'E' => "·",
//             'F' => "·−·",
//             'G' => "−−·",
//             'H' => "····",
//             'I' => "··",
//             'J' => "−·−·",
//             'K' => "−·−",
//             'L' => "-−··",
//             'M' => "−−",
//             'N' => "−·",
//             'O' => "−−−",
//             'P' => "·−−·",
//             'Q' => "−−·−",
//             'R' => "·−·",
//             'S' => "···",
//             'T' => "−",
//             'U' => "··−",
//             'V' => "···−",
//             'W' => "·−−",
//             'X' => "−··−",
//             'Y' => "−·−−",
//             'Z' => "−−··",
//             ' ' => " ",
//             _ => panic!("{} couldn't be used in substitution.", chr)
//         }
//     };
//     morse_chunk.to_string()
// }

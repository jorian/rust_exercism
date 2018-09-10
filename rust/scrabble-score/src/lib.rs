/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .fold(0, |acc, c | {
            match c.to_ascii_lowercase() {
                'a'|'e'|'i'|'o'|'u'|'l'|'n'|'r'|'s'|'t' => acc + 1,
                'd'|'g'                                 => acc + 2,
                'b'|'c'|'m'|'p'                         => acc + 3,
                'f'|'h'|'v'|'w'|'y'                     => acc + 4,
                'k'                                     => acc + 5,
                'j'|'x'                                 => acc + 8,
                'q'|'z'                                 => acc + 10,
                _                                       => acc + 0
            }
    })
}

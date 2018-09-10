use std::collections::HashMap;

/// Counts the amount of nucleotides in the searchstring.
/// If a char is found that not matches the nucleotide, an error is returned.
pub fn count(nucleotide: char, searchstr: &str) -> Result<u32, char> {
    let valids = vec!['A', 'C', 'G', 'T'];
    let mut count = 0;

    if valids.contains(&nucleotide) {
        for ch in searchstr.chars() {
            if valids.contains(&ch) {
                count = searchstr
                    .chars()
                    .filter(|c| *c == nucleotide)
                    .count();
            } else {
                return Err(ch)
            }
        }
    } else {
        return Err(nucleotide)
    }

    Ok(count as u32)
}

pub fn nucleotide_counts(searchstr: &str) -> Result<HashMap<char, usize>, char> {
    let mut m: HashMap<char, _> = vec![
        ('A', 0), ('C', 0), ('G', 0), ('T',0)
    ].into_iter().collect();

    for ch in searchstr.chars() {
        match ch {
            'A' | 'C' | 'G' | 'T' => {
                let counter = m.entry(ch).or_insert(0);
                *counter += 1;
            },
            _ => return Err(ch)
        }
    }

    Ok(m)
}
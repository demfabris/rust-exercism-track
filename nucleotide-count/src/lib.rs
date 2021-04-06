use std::collections::HashMap;

pub fn is_nucleotide_valid(nucleotide: char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut count = 0;

    if !is_nucleotide_valid(nucleotide) {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if is_nucleotide_valid(c) {
            if nucleotide == c {
                count += 1;
            }
        } else {
            return Err(c);
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('G', 0);
    map.insert('T', 0);
    map.insert('C', 0);

    for c in dna.chars() {
        if is_nucleotide_valid(c) {
            map.insert(c, map.get(&c).unwrap_or(&0) + 1);
        } else {
            return Err(c);
        }
    }

    Ok(map)
}

pub fn worth(c: char) -> u64 {
    let c: char = c.to_ascii_lowercase();
    if let 'a'..='z' = c {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        }
    } else {
        0
    }
}

pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |acc, c| acc + worth(c))
}

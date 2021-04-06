pub fn is_pangram(sentence: &str) -> bool {
    let mut cache = vec![];

    for c in sentence.chars() {
        let c = c.to_ascii_lowercase();
        if !cache.contains(&c) && c.is_alphabetic() {
            cache.push(c.clone());
        }
    }

    cache.len() >= 26
}

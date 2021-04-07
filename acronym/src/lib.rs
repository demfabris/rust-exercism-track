pub fn abbreviate(phrase: &str) -> String {
    let initials: String = phrase
        .replace("_", "")
        .replace("-", " ")
        .split(' ')
        .filter_map(|word| {
            let word = word.chars().nth(0).unwrap_or(' ');
            if word.is_alphabetic() {
                Some(word.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect();

    initials
}

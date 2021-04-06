pub fn check(candidate: &str) -> bool {
    let mut cache: Vec<char> = vec![];

    candidate.to_lowercase().chars().all(|c| {
        if let Some(_) = cache.iter().find(|&&x| x != '-' && x != ' ' && x == c) {
            return false;
        } else {
            cache.push(c);
            return true;
        }
    })
}

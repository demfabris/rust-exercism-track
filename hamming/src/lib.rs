pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut distance = 0;
    if s1.len() != s2.len() {
        return None;
    } else {
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                distance += 1;
            } else {
                continue;
            }
        }
    }

    Some(distance)
}

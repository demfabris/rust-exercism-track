pub fn m(n: u32, f: u32) -> bool {
    n % f == 0
}

pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");

    if m(n, 3) {
        result.push_str("Pling")
    }

    if m(n, 5) {
        result.push_str("Plang")
    }

    if m(n, 7) {
        result.push_str("Plong")
    }

    if result.len() == 0 {
        result = n.to_string()
    }

    result
}

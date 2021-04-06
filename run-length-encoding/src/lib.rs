pub fn encode(source: &str) -> String {
    let chars = source.chars().collect::<Vec<char>>();
    let mut result = String::from("");
    let mut count = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;

            if i == chars.len() - 1 {
                result.push_str(&format!("{}{}", count, chars[i - 1]));
                break;
            } else {
                continue;
            }
        }

        match count {
            1 => result.push_str(&format!("{}", chars[i - 1])),
            _ => result.push_str(&format!("{}{}", count, chars[i - 1])),
        }
        count = 1;

        if i == chars.len() - 1 {
            result.push_str(&format!("{}", chars[i]))
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let chars = source.chars().collect::<Vec<char>>();
    let mut result = String::from("");
    let mut factor = String::from("");

    for c in chars.iter() {
        if c.is_numeric() {
            factor.push_str(&format!("{}", c));
        } else {
            let f = factor.parse::<usize>().unwrap_or(1);
            for _ in 0..f {
                result.push_str(&format!("{}", c));
            }

            factor = String::from("");
        }
    }

    result
}

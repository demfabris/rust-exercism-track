pub fn is_valid_isbn(isbn: &str) -> bool {
    let filtered_isbn = isbn
        .chars()
        .filter(|&c| c.is_numeric() || c == 'X')
        .collect::<String>();
    let mut result = 0;

    if filtered_isbn.len() == 10 {
        for (idx, c) in filtered_isbn.chars().rev().enumerate() {
            match c {
                'X' => result = result + (idx + 1) * 10,
                c if c.is_numeric() => {
                    result = result + (idx + 1) * (c.to_digit(10).unwrap()) as usize
                }
                _ => return false,
            }
        }
    } else {
        return false;
    }

    result % 11 == 0
}

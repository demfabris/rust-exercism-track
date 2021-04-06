pub fn m(n: u64, f: u64) -> bool {
    n % f == 0
}

pub fn is_leap_year(year: u64) -> bool {
    match year {
        x if m(x, 4) && !m(x, 100) => true,
        x if m(x, 100) && m(x, 400) => true,
        _ => false,
    }
}

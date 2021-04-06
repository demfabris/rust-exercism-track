#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum = (1..=num / 2).fold(0, |acc, x| if num % x == 0 { acc + x } else { acc });

    match sum {
        sum if sum < num => Some(Classification::Deficient),
        0 => None,
        sum if sum > num => Some(Classification::Abundant),
        sum if sum == num => Some(Classification::Perfect),
        _ => None,
    }
}

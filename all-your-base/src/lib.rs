#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    let mut result = vec![];
    let mut value = 0;
    let valid_digit_range = 0..from_base;

    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    for (idx, digit) in number.iter().rev().enumerate() {
        if valid_digit_range.contains(digit) {
            value += digit * from_base.pow(idx as u32);
        } else {
            return Err(Error::InvalidDigit(*digit));
        }
    }

    if value == 0 {
        result.push(value);
    }

    while value > 0 {
        result.push(value % to_base);
        value = value.div_euclid(to_base);
    }

    result.reverse();

    Ok(result)
}

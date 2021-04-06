pub fn is_divisible_by_factors(number: u32, factors: &[u32]) -> bool {
    let mut result = false;
    for factor in factors.iter() {
        if *factor == 0 {
            continue;
        }
        if number % factor == 0 {
            result = true;
        }
    }

    result
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let result = (0..limit).fold(0, |acc, x| {
        if is_divisible_by_factors(x, factors) {
            return acc + x;
        } else {
            return acc;
        }
    });

    println!("{}", result);

    result
}

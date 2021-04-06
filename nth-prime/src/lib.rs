pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..n - 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut number = 2;
    let mut count = 0;

    while n != count {
        if is_prime(number) {
            count = count + 1;
            number = number + 1;
        } else {
            number = number + 1;
        }
    }

    number
}

pub fn main() {
    println!("{}", nth(5));
    unimplemented!();
}

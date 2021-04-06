pub fn is_prime(n: u64) -> bool {
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

pub fn factors(n: u64) -> Vec<u64> {
    let mut aux = n;
    let mut result = vec![];

    if n < 2 {
        return result;
    }

    while aux != 1 {
        for i in 2..=aux {
            if is_prime(i) && aux % i == 0 {
                aux = aux / i;
                result.push(i);
                break;
            } else {
                continue;
            }
        }
    }

    result
}

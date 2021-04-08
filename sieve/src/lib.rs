use std::collections::BTreeMap;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut btmap = BTreeMap::new();
    let mut stop = false;
    let mut p = 2;

    for i in 2..=upper_bound {
        btmap.insert(i, false);
    }

    while !stop {
        for (number, _) in btmap.clone() {
            let candidate = number * p;
            if candidate <= upper_bound {
                btmap.insert(p * number, true);
            }
        }

        p += 1;

        if p >= upper_bound {
            stop = true;
        }
    }

    // println!("{:?}", btmap);

    btmap
        .keys()
        .filter_map(|&x| {
            if let Some(marked) = btmap.get(&x) {
                if !marked {
                    return Some(x.to_owned());
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

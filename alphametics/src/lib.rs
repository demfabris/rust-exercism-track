use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut right_permunation: Option<HashMap<char, u8>> = None;
    let mut map: HashMap<char, u8> = HashMap::new();
    let mut set: HashSet<char> = HashSet::new();
    let tokens = input.replace("+", "").replace("==", "");
    let tokens: Vec<&str> = tokens.split(" ").filter(|&x| x != "").collect();

    for token in &tokens {
        token.chars().for_each(|c| {
            set.insert(c);
        });
    }

    let mut perms = (0..10).permutations(set.len());

    while let Some(perm) = perms.next() {
        for (index, c) in set.iter().enumerate() {
            map.insert(*c, perm[index]);
        }

        let mut tokens_as_number: Vec<u32> = tokens
            .iter()
            .map(|t| {
                let mut t = t.to_string();
                for (c, n) in &map {
                    t = t
                        .to_string()
                        .replace(&c.to_string()[..], &n.to_string()[..]);
                }

                t.parse::<u32>().unwrap()
            })
            .collect();

        let result = tokens_as_number.pop().unwrap();
        let sum = tokens_as_number.iter().fold(0, |acc, x| acc + x);

        if sum == result {
            right_permunation = Some(map.clone());
        }

        map.clear();
    }

    right_permunation
}

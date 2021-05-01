use std::collections::{HashMap, HashSet};

type CharMap = HashMap<char, usize>;

// N: length of word
// S: length of possible_anagrams

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_map: CharMap = HashMap::new();
    let mut result: HashSet<&'a str> = HashSet::new();

    word.chars().for_each(|c| {
        let count = *word_map.get(&c).unwrap_or(&0usize);
        word_map.insert(c, count + 1);
    }); // O(N)

    for &candidate in possible_anagrams {
        let mut candidate_map: CharMap = HashMap::new();

        if candidate.len() != word.len() {
            continue;
        }

        if candidate == word {
            continue;
        }

        candidate.chars().for_each(|c| {
            let count = *candidate_map.get(&c).unwrap_or(&0usize);
            candidate_map.insert(c, count + 1);
        }); // O(N)

        let mut is_anagram = true;

        for (c, count) in &word_map {
            let candidate_count = *candidate_map.get(&c).unwrap_or(&0usize);

            if count == &candidate_count {
                continue;
            }

            let candidate_count = *candidate_map
                .get(&c.to_ascii_lowercase())
                .unwrap_or(&0usize);

            if count == &candidate_count {
                continue;
            }

            let candidate_count = *candidate_map
                .get(&c.to_ascii_uppercase())
                .unwrap_or(&0usize);

            if count == &candidate_count {
                continue;
            }

            is_anagram = false;
        }

        if is_anagram {
            result.insert(candidate);
        }
    } // O(N*S)

    result
}

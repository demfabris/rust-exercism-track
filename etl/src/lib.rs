use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (idx, chars) in h {
        for c in chars.iter() {
            result.insert(c.to_ascii_lowercase(), *idx);
        }
    }

    result
}

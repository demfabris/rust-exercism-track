pub fn find(values: &[i32], key: i32) -> Option<usize> {
    let clone = values;
    let mut values = values;

    if values.len() == 0 {
        return None;
    }

    while values.len() > 1 {
        let half_index = values.len() / 2;
        let mid = values[half_index];
        println!("half_index: {}, mid: {}", half_index, mid);

        if mid == key {
            return Some(clone.iter().position(|&x| x == mid).unwrap());
        } else if mid > key {
            values = values.split_at(half_index).0;
            println!("new arr: {:?}", values);
        } else if mid < key {
            values = values.split_at(half_index).1;
            println!("new arr: {:?}", values);
        }
    }

    if values[0] == key {
        return Some(clone.iter().position(|&x| x == key).unwrap());
    } else {
        None
    }
}

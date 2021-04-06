pub fn build_proverb(list: &[&str]) -> String {
    let mut iter = list.iter().peekable();
    let mut result = String::from("");

    if list.len() == 0 {
        return result
    } 

    for _ in 0..list.len() - 1 {
        result.push_str(&format!("For want of a {} the {} was lost.\n", iter.next().unwrap(), iter.peek().unwrap())[..]);
    }

    result.push_str(&format!("And all for the want of a {}.", list[0])[..]);

    println!("{}", result);

    result
}

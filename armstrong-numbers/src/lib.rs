pub fn is_armstrong_number(num: u32) -> bool {
    let numbers = num
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<u32>>();
    let mut result = 0;

    for n in numbers.iter() {
        result = result + n.pow(numbers.len() as u32);
    }

    result == num
}

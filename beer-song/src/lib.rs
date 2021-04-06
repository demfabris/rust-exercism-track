use std::env;

pub fn middle_sentence(n: u32) -> String {
    if n > 0 {
        return format!("Take {} down and pass it around", if n > 1 { "one" } else { "it" });
    } else {
        return String::from("Go to the store and buy some more");
    }
}

pub fn bottles(n: u32, capitalize: bool) -> String {
    if n > 0 {
        format!("{} bottle{}", n, if n > 1 { "s" } else { "" })
    } else {
        format!("{}o more bottles", if capitalize { "N" } else { "n" })
    }
}

pub fn verse(n: u32) -> String {
    if n > 0 {
        return format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n", bottles(n, false), bottles(n, false), middle_sentence(n), bottles(n-1, false));
    } else {
        return format!("{} of beer on the wall, {} of beer.\n{}, 99 bottles of beer on the wall.\n", bottles(n, true), bottles(n, false), middle_sentence(n));
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::from("");
    for i in (end..=start).rev() {
        result.push_str(&verse(i)[..]);
        if i != end {
            result.push_str("\n");
        }
    }

    result
}

pub fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        panic!("usage: ./lib <number>");
    }

    let number = args[1].parse::<u32>().unwrap();

    println!("{}", sing(number + 1, 0));
}

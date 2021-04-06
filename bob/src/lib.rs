pub fn is_yell(msg: &str) -> bool {
    msg.chars()
        .filter(|c| c.is_alphabetic() && c != &' ')
        .all(|c| c.is_uppercase())
}

pub fn is_question(msg: &str) -> bool {
    msg.chars().filter(|c| c != &' ').last().unwrap() == '?'
}

pub fn is_blank(msg: &str) -> bool {
    msg.chars().all(|c| !c.is_digit(10) && !c.is_alphabetic())
}

pub fn reply(message: &str) -> &str {
    if is_blank(message) {
        return "Fine. Be that way!";
    } else {
        if is_yell(message) {
            if is_question(message) {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Whoa, chill out!";
            }
        } else {
            if is_question(message) {
                return "Sure.";
            } else {
                return "Whatever.";
            }
        }
    }
}

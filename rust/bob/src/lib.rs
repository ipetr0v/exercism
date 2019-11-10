fn has_letters(message: &str) -> bool {
    message.chars()
           .any(|x| x.is_alphabetic())
}

fn is_a_question(message: &str) -> bool {
    message.chars()
           .rev()
           .skip_while(|ch| ch.is_whitespace())
           .nth(0)
           .map_or(false, |ch| ch == '?')
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && has_letters(message)
}

fn is_nothing(message: &str) -> bool {
    message.chars()
           .all(|x| !x.is_alphanumeric())
}

pub fn reply(message: &str) -> &str {
    match (is_a_question(message), is_yelling(message), is_nothing(message)) {
        (true, false, _) => "Sure.",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever."
    }
}

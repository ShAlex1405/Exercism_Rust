pub fn reply(message: &str) -> &str {
    let m = message.trim();
    let has_letters = m.chars().any(|c| c.is_alphabetic());
    let scream = m.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());
    let is_question = m.chars().last() == Some('?');
    let empty = m.trim().is_empty();
    
    if scream && is_question && has_letters { return "Calm down, I know what I'm doing!"; }
    if is_question { return "Sure."; }
    if scream && has_letters { return "Whoa, chill out!"; }
    if empty { return "Fine. Be that way!"; }
    
    "Whatever."
}
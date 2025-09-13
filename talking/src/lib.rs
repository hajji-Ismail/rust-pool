pub fn talking(text: &str) -> &str {
    if text.len() == 0 {
        return "Just say something!";
    } else if text.chars().nth(text.len() - 1).unwrap() == '?' {
        if text
            .chars()
            .filter(|x| x.is_alphabetic())
            .all(|x| x.is_uppercase())
        {
            return "Quiet, I am thinking!";
        } else {
            return "Sure.";
        }
    } else if text.chars().nth(text.len() - 1).unwrap() != '?' {
        return "There is no need to yell, calm down!";
    } else {
        "Interesting"
    }
}

use regex::Regex;

pub fn reply(message: &str) -> &str {
    let treated_message = &message.replace("\t", "");
    let treated_message = treated_message.trim();
    match treated_message {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.to_uppercase() == m
            && m.get(m.len() - 1..).unwrap() == "?"
            && Regex::new(r"[a-zA-Z]").unwrap().is_match(&m) =>
        {
            "Calm down, I know what I'm doing!"
        }
        m if m.get(m.len() - 1..).unwrap() == "?" => "Sure.",
        m if m.to_uppercase() == m && Regex::new(r"[a-zA-Z]").unwrap().is_match(&m) => {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}

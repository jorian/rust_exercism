pub fn reply(message: &str) -> &str {
//    unimplemented!("have Bob reply to the incoming message: {}", message)

    match message.trim() {
        x if x.is_empty() => "Fine. Be that way!",
        x if x.ends_with("?") &&
            x.to_ascii_uppercase() == message &&
            x.to_ascii_lowercase() != message => "Calm down, I know what I'm doing!",
        x if x.to_ascii_uppercase() == message &&
            x.to_ascii_lowercase() != message => "Whoa, chill out!",
        x if x.ends_with("?") => "Sure.",
        _ => "Whatever."
    }
}


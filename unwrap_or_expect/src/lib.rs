use std::fmt::format;

#[derive(Debug)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    // todo!()
    match (server, security_level) {
      (Ok(url),Security::UnexpectedUrl) => panic!("{url}"),
      (Ok(url), _) => url.to_string(),
        (Err(e), Security::Unknown) => {
            panic!("called `Result::unwrap()` on an `Err` value: \"{e}\"")
        }
        (Err(""), Security::Message) => panic!("ERROR: program stops"),
        (Err(""), Security::Warning) =>format!("WARNING: check the server"),
        (Err(e), Security::Message) => panic!("Message: {}", e),
        (Err(e), Security::Warning) => format!("Warning: {}", e),
        (Err(e), Security::NotFound) => format!("Not found: {}", e),
        (Err(e), Security::UnexpectedUrl) => panic!("UnexpectedUrl: {}", e),
        (Err(e), _) => e.to_string(),
    }
}

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
  match(  server , security_level ) {
    (Ok(e) , _) =>  e.to_string(),
    (Err(e),Security::Unknown ) => format!("Unknown: {}", e),
    (Err(e),Security::Message ) => panic!("Message: {}", e),
    (Err(e),Security::Warning ) =>  format!("Warning: {}", e),
    (Err(e),Security::NotFound ) =>  format!("NotFound: {}", e),
    (Err(e),Security::UnexpectedUrl ) =>  format!("UnexpectedUrl: {}", e),
    (Err(""),Security::Message ) => panic!("Unknown:EROR: Programe Stops", ),
      
  }
}
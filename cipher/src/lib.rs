


#[derive(Debug, PartialEq)]
pub struct CipherError {
    expected : String
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {

let ciph = original.chars().map(|c|{
  match c {
        'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
        'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
        _ => c,
    }
}).collect::<String>();

if ciph == ciphered.to_string() {
    return Ok(());
}
Err(CipherError { expected: ciph })






}



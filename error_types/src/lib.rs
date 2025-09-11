// this will be the structure that wil handle the errors

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (& 'static str, String),
    pub date: String,
    pub err: & 'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError{form_values :(field_name, field_value) , err: err , date : chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()}
      
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let data = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        if self.name.len() == 0 {
            return Err(FormError {
                err: "Username is empty",
                form_values: ("name", self.name.to_string()),
                date:data,
            });
        }
        if self.password.len() < 8 {
            return Err(FormError {
                err: "Password should be at least 8 characters long",
                form_values: ("password", self.password.to_string()),
                date:data,
            });
        }
        let digit = self.password.chars().any(|c|{
            c.is_ascii_digit()
        });
        let letter =self.password.chars().any(|c|{
            c.is_ascii_alphabetic()
        });
            let symbol =self.password.chars().any(|c|{
            c as u8 >= 0x24 && c as u8 <= 0x2F || c.is_ascii_punctuation()
        });
if !(digit&&letter&& symbol){
        return Err(FormError {
                err: "Password should be a combination of ASCII numbers, letters and symbols",
                form_values: ("password", self.password.to_string()),
                date: data,
            });
}
Ok(())
      
    }
}

#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}

pub trait AppendStr {
    fn append_str(&mut self, str_to_append: String) -> Self;

    fn append_number(&mut self, nb_to_append: f64) -> Self;

    fn remove_punctuation_marks(&mut self) -> Self;
}

impl AppendStr for StringValue {
    fn append_str(&mut self, str_to_append: String) -> Self {
        let value = self.value.clone() + &str_to_append;
        self.value = value.clone();
        StringValue { value }
    }
   fn append_number(&mut self, nb_to_append: f64) -> Self {

    let str_to_append = nb_to_append.to_string();

          let value = self.value.clone() + &str_to_append;
        self.value = value.clone();
        StringValue { value }
    }
    fn remove_punctuation_marks(&mut self) -> Self {
        let mut value = String::new();
        for c in self.value.chars() {
            if !c.is_ascii_punctuation() && c != '-' {
                value.push(c);
            }
        }
        self.value = value.clone();
        StringValue { value }
    }
}

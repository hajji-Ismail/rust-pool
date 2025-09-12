use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    if str1.to_lowercase() == str1.to_lowercase().to_case(Case::Camel) || str1.to_lowercase()==str1.to_lowercase().to_case(Case::Snake){
       

        let steps = edit_distance(&str1.to_lowercase(), & str2.to_ascii_lowercase());
        let persent = ((str2.len() - steps) as f64 / str2.len() as f64) * 100.0;

        if persent > 50.0 {
             Some(format!("{}%", (persent).round() ))
        } else {
            None
        }

    
    } else {
         None
    }
}

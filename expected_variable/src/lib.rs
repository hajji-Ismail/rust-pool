use edit_distance::edit_distance;
use convert_case::{Case, Casing};

pub fn expected_variable(str1: &str, str2: &str) -> Option<String> {
    if str1.to_lowercase() == str1.to_lowercase().to_case(Case::Camel) || str1.to_lowercase()==str1.to_lowercase().to_case(Case::Snake){
       

        let steps = edit_distance(&str1.to_lowercase(), & str2.to_ascii_lowercase());
 let percentage=((str2.len() as f64 -steps as f64)*100.0)/str2.len() as f64;
        if percentage > 50.0 {
             Some(format!("{}%", (percentage).round() ))
        } else {
            None
        }

    
    } else {
         None
    }
}

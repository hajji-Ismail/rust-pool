
pub fn capitalize_first(input: &str) -> String {
 let  sub: Vec<char> = input.chars().collect();
 let f = sub[0].to_uppercase();
 let res :String = f.to_string()+&sub[1..].iter().collect::<String>();
 res


}

pub fn title_case(input: &str) -> String {
    let mut res : String = "".to_string();
    let  words = input.split(" ");
    for i in words {
        let  sub : String;
        let  chars :  Vec<char> = i.chars().collect();
         let f = chars[0].to_uppercase();
        sub = f.to_string()+&chars[1..].iter().collect::<String>();
        res.push_str(&sub);
        res.push(' ');

    }
    res.trim().to_string()
 
}

pub fn change_case(input: &str) -> String {
    let mut res : String = "".to_string();
    for c in input.chars(){
        if c.is_uppercase() {
            let f =c.to_lowercase();
           
            res +=&f.to_string();
        } else {
            let f =c.to_uppercase();
           res +=&f.to_string();
        }
    }
    res 
}
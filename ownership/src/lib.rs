pub fn first_subword(mut s: String) -> String {
  
    let words : Vec::<char> = s.chars().collect();
    let mut  res : String = "".to_string() ;
    for i in 1..words.len(){
        if words[i]== '_'  || words[i].is_uppercase(){
            res = s[0..i].to_string();
            break;

           
        }

    }
    if res.is_empty(){
        return s
    }
     res
}
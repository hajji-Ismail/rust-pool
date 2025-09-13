pub fn is_pangram(s: &str) -> bool {
    let mut nmb = 0 ;
     for c in s.chars() {

     let ch: char = c.to_ascii_lowercase();
    match ch {
        'a'|'b'|'c'| 'd'| 'e'| 'f'| 'j'| 'h'| 'i'|'g' |'k' | 'l' | 'm' | 'n' |'o' |'p' |'q' |'r' |'s' |'t' |'u'|'v'|'w' |'x'| 'y'| 'z'=> nmb+= 1 ,
     
       _ => nmb += 0,

    }
   }
   if nmb < 26 {
    false
   } else {
    true
   }
   
}
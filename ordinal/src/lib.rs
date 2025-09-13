pub fn num_to_ordinal(x: u32) -> String {
    let n = x % 10 ; 
    
    match n  {
        1 => format!("{x}st"),
        2 =>  format!("{x}nd"),
        3 =>  format!("{x}rd"),
        _ =>  format!("{x}th"),


    }

}
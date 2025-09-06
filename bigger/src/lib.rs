use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut res: i32 = 0 ;
    for (_, v) in h {
        if v > res {
            res = v ;
        }
    }
    res
}
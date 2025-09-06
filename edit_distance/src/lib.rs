pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut res : usize = 0;
    if source < target {
        res += target.len()- source.len()
    } else {
        res += source.len()- target.len()
    }
    let mut plus = false;
    for c1 in source.chars() {
        for c2 in target.chars(){
           
            if  c1 == c2 {
                plus = true;
                break;
            }

        }
        if plus == false {
            res += 1;
        }else {
            plus = false
        }
    }



    res

}
pub fn first_fifty_even_square() -> Vec<i32> {
    let mut  i = 1;
    let mut res:Vec<i32>  =  vec![];
        while i <=50 {
            
            res.push(i*2*i*2);

            i += 1;
            

    }
res
}
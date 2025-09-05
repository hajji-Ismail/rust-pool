pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let x : f64 = c as f64;
    (c,x.exp(), x.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut res : String = Default::default() ;
for item in a.split(" "){
    let  conv = item.parse::<f64>().unwrap();
    let  nla = conv.exp().to_string();
    res.push_str(&(nla + " "))

}


    (a,res.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut res:Vec<f64> = vec![];
    for item in &b {
        let conv: f64 = *item as f64;
        res.push(conv.abs().ln())

    }
    (b,res)
}
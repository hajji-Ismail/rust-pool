pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut  splited = s.split(" ");
    let mut res : Vec<Box<u32>>= vec![];
    for c in splited {
        print!("{c}");

        if c.ends_with("k"){
           let h =  c.strip_suffix("k");
            res.push(Box::new((h.expect("REASON").parse::<f64>().unwrap()*1000.0) as u32));
        } else {
             res.push(Box::new((c.parse::<f64>().unwrap()) as u32));
        }
    }
 res
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut res :Vec<u32> =  vec![];
    for n in a {
        res.push(*n);
    }


    res 
}
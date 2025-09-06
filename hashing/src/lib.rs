 use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
    let length = list.len() as f64;
    let mut sum: i32 = 0;
    for i in list {
        sum += i;
    }
    let total = sum as f64;
    total / length
}

pub fn median(list: &[i32]) -> i32 {
    let  res: i32;

    let mut vec_list = list.to_vec();
    vec_list.sort();

    if vec_list.len() % 2 == 0 {
        res = (vec_list[vec_list.len() / 2] + vec_list[vec_list.len() / 2 -1] ) /2  ;
        
    } else {
        res = vec_list[vec_list.len() / 2];

        
    }
    res
}

pub fn mode(list: &[i32]) -> i32 {
      let mut map = HashMap::new();
    for num in list {
        if map.contains_key(num){
            let old = map.get(num).unwrap();
            map.insert(*num, old+1);
        } else {
            map.insert(*num, 1);
        }
    }
    

       let mut res: i32 =0;
       let mut val : i32= 0 ;
       println!("{:?}", map);
    for (c, v) in map {
        if v > val {
            res = c ;
            val = v;
        }
    }
    res
}


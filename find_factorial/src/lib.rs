pub fn factorial(num: u64) -> u64 {
    if num == 1 || num == 0 {
        return 1;
    }
    let mut  res = 1 ;
    for i in 1..num {
        res = res * i;

    }
    res 
}

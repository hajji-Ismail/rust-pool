pub fn fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2)
  

  
}

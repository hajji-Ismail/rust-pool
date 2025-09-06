use hashing::*;

fn main() {
    let mut  v = [4, 7, 2, 5,5, 1, 3];

    println!("mean {}", mean(&v));
    println!("median {}", median(&mut v));
    println!("mode {}", mode(&v));
}
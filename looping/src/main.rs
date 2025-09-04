use std::io;
fn main() {
    let mut tries = 0;
    loop {
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        tries += 1;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "The letter e" {
            break;
        }
    }
    println!("Number of trials: {}", tries);
}

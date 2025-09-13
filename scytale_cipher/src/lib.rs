pub fn scytale_cipher(message: &str, i: usize) -> String {
    if message.is_empty() {
        return "".to_string();
    }
    let mut str = message.to_string();
    let remainder = message.len() % i;
    if remainder != 0 {
        str += &" ".repeat(i - remainder);
    }

    let mut grid = vec![String::new(); i];

    for (index, c) in str.chars().enumerate() {
        let col = index % i;
        grid[col].push(c);
    }

    grid.concat().trim().to_string()
}

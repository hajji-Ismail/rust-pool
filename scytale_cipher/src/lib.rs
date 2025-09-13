pub fn scytale_cipher(message: &str, i: usize) -> String {
    
 let mut str = message.to_string() ; 
 str = str + &" ".repeat(message.len()%i).to_string() ; 
  


    let mut grid = vec![String::new();i];

    for (index, c) in str.chars().enumerate() {
 
        let col = index % i;
        grid[col].push(c);
    }

    grid.concat().trim().to_string()
}

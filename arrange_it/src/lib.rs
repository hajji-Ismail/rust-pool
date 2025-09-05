pub fn arrange_phrase(phrase: &str) -> String {
    let words = phrase.split(" ");
    let mut res: String = "".to_string();

    for i in 1..words.clone().count()+1 {
        res = res + &i.to_string()
    }
    println!("{}", res);
    for (_, word) in words.enumerate() {
        for (i, c) in word.chars().enumerate() {
            let mut sub ="".to_string();     
            if c.is_numeric() {
                let number = c.to_string().parse::<u8>().unwrap();

                if i == 0 {
                    sub = word[i + 1..].to_string() + &" ".to_string();
                }
                if i < word.len() {
                    sub = word[0..i].to_string() + &word[i + 1..].to_string() + & " ".to_string();
                } else {
                    sub = word[0..i].to_string()+ &" ".to_string();
                }
                res = res.replace(&number .to_string(), &sub );
                sub.clear();
            }
        }
    }

    res.trim().to_string()
}

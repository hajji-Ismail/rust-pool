pub fn spell(n: u64) -> String {
    if n ==  0 {
        return "zero".to_string();

    }
     let n_str = n.to_string();
    let chars: Vec<char> = n_str.chars().collect();
    let mut arr_of_str_num: Vec<String> = vec![];
    let mut number_names: Vec<String> = vec![];
    let mut res: String = Default::default();
    let num_len = n_str.len();


    for c in &chars {
        match c {
            '0' => arr_of_str_num.push("zero ".to_string()),
            '1' => arr_of_str_num.push("one ".to_string()),
            '2' => arr_of_str_num.push("two ".to_string()),
            '3' => arr_of_str_num.push("three ".to_string()),
            '4' => arr_of_str_num.push("four ".to_string()),
            '5' => arr_of_str_num.push("five ".to_string()),
            '6' => arr_of_str_num.push("six ".to_string()),
            '7' => arr_of_str_num.push("seven ".to_string()),
            '8' => arr_of_str_num.push("eight ".to_string()),
            '9' => arr_of_str_num.push("nine ".to_string()),
            _ => {}
        }
    }


    for i in 0..num_len {
        let pos = num_len - i;

        let label = match pos {
            1 => "".to_string(),               
            2 => "ten ".to_string(),           
            3 => "hundred ".to_string(),       
            4 => "thousand ".to_string(),
            5 => "ten ".to_string(),
            6 => "hundred ".to_string(),
            7 => "million ".to_string(),
            _ => "nada".to_string(),
        };

        number_names.push(label);
    }


    let teens = [
        "ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty",
        "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut i = 0;
    while i < arr_of_str_num.len() {
        let digit_char = chars[i];
        let digit = digit_char.to_digit(10).unwrap();

        let place = number_names[i].as_str();

        // Handle tens
        if place == "ten " {
            // Look ahead to the next digit (unit)
            if i + 1 < arr_of_str_num.len() {
                let next_digit_char = chars[i + 1];
                let next_digit = next_digit_char.to_digit(10).unwrap();

                if digit == 1 {
                    // Handle teens
                    res.push_str(teens[next_digit as usize]);
                    res.push(' ');

                    i += 2; 
                    continue;
                } else {
                  
                    res.push_str(tens[digit as usize]);
                    if next_digit != 0 {
                        res.push('-');
                        res.push_str(&arr_of_str_num[i + 1]);
                    } else {
                        res.push(' ');
                    }

                    i += 2; 
                    continue;
                }
            } else {
                res.push_str("ten ");
                i += 1;
                continue;
            }
        }

        if arr_of_str_num[i] == "zero ".to_string() {
            i += 1;
            continue;
        }

        // Append digit word + place name
        res.push_str(&arr_of_str_num[i]);
        res.push_str(&number_names[i]);
        i += 1;
    }

    println!("{:?}", arr_of_str_num);
    println!("{:?}", number_names);
    res.trim().to_string()
}

pub fn delete_and_backspace(s: &mut String) {
    let mut res: String = "".to_string();
    let mut count = 0;
    for c in s.clone().chars() {
        if c == '-' {
            res.pop();
        }
        if c == '+' {
            count += 1;
        }
        if count == 0 && c != '+' && c != '-' {
            res.push(c);
        }
        if count != 0 && c != '+' && c != '-' {
            count -= 1
        }
        *s = res.clone()
    }
}

pub fn do_operations(v: &mut [String]) {
    let mut res: Vec<String> = vec![];
    for c in &mut * v {
        if c.contains("+") {
            let parts: Vec<&str> = c.split('+').collect();
            let a = parts[0].parse::<f64>().unwrap();
            let b = parts[1].parse::<f64>().unwrap();
            res.push((a + b).to_string());
        }
        if c.contains("-") {
            let parts: Vec<&str> = c.split('-').collect();
            let a = parts[0].parse::<f64>().unwrap();
            let b = parts[1].parse::<f64>().unwrap();
            res.push((a - b).to_string());
        }
        if c.contains("*") {
            let parts: Vec<&str> = c.split('*').collect();
            let a = parts[0].parse::<f64>().unwrap();
            let b = parts[1].parse::<f64>().unwrap();
            res.push((a * b).to_string());
        }
        if c.contains("/") {
            let parts: Vec<&str> = c.split('/').collect();
            let a = parts[0].parse::<f64>().unwrap();
            let b = parts[1].parse::<f64>().unwrap();
            res.push((a / b).to_string());
        }
        if c.contains("%") {
            let parts: Vec<&str> = c.split('%').collect();
            let a = parts[0].parse::<f64>().unwrap();
            let b = parts[1].parse::<f64>().unwrap();
            res.push((a % b).to_string());
        }
    }
       for (i, result) in res.into_iter().enumerate() {
      
            v[i] = result;
        
    }
}

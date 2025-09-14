pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen",
        "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let thousands = ["", "thousand", "million", "billion", "trillion"];

    // Spell numbers below 1000
    fn spell_chunk(n: u64, below_20: &[&str], tens: &[&str]) -> String {
        let mut result = String::new();
        let hundred = n / 100;
        let rest = n % 100;

        if hundred > 0 {
            result.push_str(below_20[hundred as usize]);
            result.push_str(" hundred");
            if rest > 0 {
                result.push(' ');
            }
        }

        if rest >= 20 {
            let ten = rest / 10;
            let unit = rest % 10;
            result.push_str(tens[ten as usize]);
            if unit > 0 {
                result.push('-');
                result.push_str(below_20[unit as usize]);
            }
        } else if rest > 0 {
            result.push_str(below_20[rest as usize]);
        }

        result
    }

    let mut num = n;
    let mut result_parts: Vec<String> = vec![];
    let mut chunk_index = 0;

    while num > 0 {
        let chunk = num % 1000;
        if chunk > 0 {
            let mut part = spell_chunk(chunk, &below_20, &tens);
            if !thousands[chunk_index].is_empty() {
                part.push(' ');
                part.push_str(thousands[chunk_index]);
            }
            result_parts.push(part);
        }
        num /= 1000;
        chunk_index += 1;
    }

    result_parts.reverse();
    result_parts.join(" ").trim().to_string()
}

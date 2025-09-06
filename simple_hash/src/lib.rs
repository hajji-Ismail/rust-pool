 use std::collections::HashMap;
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut res = HashMap::new();
    for word in words {
        if res.contains_key(word){
            let old = res.get(word).unwrap();
            res.insert(*word, old+1);
        } else {
            res.insert(word, 1);
        }
    }
    res
    

}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
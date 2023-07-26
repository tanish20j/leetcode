use::std::collections::HashMap;

fn word_grouping(word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash: HashMap<String, Vec<String>> = HashMap::new();
    let mut char_freq: Vec<i32> = vec![0; 26];

    for current_word in word_list {
        for c in current_word.to_lowercase().chars() {
            let index = (c as u8 - 'a' as u8) as usize;
            char_freq[index] += 1;
        }
        let key:String = char_freq.into_iter().map(|i| i.to_string()).collect::<String>();
        word_hash.entry(key).or_insert(Vec::new()).push(current_word);

        char_freq = vec![0; 26];
    }
    
    for (key, value) in &word_hash {
        println!("{}: {:?}", key, value);
    }

    word_hash.into_iter().map(|(_, value)| value).collect::<Vec<Vec<String>>>()


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words: Vec<String> = vec!["the".to_string(), "teh".to_string(), "het".to_string(),
         "stupid".to_string(), "stupdi".to_string(),"apple".to_string(), "appel".to_string()];

        let grouping = word_grouping(words);
        print!("Grouping {:?}", grouping);
        let input_word = "the".to_string();

        for i in grouping.into_iter() {
            if i.contains(&input_word) {
                println!("{:?}", i);
            }
        }
    }
}
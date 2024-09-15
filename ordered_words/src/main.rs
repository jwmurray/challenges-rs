use ordered_words::return_ordered_words;
use std::collections::HashMap;

fn main() {
    println!(
        "Current workingn directory: {:?}",
        std::env::current_dir().unwrap()
    );
    let string_list = read_word_list("unixdict.txt");
    let word_list: Vec<&str> = string_list.iter().map(|s| s.as_str()).collect();
    let ordered_words = return_ordered_words(word_list);
    let mut dict_words: HashMap<usize, Vec<&str>> = HashMap::new();
    let max_len = ordered_words.iter().map(|s| s.len()).max().unwrap();
    for word in ordered_words {
        dict_words
            .entry(word.len())
            .or_insert_with(Vec::new)
            .push(word);
    }

    println!("{:?}", dict_words[&max_len]);
}

fn read_word_list(filepath: &str) -> Vec<String> {
    let file = std::fs::read_to_string(filepath).expect("Failed to read file");
    let word_list: Vec<String> = file.lines().map(|line| line.to_string()).collect();
    word_list
}

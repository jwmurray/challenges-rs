use std::fs::File;
use std::io::prelude::*;

pub fn get_word_iterator_from_file(
    file_path: &str,
) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let words = reader.lines().flat_map(|line| {
        line.ok()
            .map(|l| {
                l.split_whitespace()
                    .map(|word| word.to_lowercase())
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .flatten()
    });
    Ok(words)
}

fn get_word_frequency(
    words: impl Iterator<Item = String>,
) -> std::collections::HashMap<String, u32> {
    let mut word_freq = std::collections::HashMap::new();
    for word in words {
        let count = word_freq.entry(word).or_insert(0);
        *count += 1;
    }
    word_freq
}

fn get_sorted_word_frequency(
    word_freq: std::collections::HashMap<String, u32>,
    take: usize,
) -> Vec<(String, u32)> {
    let mut word_freq_vec: Vec<_> = word_freq.into_iter().collect();
    word_freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    word_freq_vec = word_freq_vec.into_iter().take(take).collect();
    word_freq_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_iterator_from_file() {
        let file_path = "lesmis.txt";
        let mut words = get_word_iterator_from_file(file_path).unwrap();
        assert_eq!(words.next(), Some("the".to_string()));
        assert_eq!(words.next(), Some("project".to_string()));
        assert_eq!(words.next(), Some("gutenberg".to_string()));
        // let words: Vec<String> = words.collect();
        // assert_eq!(words.len(), 235886);
    }

    #[test]
    fn test_get_word_frequency() {
        let file_path = "lesmis.txt";
        let words = get_word_iterator_from_file(file_path).unwrap();
        let word_freq = get_word_frequency(words);
        assert_eq!(word_freq.get("the"), Some(&40379));
        assert_eq!(word_freq.get("project"), Some(&87));
        assert_eq!(word_freq.get("gutenberg"), Some(&25));
    }

    #[test]
    fn test_get_sorted_word_frequency() {
        let file_path = "lesmis.txt";
        let words = get_word_iterator_from_file(file_path).unwrap();
        let word_freq = get_word_frequency(words);
        let sorted_word_freq = get_sorted_word_frequency(word_freq, 10);
        assert_eq!(sorted_word_freq[0], ("the".to_string(), 40379));
        assert_eq!(sorted_word_freq[1], ("of".to_string(), 19869));
        assert_eq!(sorted_word_freq[2], ("and".to_string(), 14468));
        assert_eq!(sorted_word_freq[3].0, "a".to_string());
        assert_eq!(sorted_word_freq[4].0, "to".to_string());
        assert_eq!(sorted_word_freq[5].0, "in".to_string());
    }
}

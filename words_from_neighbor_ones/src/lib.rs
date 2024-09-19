// Task
// Use the dictionary   unixdict.txt

// Ignore any word in the dictionary whose length is less than 9.

// Let's take the words from next characters:
// 1 <= n < (dictionary length) - 9.
// char1 = 1st character of         nth   word.
// char2 = 2nd character of  (n+1)th  word.
// char3 = 3rd character of  (n+2)th  word.
//     ⋮
// char9 = 9th character of  (n+8)th  word.

// Concatenate (append) the nine characters by:

//       newword = char1 + char2 + char3 + ... + char9

// If   newword   is in the dictionary, then show on this page.

// Length of  newword = 9

use reqwest;

struct CollectedWordIterator<'a> {
    words: &'a [String],
    dictionary: &'a [String],
    index: usize,
}

impl<'a> CollectedWordIterator<'a> {
    fn new(words: &'a [String], dictionary: &'a [String]) -> Self {
        CollectedWordIterator {
            words,
            dictionary,
            index: 0,
        }
    }

    fn is_word_in_dictionary(&self, word: &str) -> bool {
        self.dictionary.contains(&word.to_string().to_lowercase())
    }
}

impl Iterator for CollectedWordIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index + 9 <= self.words.len() {
            let word_strings: Vec<&str> = self.words[self.index..self.index + 9]
                .iter()
                .map(|s| s.as_str())
                .collect();

            let word: String = word_strings
                .iter()
                .enumerate()
                .map(|(i, s)| s.chars().nth(i).unwrap())
                .collect();
            self.index += 9;
            if self.is_word_in_dictionary(&word) {
                return Some(word);
            }
        }
        None
    }
}

struct WordIterator<'a> {
    words: &'a [String],
    index: usize,
}

impl<'a> WordIterator<'a> {
    fn new(words: &'a [String]) -> Self {
        WordIterator { words, index: 0 }
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.words.len() {
            let word = self.words[self.index].clone();
            self.index += 1;
            Some(word)
        } else {
            None
        }
    }
}

pub fn get_dictionary_words_gte_nine(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cache_file = "unixdict.txt";

    if std::fs::metadata(cache_file).is_ok() {
        let cache_content = std::fs::read_to_string(cache_file)?;
        return Ok(cache_content.lines().map(|line| line.to_string()).collect());
    } else {
        let body = reqwest::blocking::get(url)?.text()?;
        let words: Vec<String> = body
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|word| word.len() >= 9)
            .collect();
        // write to cache file

        std::fs::write(cache_file, &words.join("\n"))?;
        Ok(words)
    }
}

pub fn get_random_word(words: &[String], n: usize) -> String {
    let len = words.len();
    assert_eq!(len, n);
    let random_index = rand::random::<usize>() % len;
    words[random_index].clone()
}

pub fn get_all_collected_words() -> Vec<String> {
    let url = "https://web.archive.org/web/20180611003215/http://www.puzzlers.org/pub/wordlists/unixdict.txt";
    let dictionary = get_dictionary_words_gte_nine(url).unwrap();
    let collected_word_iterator = CollectedWordIterator::new(&dictionary, &dictionary);
    collected_word_iterator.collect()
}

fn is_word_in_dictionary(dictionary: &[String], word: &str) -> bool {
    dictionary.contains(&word.to_string().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_new_word() {
        let word = "challengeadventurebeautifulchallenge";
        let words: Vec<String> = word.chars().map(|c| c.to_string().repeat(9)).collect();
        assert_eq!(
            words,
            vec![
                "ccccccccc",
                "hhhhhhhhh",
                "aaaaaaaaa",
                "lllllllll",
                "lllllllll",
                "eeeeeeeee",
                "nnnnnnnnn",
                "ggggggggg",
                "eeeeeeeee",
                "aaaaaaaaa",
                "ddddddddd",
                "vvvvvvvvv",
                "eeeeeeeee",
                "nnnnnnnnn",
                "ttttttttt",
                "uuuuuuuuu",
                "rrrrrrrrr",
                "eeeeeeeee",
                "bbbbbbbbb",
                "eeeeeeeee",
                "aaaaaaaaa",
                "uuuuuuuuu",
                "ttttttttt",
                "iiiiiiiii",
                "fffffffff",
                "uuuuuuuuu",
                "lllllllll",
                "ccccccccc",
                "hhhhhhhhh",
                "aaaaaaaaa",
                "lllllllll",
                "lllllllll",
                "eeeeeeeee",
                "nnnnnnnnn",
                "ggggggggg",
                "eeeeeeeee"
            ]
        );

        let url = "https://web.archive.org/web/20180611003215/http://www.puzzlers.org/pub/wordlists/unixdict.txt";
        let dictionary = get_dictionary_words_gte_nine(url).unwrap();
        let mut collected_word_iterator = CollectedWordIterator::new(&words, &dictionary);
        assert_eq!(
            collected_word_iterator.next(),
            Some("Challenge".to_string().to_lowercase())
        );
        assert_eq!(
            collected_word_iterator.next(),
            Some("Adventure".to_string().to_lowercase())
        );
        assert_eq!(
            collected_word_iterator.next(),
            Some("Beautiful".to_string().to_lowercase())
        );
        assert_eq!(
            collected_word_iterator.next(),
            Some("Challenge".to_string().to_lowercase())
        );
        assert_eq!(collected_word_iterator.next(), None);
    }
    #[test]
    fn test_get_next_new_word_with_big_dictionary() {
        let url = "https://web.archive.org/web/20180611003215/http://www.puzzlers.org/pub/wordlists/unixdict.txt";
        let dictionary_words = get_dictionary_words_gte_nine(url).unwrap();

        let mut collected_word_iterator =
            CollectedWordIterator::new(&dictionary_words, &dictionary_words);

        let word_option = collected_word_iterator.next();
        let word = word_option.unwrap();
        assert!(is_word_in_dictionary(&dictionary_words, &word));
    }

    #[test]
    fn test_get_next_word() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut word_iterator = WordIterator::new(&words);
        assert_eq!(word_iterator.next(), Some("hello".to_string()));
        assert_eq!(word_iterator.next(), Some("world".to_string()));
        assert_eq!(word_iterator.next(), None);
    }

    #[test]
    fn test_get_dictionary_words_gte_nine() {
        let url = "https://web.archive.org/web/20180611003215/http://www.puzzlers.org/pub/wordlists/unixdict.txt";
        let words = get_dictionary_words_gte_nine(url).unwrap();
        assert!(words.len() >= 2);
        assert!(words.iter().all(|word| word.len() >= 9));
        let cache_file = "unixdict.txt";
        let cache_result = std::fs::read_to_string(cache_file); // save the cache content if the file exists.
        let test_content = "yellow-bellied\npickled-herring\nbottlenose";
        std::fs::write(cache_file, test_content).unwrap(); // save the test content to the cache file.

        let words = get_dictionary_words_gte_nine("http://cisco.com").unwrap();
        assert_eq!(
            words,
            vec!["yellow-bellied", "pickled-herring", "bottlenose"]
        );

        // restore the file if it was there before.
        match cache_result {
            Ok(content) => {
                std::fs::write(cache_file, content).unwrap(); // restore the cache content.
            }
            _ => (), // do nothing as the file never existed.
        }
    }

    #[test]
    fn test_get_random_word() {
        let url = "https://web.archive.org/web/20180611003215/http://www.puzzlers.org/pub/wordlists/unixdict.txt";
        let words = get_dictionary_words_gte_nine(url).unwrap();
        (0..1000).for_each(|_| {
            let random_word = get_random_word(&words, words.len());
            assert!(random_word.len() >= 9);
        })
    }

    #[test]
    fn test_write_vector_to_file() {
        let filename = "test.txt";
        let words = vec!["hello".to_string(), "world".to_string()];
        std::fs::write(filename, words.join("\n")).unwrap();
        let content = std::fs::read_to_string(filename).unwrap();
        assert_eq!(content, "hello\nworld");
        std::fs::remove_file(filename).unwrap();
    }
}

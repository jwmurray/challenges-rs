pub fn return_ordered_words(word_list: Vec<&str>) -> Vec<&str> {
    let mut ordered_words: Vec<&str> = Vec::new();

    for word in word_list {
        if word.is_ordered() {
            ordered_words.push(word);
        }
    }
    ordered_words
}

trait IsOrdered {
    fn is_ordered(&self) -> bool;
}

impl IsOrdered for &str {
    fn is_ordered(&self) -> bool {
        let mut last_char = 'a';
        for c in self.chars() {
            if !c.is_alphabetic() {
                return false;
            }
            if c < last_char {
                return false;
            }
            last_char = c;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_apple() {
        let word: &str = "apple";
        assert_eq!(word.is_ordered(), false);
    }

    #[test]
    fn test_word_dirt() {
        let word: &str = "dirt";
        assert_eq!(word.is_ordered(), true);
    }

    #[test]
    fn test_word_78dirt() {
        let word: &str = "78dirt";
        assert_eq!(word.is_ordered(), false);
    }

    #[test]
    fn test_word_list_dirt() {
        let word_list = vec!["dirt"];
        let ordered_words = return_ordered_words(word_list);
        assert_eq!(ordered_words, vec!["dirt"]);
    }

    #[test]
    fn test_return_ordered_words() {
        let word_list = vec![
            "abbey",
            "dirt",
            "apple",
            "banana",
            "cherry",
            "date",
            "elderberry",
        ];
        let ordered_words = return_ordered_words(word_list);
        assert_eq!(ordered_words, vec!["abbey", "dirt"]);
    }
}

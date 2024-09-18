use std::collections::HashMap;
use std::time::Instant;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let scrambles = [
        "eeersn",
        "useiohd",
        "edsdnceonc",
        "ethlosoma",
        "tahlgys",
        "cpveteprei",
        "earnthorb",
    ];

    let word_list = get_english_words();
    let dictionary: HashMap<String, _> = word_list.iter().map(|s| (s.clone(), true)).collect();
    for scramble in scrambles {
        println!("Working on scramble: {}", scramble);
        let words = get_words_from_scramble(scramble, &dictionary);
        println!("Scramble: {}, words: {:?}", scramble, words);
    }

    // some notes on graphemes to understand how strings utf-8, chars() and the grapheme trait works.
    // let hello_str = "Helloy̆, world!";
    // let y_str = "y̆";

    // let y_graphemes = graphemes_from_str(y_str);
    // let hello_graphemes = graphemes_from_str(hello_str);
    // for grapheme in hello_graphemes {
    //     println!("{}", grapheme);
    // }
    // println!(
    //     "{}, str.len(): {} len: {}, y_graphemes.len(): {}",
    //     y_str,
    //     y_str.len(),
    //     y_str.chars().count(),
    //     y_graphemes.len()
    // );

    // for c in y_str.chars() {
    //     println!("{}", c);
    // }

    // println!("y_graphemes.len(): {}", y_graphemes.len());
    // for (i, graph) in y_graphemes.into_iter().enumerate() {
    //     println!("{i} {graph}");
    // }
}

fn graphemes_from_str(s: &str) -> Vec<&str> {
    s.graphemes(true).collect()
}

fn get_words_from_scramble(scramble: &str, dictionary: &HashMap<String, bool>) -> Vec<String> {
    // Create a list of permutations of the scramble string and then
    // return a list of words that match the permutations.
    // Measure the time to generate permutations
    let start_permutations = Instant::now();
    let permutations = generate_grapheme_permutations(scramble);
    let duration_permutations = start_permutations.elapsed();
    println!(
        "\tTime to generate permutations for {}: {:?}",
        scramble, duration_permutations
    );

    // Measure the time to look up words
    let start_lookup = Instant::now();
    let mut result: Vec<String> = Vec::new();
    for permutation in permutations.iter() {
        if dictionary.contains_key(permutation) {
            result.push(permutation.clone());
        }
    }
    let duration_lookup = start_lookup.elapsed();
    println!(
        "\tTime to look up words in the dictionary for {}: {:?}, {:?}",
        scramble, duration_lookup, result
    );
    result
}

fn get_english_words() -> Vec<String> {
    // words_alpha.txt is a list of English words, one per line from https://github.com/dwyl/english-words/blob/master/words_alpha.txt
    let words = include_str!("words_alpha.txt");
    words.lines().map(|s| s.to_string()).collect()
}

fn generate_grapheme_permutations(seed: &str) -> Vec<String> {
    let mut string_dictionary_result: HashMap<String, _> = HashMap::new();

    let graphemes = graphemes_from_str(seed);

    // If str is zero or one grapheme long, then return that grapheme as the result.
    if graphemes.len() <= 1 {
        return graphemes.into_iter().map(|s| s.to_string()).collect();
    }

    for i in 0..graphemes.len() {
        let mut new_seed = graphemes.clone();
        new_seed.remove(i);
        let sub_permutations = generate_grapheme_permutations(&new_seed.concat());
        for sub_permutation in sub_permutations {
            string_dictionary_result.insert(graphemes[i].to_string() + &sub_permutation, true);
            // string_vector_result.push(graphemes[i].to_string() + &sub_permutation);
        }
    }

    let mut string_vector_result: Vec<String> = string_dictionary_result
        .keys()
        .cloned()
        .collect::<Vec<String>>();
    string_vector_result.sort();
    string_vector_result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_get_words_from_scramble() {
        let words = vec!["a".to_string(), "apple".to_string(), "cab".to_string()];
        let dictionary: HashMap<String, _> = words.iter().map(|s| (s.clone(), true)).collect();
        let scramble = "a";
        let result = get_words_from_scramble(scramble, &dictionary);
        assert_eq!(result, vec!["a"]);
        let scramble = "b";
        let result = get_words_from_scramble(scramble, &dictionary);
        assert_eq!(result, Vec::<&str>::new());
        let scramble = "abc";
        let result = get_words_from_scramble(scramble, &dictionary);
        assert_eq!(result, vec!["cab"]);

        let words = get_english_words();
        let dictionary: HashMap<String, _> = words.iter().map(|s| (s.clone(), true)).collect();
        let scramble = "eeersn";
        let result = get_words_from_scramble(scramble, &dictionary);
        assert_eq!(result, vec!["reseen", "resene", "serene"]);

        let words = get_english_words();
        let dictionary: HashMap<String, _> = words.iter().map(|s| (s.clone(), true)).collect();
        let scramble = "cpveteprei";
        let result = get_words_from_scramble(scramble, &dictionary);
        assert_eq!(result, vec!["perceptive", "preceptive"]);

        // Original design used a Vector for the dictionary, which was very slow.  By changing to a HashMap, the performance improved dramatically.
        // A trie might further improve the lookup performance over the HashMap.  See the increase of lookup time for the last test -- 77.9ms vs 1.8us.
        //
        // Further improvements should focus on the generate_grapheme_permutations function, which is the slowest part of the code now.
        //  to improve the performance of the generate_grapheme_permutations function, we could use a HashSet instead of a Vector to store the permutations.
        //  Sorting the input seed graphemes would allow us to skip duplicates, which would reduce the number of permutations generated.
        //
        //         output:
        //         running 1 test
        // test tests::test_get_words_from_scramble ... ok

        // successes:

        // ---- tests::test_get_words_from_scramble stdout ----
        //         Time to generate permutations for a: 1.958µs
        //         Time to look up words in the dictionary for a: 750ns, ["a"]
        //         Time to generate permutations for b: 833ns
        //         Time to look up words in the dictionary for b: 375ns, []
        //         Time to generate permutations for abc: 28.875µs
        //         Time to look up words in the dictionary for abc: 1.791µs, ["cab"]
        //         Time to generate permutations for eeersn: 4.035ms
        //         Time to look up words in the dictionary for eeersn: 31.916µs, ["reseen", "resene", "serene"]
        //         Time to generate permutations for cpveteprei: 30.95946125s
        //         Time to look up words in the dictionary for cpveteprei: 77.928583ms, ["perceptive", "preceptive"]

        // successes:
        //     tests::test_get_words_from_scramble

        // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 31.68s
    }

    #[test]
    fn test_generate_letter_permutations() {
        let result = generate_grapheme_permutations("feee");
        assert_eq!(result.len(), 4);
        assert_eq!(
            result,
            vec![
                "eeef".to_string(),
                "eefe".to_string(),
                "efee".to_string(),
                "feee".to_string()
            ]
        );
        let result = generate_grapheme_permutations("cpvete");
        assert_eq!(result.len(), 360);

        let result = generate_grapheme_permutations("");
        assert_eq!(result, Vec::<String>::new());

        let result = generate_grapheme_permutations("a");
        assert_eq!(result, vec!["a".to_string()]);

        let result = generate_grapheme_permutations("");
        assert_eq!(result, Vec::<String>::new());

        let result = generate_grapheme_permutations("ab");
        assert_eq!(result, vec!["ab".to_string(), "ba".to_string()]);

        let result = generate_grapheme_permutations("cpveteprei");
        assert_eq!(result.len(), 302_400); // 10!/(3!*2!) = 302,400
        assert_eq!(result[0], "ceeeipprtv".to_string());
    }
}

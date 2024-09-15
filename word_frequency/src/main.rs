use word_frequency::*;

fn main() {
    let file_name = "lesmis.txt";
    let directory: &str = "word_frequency";

    let cwd = std::env::current_dir().unwrap();
    let file_path = cwd.join(directory).join(file_name);
    println!("CWD: {}, file_path: {}", cwd.display(), file_path.display());
    let words = get_word_iterator_from_file(file_path.to_str().unwrap()).unwrap();
    let word_freq = get_word_frequency(words);
    let sorted_word_freq = get_sorted_word_frequency(word_freq, 10);

    for (word, freq) in sorted_word_freq {
        println!("{}: \t{}", word, freq);
    }
}

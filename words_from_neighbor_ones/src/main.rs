// access functions in lib.rs
extern crate words_from_neighbor_ones;

use words_from_neighbor_ones::get_all_collected_words;

fn main() {
    let words = get_all_collected_words();
    for word in words {
        println!("{}", word);
    }
}

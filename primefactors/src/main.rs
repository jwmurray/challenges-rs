fn main() {
    let numbers = vec![3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 12, 24, 25];

    for number in numbers {
        let factors = primefactors::primefactors(number);
        println!("Prime factors of {}: {:?}", number, factors);
    }
}

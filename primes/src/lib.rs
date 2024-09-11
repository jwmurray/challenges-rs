pub fn is_prime(number: i32) -> bool {
    let limit = (number as f64).sqrt() as i32;
    for i in 2..=limit {
        if number % i == 0 {
            return false;
        }
    }
    if number == 1 {
        return false;
    } else {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes() {
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(6), false);
        assert_eq!(is_prime(8), false);
    }
}

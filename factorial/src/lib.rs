pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_1() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn test_factorial_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn test_factorial_3() {
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn test_max_factorial_for_u64() {
        assert_eq!(factorial(20), 2432902008176640000);
        let result = 21u64.checked_mul(factorial(20));
        assert!(result.is_none(), "Overflow would occur for factorial(21)");
    }
}

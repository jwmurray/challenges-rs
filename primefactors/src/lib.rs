use primes::is_prime;

pub fn primefactors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    if factors.len() == 0 {
        assert!(is_prime(n as i32) || n == 0 || n == 1);
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(primefactors(3), vec![3]);
        assert_eq!(primefactors(0_u64), vec![]);
        assert_eq!(primefactors(1), vec![]);
        assert_eq!(primefactors(2), vec![2]);
        assert_eq!(primefactors(4), vec![2, 2]);
        assert_eq!(primefactors(5), vec![5]);
        assert_eq!(primefactors(6), vec![2, 3]);
        assert_eq!(primefactors(7), vec![7]);
        assert_eq!(primefactors(8), vec![2, 2, 2]);
        assert_eq!(primefactors(9), vec![3, 3]);
        assert_eq!(primefactors(12), vec![2, 2, 3]);
        assert_eq!(primefactors(24), vec![2, 2, 2, 3]);
        assert_eq!(primefactors(25), vec![5, 5]);
    }
}

use std::collections::HashSet;

pub fn divisors(number: i32) -> Vec<i32> {
    if number == 1 {
        return vec![1];
    }
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(number);
    let limit = (number as f32).sqrt() as i32;
    for i in 2..=limit {
        if number % i == 0 {
            set.insert(i);
            set.insert(number / i);
        }
    }
    let mut list: Vec<i32> = set.into_iter().collect();
    list.sort();
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divs() {
        assert_eq!(divisors(2), vec![1, 2]);
        assert_eq!(divisors(3), vec![1, 3]);
        assert_eq!(divisors(4), vec![1, 2, 4]);
        assert_eq!(divisors(5), vec![1, 5]);
        assert_eq!(divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(divisors(7), vec![1, 7]);
        assert_eq!(divisors(8), vec![1, 2, 4, 8]);
        assert_eq!(divisors(9), vec![1, 3, 9]);
    }

    #[test]
    fn test_div_2() {
        assert_eq!(divisors(2), vec![1, 2]);
    }

    #[test]
    fn test_div_1() {
        let list = divisors(1);
        assert_eq!(list, vec![1]);
    }
}

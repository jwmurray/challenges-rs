pub fn is_arithmetic(n: i32) -> bool {
    let factors = factors(n);
    let average_value = (factors.iter().sum::<i32>() as f32) / (factors.len() as f32);
    is_integer(average_value)
}

fn is_integer(float: f32) -> bool {
    float == float as i32 as f32
}

fn factors(n: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = vec![];
    for i in 1..n + 1 {
        if n % i == 0 {
            factors.push(i);
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aritmetic_1() {
        assert_eq!(is_arithmetic(1), true);
    }
    #[test]
    fn test_aritmetic_3() {
        assert_eq!(is_arithmetic(3), true);
    }
    #[test]
    fn test_aritmetic_4() {
        assert_eq!(is_arithmetic(4), false);
    }
    #[test]
    fn test_many_arithmetic() {
        assert_eq!(is_arithmetic(5), true);
        assert_eq!(is_arithmetic(6), true);
    }
}

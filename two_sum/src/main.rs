fn main() {
    let nums = [0, 2, 11, 19, 90];
    let target_sum = 21;

    let result = two_sum(nums.to_vec(), target_sum);
    match result {
        Some(result) => println!("Result: {:?}", result),
        None => println!("No result found"),
    }
    main2();
}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let sum = nums[i] + nums[j];
            match sum {
                sum if sum == target => {
                    result.push(i as i32);
                    result.push(j as i32);
                    return Some(result);
                }
                sum if sum > target => break,
                _ => {}
            }
        }
    }
    None
}

use std::cmp::Ordering;
use std::ops::Add;

fn two_sum_2<T>(arr: &[T], sum: T) -> Option<(usize, usize)>
where
    T: Add<Output = T> + Ord + Copy,
{
    if arr.len() == 0 {
        return None;
    }

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        match (arr[i] + arr[j]).cmp(&sum) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }

    None
}

fn main2() {
    let arr = [0, 2, 11, 19, 90];
    let sum = 21;

    println!("{:?}", two_sum_2(&arr, sum));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), Some(vec![0, 1]));
        assert_eq!(two_sum(vec![3, 2, 4], 6), Some(vec![1, 2]));
        assert_eq!(two_sum(vec![3, 3], 6), Some(vec![0, 1]));
        assert_eq!(two_sum(vec![3, 5], 6), None);
    }
}

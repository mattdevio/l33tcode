use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;

/// # Two Sum
///
/// Given an array of integers `nums` and an integer `target`,
/// return _indicies of the two numbers shuch that they add up to `target`_.
///
/// You may assume that each input would have *exactly one solution*, and you
/// may not use the same element twice.
///
/// You can return the answer in any order
///
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::<i32, i32>::with_capacity(nums.len());

    for (index, &num) in nums.iter().enumerate() {
        let result = target - num;
        if let Occupied(entry) = hm.entry(result) {
            return vec![*entry.get(), index as i32];
        } else {
            hm.insert(num, index as i32);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod two_sum_tests {
    use super::*;

    fn sort_equal(result: &mut Vec<i32>, answer: &mut Vec<i32>) {
        result.sort();
        answer.sort();
        assert_eq!(result, answer);
    }

    #[test]
    fn test_a() {
        // Arrange
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let mut answer = vec![0, 1];
        // Act
        let mut result = two_sum(nums, target);
        // Assert
        sort_equal(&mut result, &mut answer);
    }
    #[test]
    fn test_b() {
        // Arrange
        let nums = vec![3, 2, 4];
        let target = 6;
        let mut answer = vec![1, 2];
        // Act
        let mut result = two_sum(nums, target);
        // Assert
        sort_equal(&mut result, &mut answer);
    }

    #[test]
    fn test_c() {
        // Arrange
        let nums = vec![3, 3];
        let target = 6;
        let mut answer = vec![0, 1];
        // Act
        let mut result = two_sum(nums, target);
        // Assert
        sort_equal(&mut result, &mut answer);
    }
}

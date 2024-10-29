/*
Problem: Two Sum - LeetCode example with Rich
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution (CHANGED: Multiple solution), and you may not use the same element twice.
You can return the answer in any order.


Example 1:

Input: nums = [2,11,15,7], target = 9
Output: [0,3]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 3].

Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

*/

fn main() {
    println!("Hello, world!");
    let s = two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", s);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for num_idx in 0..nums.len() {
        for num_idx_2 in num_idx + 1..nums.len() {
            if nums[num_idx] + nums[num_idx_2] == target {
                return vec![num_idx.try_into().unwrap(), num_idx_2.try_into().unwrap()];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod two_sum_unit_tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let mut two_sum_values: Vec<i32> = two_sum(vec![3, 2, 4], 6);

        two_sum_values.sort();

        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_two_sum_not_same_indices() {
        // Option 1
        let mut two_sum_values: Vec<i32> = two_sum(vec![3, 2, 4, 3], 6);
        two_sum_values.sort();

        assert_ne!(two_sum_values[0], two_sum_values[1]);

        // Option 2
        assert_ne!(two_sum_values, vec![0, 0]);
    }

    #[test]
    fn test_two_sum_any_order_indices() {
        let mut two_sum_values: Vec<i32> = two_sum(vec![2, 7, 11, 15], 9);

        two_sum_values.sort();

        assert_eq!(two_sum_values, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_indices_apart() {
        let mut two_sum_values: Vec<i32> = two_sum(vec![2, 11, 15, 7], 9);

        two_sum_values.sort();

        assert_eq!(two_sum_values, vec![0, 3]);
    }
}

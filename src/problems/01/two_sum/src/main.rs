// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// Example:

// Given nums = [2, 7, 11, 15], target = 9,
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

fn main() {
    println!("Two Sum");
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(vec![0, 1], result);
}

struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![];

        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                if (nums[i] + nums[j]) == target {
                    result.push(i as i32);
                    result.push(j as i32);
                }
            }
        }
        return result;
    }
}
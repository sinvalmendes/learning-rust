// Given a set of candidate numbers (candidates) (without duplicates) and a
// target number (target), find all unique combinations in candidates where the
// candidate numbers sums to target.

// The same repeated number may be chosen from candidates unlimited number of times.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.
// Example 1:

// Input: candidates = [2,3,6,7], target = 7,
// A solution set is:
// [
//   [7],
//   [2,2,3]
// ]
// Example 2:

// Input: candidates = [2,3,5], target = 8,
// A solution set is:
// [
//   [2,2,2,2],
//   [2,3,3],
//   [3,5]
// ]
use std::collections::HashMap;

fn main() {
    let result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    assert_eq!(vec![vec![7], vec![2, 2, 3]], result);
}

struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut count = 0;
        let mut hash_map: HashMap<Vec<i32>, i32> = HashMap::new();

        while count < candidates.len() {
            let mut available_choices = candidates.clone();
            let choice = available_choices.remove(count);

            count += 1;
        }

        Solution::recursion();
        return vec![vec![0]];
    }

    pub fn recursion() {
        println!("{}", 0);
    }
}

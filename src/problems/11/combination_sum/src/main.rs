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
    assert_eq!(Solution::sum_list(&vec![1, 1, 1, 1]), 4);
    assert_eq!(Solution::sum_list(&vec![]), 0);
    assert_eq!(Solution::sum_list(&vec![1, 2]), 3);

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
            Solution::recursion(vec![choice], candidates.clone(), target);

            count += 1;
        }

        return vec![vec![0]];
    }

    pub fn recursion(choices: Vec<i32>, candidates: Vec<i32>, target: i32) {
        let choices_sum = Solution::sum_list(&choices);
        println!(
            "{:?}|{:?}|{:?}|{:?}",
            choices, candidates, target, choices_sum
        );
    }

    pub fn sum_list(list: &Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for num in list {
            count += num;
        }
        return count;
    }
}

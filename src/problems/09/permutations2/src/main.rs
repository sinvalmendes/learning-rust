// Given a collection of numbers that might contain duplicates, return all possible unique permutations.

// Example:

// Input: [1,1,2]
// Output:
// [
//   [1,1,2],
//   [1,2,1],
//   [2,1,1]
// ]
use std::collections::HashMap;

fn main() {
    let nums = vec![1, 1, 2];
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    let result = Solution::permute_unique(nums);
    assert_eq!(expected, result);
}

struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut count = 0;
        let mut hash_map: HashMap<Vec<i32>, i32> = HashMap::new();

        while count < nums.len() {
            let mut available_choices = nums.clone();
            let head_num = available_choices.remove(count);

            let preselected_nums = vec![head_num];
            println!(
                "\n===preselected_nums:{:?}, available_choices:{:?}",
                preselected_nums, available_choices
            );
            Solution::recursion(
                &mut result,
                preselected_nums,
                available_choices,
                &mut hash_map,
            );

            count += 1;
        }
        println!("final result: {:?}", result);
        return result;
    }

    pub fn recursion(
        result: &mut Vec<Vec<i32>>,
        preselected_nums: Vec<i32>,
        available_choices: Vec<i32>,
        hash_map: &mut HashMap<Vec<i32>, i32>,
    ) {
        println!(
            "result:{:?}, preselected_nums:{:?}, available_choices:{:?}",
            result, preselected_nums, available_choices
        );

        if available_choices.len() == 0 {
            let map_head_num = hash_map.get(&preselected_nums);
            match map_head_num {
                Some(x) => println!("{} was already checked", x),
                None => {
                    println!("adding {:?} to result", preselected_nums);
                    hash_map.insert(preselected_nums.clone(), 1);
                    result.push(preselected_nums.clone());
                }
            };
            return;
        }

        let mut count = 0;
        while count < available_choices.len() {
            let mut available_choices_clone = available_choices.clone();
            let mut preselected_nums_clone = preselected_nums.clone();

            preselected_nums_clone.push(available_choices_clone.remove(count));
            Solution::recursion(
                result,
                preselected_nums_clone,
                available_choices_clone,
                hash_map,
            );

            count += 1;
        }
    }
}

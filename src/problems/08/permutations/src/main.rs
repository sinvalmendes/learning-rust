// Given a collection of distinct integers, return all possible permutations.

// Example:

// Input: [1,2,3]
// Output:
// [
//   [1,2,3],
//   [1,3,2],
//   [2,1,3],
//   [2,3,1],
//   [3,1,2],
//   [3,2,1]
// ]

fn main() {
    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    let result = Solution::permute(nums);
    assert_eq!(expected, result);

    let nums = vec![1, 2];
    let expected = vec![vec![1, 2], vec![2, 1]];
    let result = Solution::permute(nums);
    assert_eq!(expected, result);
}

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut count = 0;
        while count < nums.len() {
            let mut available_choices = nums.clone();
            let head_num = available_choices.remove(count);
            let preselected_nums = vec![head_num];
            println!(
                "\n===preselected_nums:{:?}, available_choices:{:?}",
                preselected_nums, available_choices
            );
            Solution::recursion(&mut result, preselected_nums, available_choices);

            count += 1;
        }
        println!("final result: {:?}", result);
        return result;
    }

    pub fn recursion(
        result: &mut Vec<Vec<i32>>,
        preselected_nums: Vec<i32>,
        available_choices: Vec<i32>,
    ) {
        println!(
            "result:{:?}, preselected_nums:{:?}, available_choices:{:?}",
            result, preselected_nums, available_choices
        );

        if available_choices.len() == 0 {
            println!("adding {:?} to result", preselected_nums);
            result.push(preselected_nums);
            return;
        }

        let mut count = 0;
        while count < available_choices.len() {
            let mut available_choices_clone = available_choices.clone();
            let mut preselected_nums_clone = preselected_nums.clone();

            preselected_nums_clone.push(available_choices_clone.remove(count));
            Solution::recursion(result, preselected_nums_clone, available_choices_clone);

            count += 1;
        }
    }
}

// Given a collection of candidate numbers (candidates) and a target number (target),
// find all unique combinations in candidates where the candidate numbers sums to target.

// Each number in candidates may only be used once in the combination.

// Note:

// All numbers (including target) will be positive integers.
// The solution set must not contain duplicate combinations.

// Example 1:
// Input: candidates = [10,1,2,7,6,1,5], target = 8,
// A solution set is:
// [
//   [1, 7],
//   [1, 2, 5],
//   [2, 6],
//   [1, 1, 6]
// ]

// Example 2:
// Input: candidates = [2,5,2,1,2], target = 5,
// A solution set is:
// [
//   [1,2,2],
//   [5]
// ]

fn main() {
    // let result = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    // let expected = vec![vec![1, 2, 2], vec![5]];
    // assert_eq!(expected.len(), result.len());
    // for value in expected {
    //     assert_eq!(true, result.contains(&value));
    // }

    let result = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    let expected = vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]];
    assert_eq!(expected.len(), result.len());
    for value in expected {
        assert_eq!(true, result.contains(&value));
    }
}

struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let candidates_clone = candidates.clone();
        let mut selected: Vec<i32> = vec![];
        let mut count: i32 = 0;

        let mut position = 0;
        for candidate in candidates {
            println!("initial candidate {:?}, position {:?}", candidate, position);
            position += 1;
        }

        Solution::recursion(
            &candidates_clone,
            &mut selected,
            &mut count,
            target,
            &mut result,
        );

        println!("result {:?}", result);
        return result;
    }

    pub fn recursion(
        candidates: &Vec<i32>,
        selected: &mut Vec<i32>,
        count: &mut i32,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        println!(
            "candidates {:?}, selected {:?}, count {:?}, target {:?}",
            candidates, selected, count, target
        );

        if *count as usize >= candidates.len() {
            return;
        }

        let current_selected = candidates[*count as usize];
        let selected_sum = Solution::sum_list(&selected);

        if current_selected == target {
            result.push(vec![current_selected]);
        }
        if (selected_sum + current_selected) == target {
            selected.push(current_selected);
            selected.sort();
            result.push(selected.to_vec());
            return;
        }
        if (selected_sum + current_selected) < target {
            selected.push(current_selected);
            *count += 1;
            Solution::recursion(&candidates, selected, count, target, result)
        }
        if (selected_sum + current_selected) > target {
            *count += 1;
            Solution::recursion(&candidates, selected, count, target, result)
        }
    }

    pub fn sum_list(list: &Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for num in list {
            count += num;
        }
        return count;
    }
}

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
    let result = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    let expected = vec![vec![1, 2, 2], vec![5]];
    assert_eq!(expected.len(), result.len());
    for value in expected {
        assert_eq!(true, result.contains(&value));
    }

    // let result = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    // let expected = vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]];
    // assert_eq!(expected.len(), result.len());
    // for value in expected {
    //     assert_eq!(true, result.contains(&value));
    // }
}

struct Solution {}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let candidates_clone = candidates.clone();
        let mut selected: Vec<i32> = vec![];

        result = Solution::recursion(&candidates_clone, &mut selected, 0, target);

        println!("result {:?}", result);
        return result;
    }

    pub fn recursion(
        candidates: &Vec<i32>,
        selected: &mut Vec<i32>,
        count: i32,
        target: i32,
    ) -> Vec<Vec<i32>> {
        println!(
            "candidates {:?}, selected {:?}, count {:?}, target {:?}",
            candidates, selected, count, target
        );

        let mut internal_result = vec![];

        if count as usize >= candidates.len() {
            return internal_result;
        }
        let selected_sum = Solution::sum_list(&selected);
        if selected_sum == target {
            selected.sort();
            internal_result.push(selected.to_vec());
            return internal_result;
        }

        let candidate = candidates[count as usize];
        println!("current candidate {:?}", candidate);

        if selected_sum < target {
            let mut selected_clone = selected.clone();
            let result1 = Solution::recursion(&candidates, &mut selected_clone, count + 1, target);

            let mut selected_clone2 = selected.clone();
            selected_clone2.push(candidate);
            let result2 = Solution::recursion(&candidates, &mut selected_clone2, count + 1, target);

            for result in result1 {
                internal_result.push(result);
            }
            for result in result2 {
                internal_result.push(result);
            }
        }
        return internal_result;
    }

    pub fn sum_list(list: &Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        for num in list {
            count += num;
        }
        return count;
    }
}

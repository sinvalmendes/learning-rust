// Given a collection of integers that might contain duplicates, nums, return all possible subsets (the power set).

// Note: The solution set must not contain duplicate subsets.

// Example:

// Input: [1,2,2]
// Output:
// [
//   [2],
//   [1],
//   [1,2,2],
//   [2,2],
//   [1,2],
//   []
// ]

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 2, 2];
    let result = Solution::subsets_with_dup(nums);
    let expected_result: Vec<Vec<i32>> =
        vec![vec![2], vec![1], vec![1, 2, 2], vec![2, 2], vec![1, 2]];

    assert_eq!(expected_result.len(), result.len());

    for expected in &expected_result {
        assert_eq!(true, expected_result.contains(&expected));
    }
}

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("nums {:?}", nums);
        return vec![];
    }
}

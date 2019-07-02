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
    let nums = vec![1,2,3];
    let expected = vec![
        vec![1,2,3], vec![1,3,2], vec![2,1,3],
        vec![2,3,1], vec![3,1,2], vec![3,2,1],
    ];
    let result = Solution::permute(nums);
    assert_eq!(expected, result);
}

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        
        return vec![vec![1,2,3]];
    }
}
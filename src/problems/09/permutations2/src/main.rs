// Given a collection of numbers that might contain duplicates, return all possible unique permutations.

// Example:

// Input: [1,1,2]
// Output:
// [
//   [1,1,2],
//   [1,2,1],
//   [2,1,1]
// ]

fn main() {
    let nums = vec![1,1,2];
    let expected = vec![
        vec![1,1,2], vec![1,2,1], vec![2,1,1],
    ];
    let result = Solution::permute_unique(nums);
    assert_eq!(expected, result);
}

struct Solution {}

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {

        return vec![vec![0,0,0]];
    }
}
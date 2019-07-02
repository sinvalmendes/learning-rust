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
        let mut result = vec![];
        let mut count = 0;
        for num in &nums {
            let mut nums_clone = nums.clone();
            let head_num = nums_clone.remove(count);
            println!("\n===head_num:{:?}, nums:{:?}", head_num, nums_clone);
            Solution::recursion(&mut result, head_num, nums_clone);

            count += 1;
        }
        return vec![vec![1,2,3]];
    }

    pub fn recursion(result: &mut Vec<Vec<i32>>, head_num: i32, nums: Vec<i32>) {
        println!("result:{:?}, head_num:{:?}, nums:{:?}", result, head_num, nums);
    }
}
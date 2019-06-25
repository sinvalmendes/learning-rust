// Given a sorted array and a target value, return the index if the target is found. 
// If not, return the index where it would be if it were inserted in order.
// You may assume no duplicates in the array.

// Example 1:
// Input: [1,3,5,6], 5
// Output: 2

// Example 2:
// Input: [1,3,5,6], 2
// Output: 1

// Example 3:
// Input: [1,3,5,6], 7
// Output: 4

// Example 4:
// Input: [1,3,5,6], 0
// Output: 0

fn main() {

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 5);
    assert_eq!(2, result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 2);
    assert_eq!(1, result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 7);
    assert_eq!(4, result);

    let nums = vec![1,3,5,6];
    let result = Solution::search_insert(nums, 0);
    assert_eq!(0, result);
}

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut count = 0;
        for num in nums {
            if target == num {
                return count
            } else if target < num {
                return count
            }
            count += 1;
        }
        return count;
    }
}
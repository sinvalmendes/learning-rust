// Given a sorted array nums, remove the duplicates in-place such that each element appear only once and return the new length.

// Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

// Example 1:

// Given nums = [1,1,2],

// Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.

// It doesn't matter what you leave beyond the returned length.
// Example 2:

// Given nums = [0,0,1,1,1,2,2,3,3,4],

// Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.

// It doesn't matter what values are set beyond the returned length.

struct Solution {}
use std::collections::HashMap;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut existing_nums = HashMap::new();
        let mut duplicates_counter = 0;
        let len = nums.len() as i32;
        for num in nums {
            match existing_nums.get(num) {
                Some(x) => {
                    existing_nums.insert(num, x+1);
                    duplicates_counter += 1;
                },
                None => {existing_nums.insert(num, 1);}
            }
        }
        return len - duplicates_counter;
    }
}

fn main() {
    let mut nums = vec![1,1,2];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(2, result);

    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(5, result);

    let mut nums = vec![1,1,2,2,3,3,4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(4, result);

    let mut nums = vec![0,0,1,1,1,2,2,3,3,4,4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(5, result);
}

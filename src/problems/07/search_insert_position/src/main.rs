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



    let nums = vec![1,3,5,6];
    let result = Solution::binary_search_insert(nums, 5);
    assert_eq!(2, result);

    let nums = vec![1,3,5,6];
    let result = Solution::binary_search_insert(nums, 2);
    assert_eq!(1, result);

    let nums = vec![1,3,5,6];
    let result = Solution::binary_search_insert(nums, 7);
    assert_eq!(4, result);

    let nums = vec![1,3,5,6];
    let result = Solution::binary_search_insert(nums, 0);
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

    pub fn binary_search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len() as i32;
        let result = Solution::recursive_search(nums, target, 0, len - 1);
        return result;
    }

    pub fn recursive_search(nums: Vec<i32>, target: i32, begin: i32, end: i32) -> i32 {
        println!("\n+++++++++++++");
        println!("{} {}", begin, end);
        let middle = (begin + end) / 2;
        println!("| begin {:?} | end {:?} | middle {:?}|", begin, end, middle);
        println!("|{:?} | target {:?} |", nums, target);
        if middle == begin { 
            return middle;
        }
        let mut current_value: i32 = 999999999;

        match nums.get(middle as usize) {
                Some(x) => {
                    current_value = *x;
                }
                None => {
                    panic!("Something is wrong.");
                }
        };



        if current_value > target {
            let new_end = middle - 1;
            println!("{} > {}, {}:{}", current_value, target, begin, new_end);
            return Solution::recursive_search(nums, target, begin, new_end);
        } else {
            println!("{} <= {}, {}:{}", current_value, target, middle, end);
            return Solution::recursive_search(nums, target, middle, end);
        }

    }
}

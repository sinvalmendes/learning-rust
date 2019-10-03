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
    let expected_result: Vec<Vec<i32>> = vec![
        vec![2],
        vec![1],
        vec![1, 2, 2],
        vec![2, 2],
        vec![1, 2],
        vec![],
    ];

    assert_eq!(expected_result.len(), result.len());

    for expected in &expected_result {
        assert_eq!(true, expected_result.contains(&expected));
    }
}

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        println!("nums {:?}", nums);
        let mut result: Vec<Vec<i32>> = vec![vec![]];

        Solution::helper(nums, &mut vec![], &mut result, "---");
        println!("final result => {:?}", result);
        return result;
    }

    pub fn helper(
        nums: Vec<i32>,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        string: &str,
    ) {
        println!(
            "{:?} nums {:?}, combination {:?}, result {:?}",
            string, nums, combination, result
        );
        if nums.len() == 0 {
            println!(
                "{:?} nums.len() == 0, combination {:?}",
                string, combination
            );
            result.push(combination.clone());
            return;
        }

        let mut nums_clone = nums.clone();
        let selected = nums_clone.remove(0);
        println!("{:?} selected {:?}", string, selected);

        let new_string = format!("---{}", string);

        let result1 = Solution::helper(
            nums_clone.clone(),
            &mut combination.clone(),
            result,
            &new_string,
        );
        println!("{:?} result1 {:?}", string, result1);

        combination.push(selected);
        let result2 = Solution::helper(
            nums_clone.clone(),
            &mut combination.clone(),
            result,
            &new_string,
        );
        println!("{:?} result2 {:?}", string, result2);

        return;
    }
}

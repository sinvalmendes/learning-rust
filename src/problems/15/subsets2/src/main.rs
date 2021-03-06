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
    //println!("Hello, world!");
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

    let nums = vec![4, 4, 4, 1, 4];
    let result = Solution::subsets_with_dup(nums);
    let expected_result: Vec<Vec<i32>> = vec![
        vec![],
        vec![1],
        vec![1, 4],
        vec![1, 4, 4],
        vec![1, 4, 4, 4],
        vec![1, 4, 4, 4, 4],
        vec![4],
        vec![4, 4],
        vec![4, 4, 4],
        vec![4, 4, 4, 4],
    ];

    assert_eq!(expected_result.len(), result.len());

    for expected in &expected_result {
        assert_eq!(true, expected_result.contains(&expected));
    }
}

struct Solution {}

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // println!("nums {:?}", nums);
        let mut result: Vec<Vec<i32>> = vec![vec![]];
        Solution::helper(&mut nums.clone(), &mut vec![], &mut result, "---");
        // println!("final result => {:?}", result);
        return result;
    }

    #[rustfmt::skip]
    pub fn helper(
        nums: &mut Vec<i32>,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        string: &str,
    ) {
        combination.sort();

        let new_string = format!("---{}", string);

        // println!("{:?} nums {:?}, combination {:?}, result {:?}", string, nums, combination, result);
        if nums.len() == 0 {
            // println!("{:?} nums.len() == 0, combination {:?}", string, combination);
            if !result.contains(combination) {
                result.push(combination.to_vec());
            }
            return;
        }

        let selected = nums.remove(0);

        Solution::helper(&mut nums.clone(), &mut combination.clone(), result, &new_string);

        combination.push(selected);
        Solution::helper(&mut nums.clone(), &mut combination.clone(), result, &new_string);

        return;
    }
}

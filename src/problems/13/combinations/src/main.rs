// Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.

// Example:

// Input: n = 4, k = 2
// Output:
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]

fn main() {
    let result = Solution::combine(4, 2);
    let expected = vec![
        vec![2, 4],
        vec![3, 4],
        vec![2, 3],
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
    ];
    println!("expected {:?}", expected);
    assert_eq!(expected.len(), result.len());
    for value in expected {
        assert_eq!(true, result.contains(&value));
    }
}

struct Solution {}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut candidates = vec![];
        for i in 1..n + 1 {
            candidates.push(i);
        }
        println!("{:?}", candidates);
        Solution::recursion(candidates, k, &mut vec![]);
        return vec![];
    }

    pub fn recursion(candidates: Vec<i32>, k: i32, selected: &mut Vec<i32>) {
        // println!("{:?}, {:?}", k, candidates);
        // let mut internal_result: Vec<Vec<i32>> = vec![];
        // let mut count = 0;

        if selected.len() == k as usize {
            selected.sort();
            println!("--> {:?}", selected);
            return;
        }

        for i in 0..candidates.len() {
            let mut selected_clone = selected.clone();
            let mut candidates_clone = candidates.clone();
            let current_candidate = candidates.get(i).unwrap();
            selected_clone.push(*current_candidate);
            candidates_clone.remove(i);

            println!(
                "current_candidate {:?}, selected {:?}, candidates_clone {:?}",
                current_candidate, selected, candidates_clone
            );
            Solution::recursion(candidates_clone, k, &mut selected_clone);
        }
        // let mut selected_clone = selected.clone();
        // for candidate in candidates_clone {
        //     let current_candidate = candidate;
        //     selected.push(current_candidate);
        //     let mut candidates_clone2 = candidates.clone();
        //     candidates_clone2.remove(count);
        //     Solution::recursion(candidates_clone2, k, &mut selected.clone());
        //     count += 1;
        // }
        // if selected_clone.len() as i32 == k {
        //     println!("---{:?}", selected_clone);
        // }
    }
}

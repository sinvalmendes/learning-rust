// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

// For example, given n = 3, a solution set is:

// [
//   "((()))",
//   "(()())",
//   "(())()",
//   "()(())",
//   "()()()"
// ]

fn main() {
    let n = 3;
    let expected = vec![
        String::from("((()))"), String::from("(()())"), String::from("(())()"), 
        String::from("()(())"), String::from("()()()")
    ];
    let result = Solution::generate_parenthesis(n);
    assert_eq!(expected, result);
}

struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        
        return vec![String::from("")];
    }
}
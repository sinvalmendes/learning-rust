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
        let mut result: Vec<String> = vec![];

        let mut open_counter = 0;
        let mut left_counter = n;
        let mut right_counter = n;

        let mut string = String::from("");
        Solution::recursive(open_counter, left_counter, right_counter, string, &mut result, n*2);
        return result;
    }

    pub fn recursive(open_counter: i32, left_counter: i32, right_counter:  i32, string: String, result: &mut Vec<String>, string_size: i32) {
        println!("\nopen_counter:{:?}, left_counter:{:?}, right_counter:{:?}, string:|{:?}|", open_counter, left_counter, right_counter, string);

        if string.len() as i32 == string_size {
            result.push(string);
            return;
        }

        if left_counter > 0 {
            let mut current_string = string.clone();
            let new_left_counter = left_counter - 1;
            let new_open_counter = open_counter + 1;
            current_string.push('(');
            if new_open_counter <= right_counter {
                Solution::recursive(new_open_counter, new_left_counter, right_counter, current_string, result, string_size);
            }
        }

        if right_counter > 0 && open_counter >= 1 {
            let mut current_string = string.clone();
            let new_right_counter = right_counter - 1;
            let new_open_counter = open_counter - 1;
            current_string.push(')');
            Solution::recursive(new_open_counter, left_counter, new_right_counter, current_string, result, string_size);
        }
    }
}
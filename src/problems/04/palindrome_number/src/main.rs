// Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.

// Example 1:
// Input: 121
// Output: true

// Example 2:
// Input: -121
// Output: false

// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:
// Input: 10
// Output: false

// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

fn main() {
    let result = Solution::is_palindrome(123);
    assert_eq!(false, result);

    let result = Solution::is_palindrome(121);
    assert_eq!(true, result);

    let result = Solution::is_palindrome(-121);
    assert_eq!(false, result);
}

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let string = x.to_string();
        let reverse_string = string.chars().rev().collect::<String>();
        return reverse_string == string;
    }
}
// Given a 32-bit signed integer, reverse digits of an integer.

// Example 1:

// Input: 123
// Output: 321

// Example 2:
// Input: -123
// Output: -321

fn main() {
    let result = Solution::reverse(123);
    assert_eq!(321, result);

    let result = Solution::reverse(-123);
    assert_eq!(-321, result);
}

struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let is_negative = x < 0;
        let string = x.to_string();
        let mut reverse_string = string.chars().rev().collect::<String>();
        if is_negative {
            reverse_string.truncate(reverse_string.len() - 1);
            return reverse_string.parse::<i32>().unwrap() * -1;
        }

        return reverse_string.parse::<i32>().unwrap();
    }
}
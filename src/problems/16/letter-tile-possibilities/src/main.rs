// You have a set of tiles, where each tile has one letter tiles[i] printed on it.
// Return the number of possible non-empty sequences of letters you can make.

// Example 1:

// Input: "AAB"
// Output: 8
// Explanation: The possible sequences are
// "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
// Example 2:

// Input: "AAABBC"
// Output: 188

// Note:
// 1 <= tiles.length <= 7
// tiles consists of uppercase English letters.
use std::thread;

fn main() {
    println!("Letter Tiles Possibilities");
    Solution::num_tile_possibilities(String::from("AAB"));
    // Solution::num_tile_possibilities(String::from("AAABBC"));
}

struct Solution {}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        println!("num_tile_possibilities: {}", tiles);
        let mut result: Vec<String> = vec![];
        let combinations = Solution::rotate(tiles.clone());
        println!("num_tile_possibilities, combinations: {:?}", combinations);
        return result.len() as i32;
    }

    pub fn rotate(string: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut count = 0;
        let mut mutable_string = string.clone();
        while count < string.len() {
            result.push(mutable_string.clone());
            let selected = string.chars().nth(0).unwrap().to_string();
            mutable_string.remove(0);
            mutable_string.push_str(&selected);
            count += 1;
        }
        return result;
    }

    pub fn helper(combination: &str, tiles: &mut String, result: &mut Vec<String>) {}

    pub fn append(string: &mut String, letter: &String) -> String {
        //println!("append: {} in {}", letter, string);
        string.push_str(letter);
        //println!("append result: {}", string);
        return string.clone();
    }
}

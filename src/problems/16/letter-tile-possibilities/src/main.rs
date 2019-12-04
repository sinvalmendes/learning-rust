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
    let result = Solution::num_tile_possibilities(String::from("AAB"));
    println!("result: {}", result);
    let result = Solution::num_tile_possibilities(String::from("AAABBC"));
    println!("result: {}", result);
}

struct Solution {}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        println!("num_tile_possibilities: {}", tiles);
        let mut result: Vec<String> = vec![];
        let mut visited: Vec<String> = vec![];

        for i in 0..tiles.len() {
            println!("i: {}", i);
            let mut answer: Vec<String> = vec![];
            Solution::helper(
                &mut tiles.clone(),
                &mut result.clone(),
                &mut visited.clone(),
                (i + 1) as i32,
            );
        }
        return 0;
    }

    pub fn helper(
        tiles: &mut String,
        result: &mut Vec<String>,
        visited: &mut Vec<String>,
        limit: i32,
    ) -> Vec<String> {
        return vec![];
    }
}

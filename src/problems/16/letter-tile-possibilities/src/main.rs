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

fn main() {
    println!("Letter Tiles Possibilities");
    Solution::num_tile_possibilities(String::from("AAB"));
    Solution::num_tile_possibilities(String::from("AAABBC"));
}

struct Solution {}

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        Solution::get_unique_letters(tiles);
        return 0;
    }

    pub fn get_unique_letters(string: String) -> Vec<String> {
        println!("{}", string);
        return vec![];
    }
}

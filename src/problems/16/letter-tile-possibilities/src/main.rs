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
        Solution::helper("", &mut tiles.clone(), &mut result);
        return result.len() as i32;
    }

    pub fn helper(combination: &str, tiles: &mut String, result: &mut Vec<String>) {
        // println!("helper: {:?}, {:?}, {:?}", combination, tiles, result);
        // thread::sleep_ms(1000);
        if combination.len() > 3 {
            return;
        }

        if result.contains(&String::from(combination)) {
            return;
        }

        if combination.len() > 0 {
            result.push(String::from(combination));
            println!("pushed, > result: {:?}", result);
        }

        if tiles.len() == 0 {
            return;
        }

        let selected = tiles.chars().nth(0).unwrap().to_string();
        tiles.remove(0);
        println!(
            "helper, selected: {:?}, tiles: {:?}, result: {:?}",
            selected, tiles, result
        );

        let new_combination = Solution::append(&mut String::from(combination), &selected);
        Solution::helper(&new_combination, &mut tiles.clone(), &mut result.clone());

        let new_tiles = Solution::append(&mut tiles.clone(), &selected);
        Solution::helper(combination, &mut new_tiles.clone(), &mut result.clone());
    }

    pub fn append(string: &mut String, letter: &String) -> String {
        //println!("append: {} in {}", letter, string);
        string.push_str(letter);
        //println!("append result: {}", string);
        return string.clone();
    }
}

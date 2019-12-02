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
        let mut general_result: Vec<String> = vec![];
        let combinations = Solution::rotate(tiles.clone());
        println!("num_tile_possibilities, combinations: {:?}", combinations);
        for combination in combinations {
            println!("num_tile_possibilities, combination: {:?}", &combination);
            let mut result: Vec<String> = vec![];
            let mut calls: Vec<String> = vec![];
            result = Solution::helper(
                &mut String::from(""),
                &mut combination.clone(),
                &mut result.clone(),
                &mut calls.clone(),
            );
            result.push(combination);
            println!("num_tile_possibilities, result: {:?}", result);
            general_result.append(&mut result.clone());
        }
        println!(">>> {:?}", general_result);
        general_result.sort();
        general_result.dedup();
        return general_result.len() as i32;
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

    pub fn helper(
        combination: &mut String,
        tiles: &mut String,
        result: &mut Vec<String>,
        calls: &mut Vec<String>,
    ) -> Vec<String> {
        let mut call = Solution::append(&mut combination.clone(), "-");
        call = Solution::append(&mut call.clone(), &tiles);
        calls.push(call);
        println!("helper, calls:{:?}", calls);
        return vec![];
    }

    pub fn append(string: &mut String, letter: &str) -> String {
        println!("append: {} in {}", letter, string);
        string.push_str(letter);
        println!("append result: {}", string);
        return string.clone();
    }
}

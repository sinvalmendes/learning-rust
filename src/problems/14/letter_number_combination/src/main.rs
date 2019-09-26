// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.

// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.

// Example:

// Input: "23"
// Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].

fn main() {}

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {}
}

fn _main() {
    println!("Hello, world!");

    let one: Vec<&'static str> = vec!["a", "b", "c"];
    let two: Vec<&'static str> = vec!["d", "e", "f"];
    let three: Vec<&'static str> = vec!["g", "h", "i"];
    // let four = vec!["d", "e", "f"];
    // let five = vec!["d", "e", "f"];
    // let six = vec!["d", "e", "f"];
    // let seven = vec!["d", "e", "f"];
    // let eight = vec!["d", "e", "f"];
    // let nine = vec!["d", "e", "f"];

    let result_one = append_to_string(&one, String::from(""));
    println!("{:?}", result_one);

    let mut result_two = vec![];
    for result in result_one {
        let result_append = append_to_string(&two, result);
        for i in result_append {
            result_two.push(i);
        }
    }
    println!("{:?}", result_two);

    let mut result_three = vec![];
    for result in result_two {
        let result_append = append_to_string(&three, result);
        for i in result_append {
            result_three.push(i);
        }
    }
    println!("{:?}", result_three);
}

fn append_to_string(vec: &Vec<&'static str>, string: String) -> Vec<String> {
    let mut result = vec![];

    for i in vec {
        result.push(format!("{}{}", string, i));
    }
    return result;
}

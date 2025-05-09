// Leetcode Problem 17

use std::collections::HashMap;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // Return if digits is empty
        if digits.is_empty() {
            return vec![];
        }

        let keyboard: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ].iter().cloned().collect();

        let mut combinations: Vec<String> = Vec::new();

        backtrack(String::new(), &digits, &keyboard, &mut combinations);

        combinations
    }
}

fn backtrack(combination: String, next_digits: &str, keyboard: &HashMap<char, &str>, combinations: &mut Vec<String>) {
    // If next_digits is empty we reached the end and can push the string into combinations
    if next_digits.is_empty() {
        combinations.push(combination);
        return;
    }

    if let Some(&letters) = keyboard.get(&next_digits.chars().next().unwrap()) {
        for letter in letters.chars() {
            let mut new_combination = combination.clone();
            new_combination.push(letter);
            backtrack(new_combination, &next_digits[1..], keyboard, combinations);
        }
    }
}


#[test]
fn test_letter_combinations() {
    let digits1 = String::from("23");
    let expected1 = vec![String::from("ad"),String::from("ae"),String::from("af"),String::from("bd"),String::from("be"),String::from("bf"),String::from("cd"),String::from("ce"),String::from("cf")];

    let digits2 = String::from("");
    let expected2: Vec<String> = vec![];

    let digits3 = String::from("2");
    let expected3 = vec![String::from("a"), String::from("b"), String::from("c")];

    assert_eq!(Solution::letter_combinations(digits1), expected1);
    assert_eq!(Solution::letter_combinations(digits2), expected2);
    assert_eq!(Solution::letter_combinations(digits3), expected3);

}
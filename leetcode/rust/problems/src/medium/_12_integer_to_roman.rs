// Leetcode Problem 12

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman_num: String = String::new();

        let dif = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let letter = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut pos = 0;
        
        while num != 0 {
            while num - dif[pos] >= 0 {
                roman_num.push_str(letter[pos]);
                num -= dif[pos];
            }
            pos += 1;
        }

        roman_num
    }
}

#[test]
pub fn test_int_to_roman() {
    assert_eq!(Solution::int_to_roman(3749), String::from("MMMDCCXLIX"));
    assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
    assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
}
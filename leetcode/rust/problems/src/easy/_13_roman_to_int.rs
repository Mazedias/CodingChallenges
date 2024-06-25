// Problem 13
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int (s: String) -> i32 {
        let mut num: i32 = 0;
        let mut carry_over = 0;

        for c in s.chars() {
            let value: i32 = translate_char(c);

            if value > carry_over {
                num = (num - carry_over) + (value - carry_over);
            } else {
                num += value;
            }

            carry_over = value;
        }

        return num;
    }

    
}

fn translate_char(char: char) -> i32 {
    match char {
        'I' => {
            return 1;
        }
        'V' => {
            return 5;
        }
        'X' => {
            return 10;
        }
        'L' => {
            return 50;
        }
        'C' => {
            return 100;
        }
        'D' => {
            return 500;
        }
        'M' => {
            return 1000;
        }
        _ => {
            panic!("Invalid Roman Number");
        }
    }
}

#[test]
pub fn test_roman_to_int () {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(Solution::roman_to_int("IVLCM".to_string()), 844);
}
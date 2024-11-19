// Problem 8
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut sol: i32 = 0;
        let mut negative = false;
        let mut found_digit = false;
        let mut greater = false;
        
        for c in s.chars() {
            if c.is_alphabetic() || c.eq_ignore_ascii_case(&'.') {
                break;
            }

            if !found_digit && c.eq_ignore_ascii_case(&'-') {
                negative = true;
                found_digit = true;
            } else if !found_digit && c.eq_ignore_ascii_case(&'+'){
                found_digit = true;
            } else if !found_digit && c.is_digit(10) {
                sol = c.to_digit(10).unwrap() as i32;
                found_digit = true;
            } else if found_digit && c.is_digit(10) {
                let mul_temp = match sol.checked_mul(10) {
                    Some(val ) => val,
                    None => {
                        sol = std::i32::MAX;
                        greater = true;
                        break;
                    },
                };

                let add_temp = match mul_temp.checked_add(c.to_digit(10).unwrap() as i32) {
                    Some(val) => val,
                    None => {
                        sol = std::i32::MAX;
                        greater = true;
                        break;
                    },
                };

                sol = add_temp;

            } else if found_digit && !c.is_digit(10) {
                break;
            }
        }

        if negative {
            if sol == std::i32::MAX && greater {
                sol = std::i32::MIN;
            } else if sol == std::i32::MAX {
                sol = std::i32::MIN + 1;
            } else {
                sol *= -1;
            }
        }
        sol
    }
}

#[test]
pub fn test_my_atoi() {
    let s1 = String::from("42");
    let s2 = String::from("-042");
    let s3 = String::from("1337c0d3");
    let s4 = String::from("0-1");
    let s5 = String::from("words and 987");

    let s6 = String::from("00000-42a1234");
    let s7 = String::from("-91283472332");
    let s8 = String::from("-000000000000000000000000000001");
    let s9 = String::from("+001a14");
    let s10 = String::from("+-2");
    let s11 = String::from(".1");
    let s12 = String::from("-2147483647");
    let s13 = String::from("2147483648");

    assert_eq!(Solution::my_atoi(s1), 42);
    assert_eq!(Solution::my_atoi(s2), -42);
    assert_eq!(Solution::my_atoi(s3), 1337);
    assert_eq!(Solution::my_atoi(s4), 0);
    assert_eq!(Solution::my_atoi(s5), 0);

    assert_eq!(Solution::my_atoi(s6), 0);
    assert_eq!(Solution::my_atoi(s7), -2147483648);
    assert_eq!(Solution::my_atoi(s8), -1);
    assert_eq!(Solution::my_atoi(s9), 1);
    assert_eq!(Solution::my_atoi(s10), 0);
    assert_eq!(Solution::my_atoi(s11), 0);
    assert_eq!(Solution::my_atoi(s12), -2147483647);
    assert_eq!(Solution::my_atoi(s13), 2147483647)
}
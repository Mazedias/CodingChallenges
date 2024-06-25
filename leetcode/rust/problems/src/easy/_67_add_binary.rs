// Problem 67
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary (a: String, b: String) -> String {
        let maxlen = a.len().max(b.len());
        let mut carry = false;

        let mut solution = String::new();

        let a = format!("{:0>width$}", a, width=maxlen);
        let b = format!("{:0>width$}", b, width=maxlen);

        for i in (0..maxlen).rev() {
            let (a_char, b_char) = (a.as_bytes()[i] as char, b.as_bytes()[i] as char);
            match (a_char, b_char, carry) {
                ('1', '1', false) | ('1', '0', true) | ('0', '1', true) => {
                    solution.push('0');
                    carry = true;
                }
                ('1', '0', false) | ('0', '1', false) | ('0', '0', true) => {
                    solution.push('1');
                    carry = false;
                }
                ('1', '1', true) => {
                    solution.push('1');
                }
                ('0', '0', false) => {
                    solution.push('0');
                }
                _ => unreachable!(),
            }
        }

        if carry == true { solution.push('1'); }

        solution.chars().rev().collect()
        
    }

    pub fn add_binary_inbuild (a: String, b: String) -> String {
        let val_a = i128::from_str_radix(&a, 2).unwrap_or(0);
        let val_b = i128::from_str_radix(&b, 2).unwrap_or(0);

        format!("{:b}", val_a+val_b)
    }
}

#[test]
pub fn test_add_binary () {
    let a1 = "11".to_string();
    let b1 = "1".to_string();
    let expected1 = "100".to_string();

    let a2 = "1010".to_string();
    let b2 = "1011".to_string();
    let expected2 = "10101".to_string();

    assert_eq!(Solution::add_binary(a1, b1), expected1);
    assert_eq!(Solution::add_binary(a2, b2), expected2);
}
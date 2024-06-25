// Problem 66
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one (mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;

        for i in (0..digits.len()).rev() {
            if (digits[i] + carry) < 10 {
                digits[i] += carry;
                carry = 0;
                break;
            } else {
                let sum = digits[i] + carry;
                digits[i] = (digits[i] + carry) % 10;
                carry = sum / 10;
            }
        }
        
        if carry != 0 {
            digits.insert(0, carry);
        }

        digits
    }

    /**
     * Better solution for this specific problem since there is only ever a 1
     * as a carry over
     */
    pub fn plus_one_b (mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = true;

        for i in (0..digits.len()).rev() {
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                carry = false;
                break;
            }
        }

        if carry {
            digits.insert(0, 1);
        }

        digits
    }
}

#[test]
pub fn test_plus_one () {
    let a = vec![1, 2, 3];
    let a_sol = vec![1, 2, 4];
    let b = vec![4 ,3 ,2, 1];
    let b_sol = vec![4, 3, 2, 2];
    let c = vec![9];
    let c_sol = vec![1, 0];

    assert_eq!(Solution::plus_one(a), a_sol);
    assert_eq!(Solution::plus_one(b), b_sol);
    assert_eq!(Solution::plus_one(c), c_sol);
}
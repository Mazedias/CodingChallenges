// Leetcode Problem 2429

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimize_xor(num1: i32, mut num2: i32) -> i32 {
        let mut x: i32 = 0;

        let mut bits = 0;
        while num2 != 0 {
            bits += num2 & 1;
            num2 >>= 1;
        }

        // For each set bit in num1 set a bit in x if we have bits left to set
        for i in (0..32).rev() {
            if num1 & (1 << i) != 0 && bits > 0 {
                x |= 1 << i;
                bits -= 1;
            }
        }

        // Set the remaining bits
        for i in 0..32 {
            if bits == 0 {
                break;
            }

            if x & (1 << i) == 0 {
                x |= 1 << i;
                bits -= 1;
            }
        }

        x
    }
}


#[test]
fn test_minimize_xor() {
    assert_eq!(Solution::minimize_xor(3, 5), 3);
    assert_eq!(Solution::minimize_xor(1, 12), 3);
    assert_eq!(Solution::minimize_xor(15, 1), 8);
    assert_eq!(Solution::minimize_xor(15, 15), 15);
}
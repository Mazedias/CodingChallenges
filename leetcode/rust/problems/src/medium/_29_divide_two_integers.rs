// Leetcode Problem 29

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide(divident: i32, divisor: i32) -> i32 {
        if divisor == divident {
            return 1;
        }

        // If the signs of divident and divisor differ the result will be negative
        let is_negative = divident.is_negative() ^ divisor.is_negative();

        // Convert to absolute values and use u32 to avoid negative shifts
        let (mut divid, divis) = (divident.abs() as u32, divisor.abs() as u32);
        let mut ans = 0;

        // While the remaining divident is greater than or equal to the divisor, 
        // continue subtracting the largest possible multiple of the divisor
        while divid >= divis {
            let mut quotient = 0;

            // Find the highest power of two such that (divisor << quotiont + 1) is still smaller of equal to divident
            while divid > (divis << (quotient + 1)) {
                quotient += 1;
            }

            // Add that power of two to the answer (2^quotient * divisor is subtracted)
            ans += 1 << quotient;

            // Subtract the found multiple of the divisor from the divident
            divid -= divis << quotient
        }

        // Account for overflow
        if ans == 1 << 31 && !is_negative {
            return i32::MAX;
        }

        match is_negative {
            true => -ans,
            false => ans,
        }
    }
}


#[test]
fn test_divide() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
}
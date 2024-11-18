// Problem 1652
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decrypt(mut code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0; code.len()];
        }

        let original = code.clone();
        let len = code.len();

        if k > 0 {
            let mut sum = (1..=k as usize)
                .map(|i| original[i % len])
                .sum();

            for i in 0..len {
                code[i] = sum;
                sum = (sum - original[(i + 1) % len]) + original[(i + k as usize + 1) % len];
            }
        } else {
            let k = k.abs() as usize;

            let mut sum: i32 = (1..=k)
                .map(|i| original[(len - i) % len])
                .sum();

            for i in 0..len {
                code[i] = sum;

                sum -= original[(len + i - k) % len];
                sum += original[i % len];
            }
        }

        code
    }
}

#[test]
pub fn test_decrypt() {
    let vec1 = vec![5, 7, 1, 4];
    let sol1 = vec![12, 10, 16, 13];

    let vec2 = vec![1, 2, 3, 4];
    let sol2 = vec![0, 0, 0, 0];

    let vec3 = vec![2, 4, 9, 3];
    let sol3 = vec![12, 5, 6, 13];

    let vec4 = vec![1, 2, 3, 4];
    let sol4 = vec![15, 17, 15, 13];

    assert_eq!(Solution::decrypt(vec1, 3), sol1);
    assert_eq!(Solution::decrypt(vec4, 6), sol4);
    assert_eq!(Solution::decrypt(vec2, 0), sol2);
    assert_eq!(Solution::decrypt(vec3, -2), sol3);
}
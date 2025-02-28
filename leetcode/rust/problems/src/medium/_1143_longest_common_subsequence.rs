// Leetcode Problem 1143

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let m = text1.len();
        let n = text2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        let text1_chars: Vec<char> = text1.chars().collect();
        let text2_chars: Vec<char> = text2.chars().collect();

        for i in 1..=m {
            for j in 1..=n {
                if text1_chars[i-1] == text2_chars[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                } else {
                    dp[i][j] = dp[i-1][j].max(dp[i][j-1]);
                }
            }
        }

        dp[m][n]
    }
}


#[test]
fn test_longest_common_subsequence() {
    let text11 = String::from("abcde");
    let text12 = String::from("ace");

    let text21 = String::from("abc");
    let text22 = String::from("abc");

    let text31 = String::from("abc");
    let text32 = String::from("def");

    let text41 = String::from("abcdea");
    let text42 = String::from("acea");

    assert_eq!(Solution::longest_common_subsequence(text11, text12), 3);
    assert_eq!(Solution::longest_common_subsequence(text21, text22), 3);
    assert_eq!(Solution::longest_common_subsequence(text31, text32), 0);
    assert_eq!(Solution::longest_common_subsequence(text41, text42), 4);
}
// Leetcode Problem 2109

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut solution = String::with_capacity(s.len() + spaces.len());

        let mut next_space_index = 0;
        for (i, char) in s.chars().enumerate() {
            if next_space_index < spaces.len() && i == spaces[next_space_index] as usize {
                solution.push(' ');
                next_space_index += 1;
            }
            solution.push(char);
        }

        solution
    }
}


#[test]
fn test_add_spaces() {
    let s1 = String::from("LeetcodeHelpsMeLearn");
    let spaces1 = vec![8, 13, 15];
    let sol1 = String::from("Leetcode Helps Me Learn");

    let s2 = String::from("icodeinpython");
    let spaces2 = vec![1, 5, 7, 9];
    let sol2 = String::from("i code in py thon");

    let s3 = String::from("spacing");
    let spaces3 = vec![0, 1, 2, 3, 4, 5, 6];
    let sol3 = String::from(" s p a c i n g");

    assert_eq!(Solution::add_spaces(s1, spaces1), sol1);
    assert_eq!(Solution::add_spaces(s2, spaces2), sol2);
    assert_eq!(Solution::add_spaces(s3, spaces3), sol3);
}
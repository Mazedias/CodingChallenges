// Leetcode Problem 3174

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut marked = vec![0; s.chars().count()];
        
        // Find all numbers and characters that have to be deleted
        for (idx, char) in s.chars().into_iter().enumerate() {
            if char.is_ascii_digit() {
                let mut marker_pos = idx;

                // Mark the found number
                marked[idx] = 1;

                // Mark the next char on the left that is not marked yet
                while marked[marker_pos] == 1 {
                    marker_pos -= 1;
                }
                marked[marker_pos] = 1;
            }
        }

        s.char_indices().filter(|(idx, _)| {
            if marked[*idx] == 1{
                return false;
            } else {
                return true;
            }
        }).map(|(_, char)| char).collect()
    }
}


#[test]
fn test_clear_digits() {
    let s1 = String::from("abc");
    let s2 = String::from("cb34");

    assert_eq!(Solution::clear_digits(s1), String::from("abc"));
    assert_eq!(Solution::clear_digits(s2), String::from(""));
}
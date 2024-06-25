// Problem 28
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_first_occurrance (haystack: String, needle: String) -> i32 {
        let mut i = 0;

        if haystack.len() < needle.len() {
            return -1;
        }

        while i <= (haystack.len() - needle.len()) {
            if &haystack[i..(i+needle.len())] == needle {
                return i as i32;
            }
            i += 1;
        }

        -1
    }
}

#[test]
pub fn test_find_first_occurrance () {
    let haystack_a = "sadbutsat".to_string();
    let needle_a = "sad".to_string();

    let haystack_b = "abc".to_string();
    let needle_b = "c".to_string();

    let haystack_c = "aaa".to_string();
    let needle_c = "aaa".to_string();

    assert_eq!(Solution::find_first_occurrance(haystack_a, needle_a), 0);
    assert_eq!(Solution::find_first_occurrance(haystack_b, needle_b), 2);
    assert_eq!(Solution::find_first_occurrance(haystack_c, needle_c), 0);
}
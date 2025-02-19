// Leetcode Problem 1415

struct Solution;

fn get_all_happy_string(n: &i32, strings: &mut Vec<String>, s: &mut String) {
    if s.len() == *n as usize {
        strings.push(s.clone());
        return;
    }
    
    for c in ['a', 'b', 'c'] {
        if let Some(&last) = s.as_bytes().last() {
            if last as char == c {
                continue;
            }
        }

        s.push(c);
        get_all_happy_string(n, strings, s);
        s.pop();
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut strings: Vec<String> = Vec::new();

        let mut s = String::from("");
        get_all_happy_string(&n, &mut strings, &mut s);

        if k as usize > strings.len() {
            return "".to_string();
        } else {
            strings[k as usize - 1].clone()
        }
    }
}


#[test]
fn test_get_happy_string() {
    let expected1 = String::from("c");
    let expected2 = String::from("");
    let expected3 = String::from("cab");

    assert_eq!(Solution::get_happy_string(1, 3), expected1);
    assert_eq!(Solution::get_happy_string(1, 4), expected2);
    assert_eq!(Solution::get_happy_string(3, 9), expected3);
}
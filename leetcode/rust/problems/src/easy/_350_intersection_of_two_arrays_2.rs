use std::collections::HashMap;

// Problem 350
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn intersect (nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut solution: Vec<i32> = Vec::new();

        let (shorter, longer) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        for num in shorter {
            if map.contains_key(&num) {
                if let Some(value) = map.get_mut(&num) {
                    *value += 1;
                }
            } else {
                map.insert(num, 1);
            }
        }

        for num in longer {
            if map.contains_key(&num) {
                if let Some(value) = map.get_mut(&num) {
                    *value -= 1;
                }

                solution.insert(0, num);

                if *map.get(&num).unwrap() == 0 {
                    map.remove(&num);
                }
            }
        }

        solution
    }
}

#[test]
pub fn test_intersect () {
    let a = vec![1, 2, 2, 1];
    let b = vec![2, 2];
    let sol_a = vec![2, 2];

    let c = vec![4, 9, 5];
    let d = vec![9, 4, 9, 8, 4];
    let sol_b = vec![4, 9];

    assert_eq!(Solution::intersect(a, b), sol_a);
    assert_eq!(Solution::intersect(c, d), sol_b);
}
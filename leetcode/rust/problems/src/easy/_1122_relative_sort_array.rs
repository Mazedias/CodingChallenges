use std::collections::HashMap;

// Problem 1122
struct Solution;

#[allow(dead_code)]
    impl Solution {
        pub fn relative_sort_array (arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
            let mut map = HashMap::new();
            let mut solution = Vec::new();

            for i in arr2.iter() {
                map.insert(i, 0);
            }

            for i in arr1 {
                if map.contains_key(&i) {
                    *map.get_mut(&i).unwrap() += 1;
                } else {
                    solution.insert(0, i);
                }
            }

            solution.sort();

            for i in arr2.iter().rev() {
                for _ in 0..*map.get(&i).unwrap() {
                    solution.insert(0, *i);
                }
            }

            solution
        }
    }

#[test]
pub fn test_relative_sort_array () {
    let arr_1a = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr_1b = vec![2, 1, 4, 3, 9, 6];
    let arr_1sol = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];

    let arr_2a = vec![28, 6, 22, 8, 44, 17];
    let arr_2b = vec![22, 28, 8, 6];
    let arr_2sol = vec![22, 28, 8, 6, 17, 44];

    assert_eq!(Solution::relative_sort_array(arr_1a, arr_1b), arr_1sol);
    assert_eq!(Solution::relative_sort_array(arr_2a, arr_2b), arr_2sol);
}
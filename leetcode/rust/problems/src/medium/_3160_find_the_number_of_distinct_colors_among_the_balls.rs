// Leetcode Problem 3160

use std::collections::{hash_map::Entry, HashMap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    
    pub fn query_results(_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_colors = HashMap::new();
        let mut color_counts = HashMap::new();

        queries
            .into_iter()
            .map(move |query| {
                match ball_colors.entry(query[0]) {
                    Entry::Vacant(ball_color) => {
                        ball_color.insert(query[1]);
                        let color_count = color_counts.entry(query[1]).or_insert(0);
                        *color_count += 1;
                    }
                    Entry::Occupied(mut ball_color) => {
                        let old_color = std::mem::replace(ball_color.get_mut(), query[1]);

                        if old_color != query[1] {
                            let color_count = color_counts.entry(query[1]).or_insert(0);
                            *color_count += 1;

                            let Entry::Occupied(mut color_count) = color_counts.entry(old_color)
                            else {
                                unreachable!("missing color");
                            };

                            *color_count.get_mut() -= 1;
                            if *color_count.get() == 0 {
                                color_count.remove();
                            }
                        }
                    }
                }

                color_counts.len() as i32
            })
            .collect()
    }
}


#[test]
fn test_query_results() {
    let queries1 = vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]];
    let queries2 = vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]];

    let expected1 = vec![1, 2, 2, 3];
    let expexted2 = vec![1, 2, 2, 3, 4];

    assert_eq!(Solution::query_results(4, queries1), expected1);
    assert_eq!(Solution::query_results(4, queries2), expexted2);
}
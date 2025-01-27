// Leetcode Problem 1462

use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {

    // A faster solution but not my own
    pub fn check_if_prerequisite(num_courses: i32, prerequisities: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let (mut graph, mut in_degree) = prerequisities
            .into_iter()
            .fold(([0_u128; 100], [0_i32; 100]), |(mut graph, mut in_degree), prereq| {
                let (a, b) = (prereq[0], prereq[1]);
                in_degree[a as usize] += 1;
                graph[b as usize] |= 1 << a;
                (graph, in_degree)
            });

        let mut zero_in_degree = in_degree
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, d)| d==0)
            .map(|(i, _)| i as i32)
            .collect::<VecDeque<_>>();

        let mut is_prereq = [0_u128; 100];
        std::iter::successors(zero_in_degree.pop_front(), |&n| {
            let adjs = graph[n as usize];
            (0..num_courses)
                .filter(|adj| (1 << adj) & adjs != 0)
                .for_each(|adj| {
                    is_prereq[adj as usize] |= 1 << n;
                    is_prereq[adj as usize] |= is_prereq[n as usize];
                    in_degree[adj as usize] -= 1;
                    if in_degree[adj as usize] == 0 {
                        zero_in_degree.push_back(adj);
                    }
                });
                zero_in_degree.pop_front()
        })
        .last();

        queries.into_iter().map(|q| {
            let (u, v) = (q[0], q[1]);
            is_prereq[u as usize] & (1 << v) != 0
        }).collect()
        
    }

    // My solution, works but is very slow
    pub fn check_if_prerequisite_slow(num_courses: i32, prerequisities: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut answer = vec![false; queries.len()];
        
        let mut reachable: Vec<Vec<bool>> = vec![vec![false; num_courses as usize]; num_courses as usize];

        let mut q: VecDeque<i32> = VecDeque::new();
        for i in 0..num_courses {
            q.push_back(i);

            while let Some(next) = q.pop_front() {
                for pre in &prerequisities {
                    if pre[0] == next {
                        reachable[i as usize][pre[1] as usize] = true;
                        q.push_back(pre[1]);
                    }
                }
            }
        }

        for (idx, query) in queries.iter().enumerate() {
            if reachable[query[0] as usize][query[1] as usize] {
                answer[idx] = true;
            }
        }

        answer
    }
}


#[test]
fn test_check_if_prerequisite() {
    let prerequisites1 = vec![vec![1, 0]];
    let queries1 = vec![vec![0, 1], vec![1, 0]];
    let expected1 = vec![false, true];

    let prerequisites2 = vec![];
    let queries2 = vec![vec![1, 0], vec![0, 1]];
    let expected2 = vec![false, false];

    let prerequisites3 = vec![vec![1, 2], vec![1, 0], vec![2, 0]];
    let queries3 = vec![vec![1, 0], vec![1, 2]];
    let expected3 = vec![true, true];

    let prerequisites4 = vec![vec![1, 0], vec![0, 2], vec![3, 5], vec![5, 6], vec![5, 7], vec![7, 8]];
    let queries4 = vec![vec![1, 2], vec![2, 1], vec![3, 8], vec![3, 6], vec![5, 6], vec![5, 8], vec![0, 8], vec![8, 6]];
    let expected4 = vec![true, false, true, true, true, true, false, false];

    assert_eq!(Solution::check_if_prerequisite(2, prerequisites1, queries1), expected1);
    assert_eq!(Solution::check_if_prerequisite(2, prerequisites2, queries2), expected2);
    assert_eq!(Solution::check_if_prerequisite(3, prerequisites3, queries3), expected3);
    assert_eq!(Solution::check_if_prerequisite(9, prerequisites4, queries4), expected4);
}
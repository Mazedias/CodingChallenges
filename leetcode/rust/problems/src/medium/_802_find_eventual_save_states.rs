// Leetcode Problem 802

use std::collections::VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    // This is my solution but its very slow
    pub fn eventual_safe_nodes(mut graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();

        let mut save_nodes: Vec<i32> = Vec::new();
        let mut q: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            if graph[i].len() == 0 {
                q.push_back(i);
            }
        }

        while let Some(node) = q.pop_front() {
            save_nodes.push(node as i32);

            for i in 0..n {
                if i == node {
                    continue;
                }

                for (idx, num) in graph[i].iter().enumerate() {
                    if *num == node as i32 {
                        graph[i].remove(idx);
                        
                        if graph[i].len() == 0 {
                            q.push_back(i);
                        }

                        break;
                    }
                }
            }
        }

        save_nodes.sort();
        save_nodes
    }

    pub fn eventual_safe_nodes_fast(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut states = vec![0; graph.len()];

        for i in 0..graph.len() {
            recur(&graph, i, &mut states);
        }

        states.iter().enumerate().filter_map(|(i, x)| if *x == 2 {Some(i as i32)} else {None}).collect()
    }
}

pub fn recur(graph: &Vec<Vec<i32>>, ndx: usize, states: &mut Vec<i32>) -> i32 {
    if states[ndx] != 0 {
        return states[ndx];
    }

    states[ndx] = 1;

    for k in &graph[ndx] {
        if states[*k as usize] == 1 || recur(graph, *k as usize, states) != 2 {
            return 1;
        }
    }

    states[ndx] = 2;
    2
}


#[test]
fn test_eventual_safe_nodes() {
    let graph1 = vec![
        vec![1, 2],
        vec![2, 3],
        vec![5],
        vec![0],
        vec![5],
        vec![],
        vec![]
    ];

    let graph2 = vec![
        vec![1, 2, 3, 4],
        vec![1, 2],
        vec![3, 4],
        vec![0, 4],
        vec![]
    ];

    let graph3 = vec![
        vec![1, 8],
        vec![2, 3],
        vec![3, 4],
        vec![4, 5],
        vec![],
        vec![6, 7],
        vec![7],
        vec![8],
        vec![]
    ];

    let graph4 = vec![
        vec![1],
        vec![2, 3],
        vec![0, 3],
        vec![]
    ];

    let graph5 = vec![
        vec![1],
        vec![2],
        vec![0]
    ];

    assert_eq!(Solution::eventual_save_nodes(graph1), vec![2, 4, 5, 6]);
    assert_eq!(Solution::eventual_save_nodes(graph2), vec![4]);
    assert_eq!(Solution::eventual_save_nodes(graph3), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(Solution::eventual_save_nodes(graph4), vec![3]);
    assert_eq!(Solution::eventual_save_nodes(graph5), vec![]);
}
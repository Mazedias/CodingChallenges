// Leetcode Problem 2467

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();

        // Create adjacency map for more efficient access to edges of a node
        for edge in edges {
            adj_list.entry(edge[0]).and_modify(|v| v.push(edge[1])).or_insert_with(Vec::new).push(edge[1]);
            adj_list.entry(edge[1]).and_modify(|v| v.push(edge[0])).or_insert_with(Vec::new).push(edge[0]);
        }

        // Find the path Bob takes as preparation of the DFS we preform afterwards for Alices path
        let mut parents: HashMap<i32, i32> = HashMap::new();
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(0);

        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj_list[&node] {
                if !parents.contains_key(&neighbor) {
                    parents.insert(neighbor, node);
                    queue.push_back(neighbor);

                    if neighbor == bob {
                        break;
                    }
                }
            }
        }

        // With the generated parent map we can now find the path bob takes
        let mut bob_arrival: HashMap<i32, i32> = HashMap::new();
        let mut node = bob;
        let mut time = 0;
        while node != 0 {
            bob_arrival.insert(node, time);
            node = parents[&node];
            time += 1;
        }

        // Preform DFS from the root node to calculate the distance of each node to the root
        // Meanwhile calcualte the profit of the path. 
        let mut path_alice: Vec<(i32 ,i32, i32)> = vec![(0, 0, amount[0])];
        let mut visited: HashSet<i32> = HashSet::new();
        visited.insert(0);

        let mut max_profit = i32::MIN;

        while let Some((node, alice_time, profit)) = path_alice.pop() {
            let mut is_leaf = true;

            for &neighbor in &adj_list[&node] {
                if !visited.contains(&neighbor) {
                    is_leaf = false;
                    visited.insert(neighbor);

                    // Calculate the profit of this node considering the possibility of bobs presence
                    let mut new_profit = profit + amount[neighbor as usize];

                    if let Some(&bob_time) = bob_arrival.get(&neighbor) {
                        if alice_time + 1 > bob_time {
                            // Alice reaches the node after bob took the profit from it
                            new_profit = profit;
                        } else if alice_time + 1 == bob_time {
                            // Alice reaches the node together with bob
                            new_profit = profit + (amount[neighbor as usize] / 2);
                        }
                    }

                    path_alice.push((neighbor, alice_time + 1, new_profit));
                }
            }

            if is_leaf {
                max_profit = max_profit.max(profit);
            }
        }

        max_profit
    }
}


#[test]
fn test_most_profitable_path() {
    let edges1 = vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]];
    let amount1 = vec![-2, 4, 2, -4, 6];

    let edges2 = vec![vec![0, 1]];
    let amount2 = vec![-7280, 2350];

    let edges3 = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let amount3 = vec![-5644, -6018, 1188, -8520];

    assert_eq!(Solution::most_profitable_path(edges1, 3, amount1), 6);
    assert_eq!(Solution::most_profitable_path(edges2, 1, amount2), -7280);
    assert_eq!(Solution::most_profitable_path(edges3, 3, amount3), -11662);
}
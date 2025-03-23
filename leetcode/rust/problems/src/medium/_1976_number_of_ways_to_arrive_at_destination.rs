// Leetcode Problem 1976

use std::{cmp::Reverse, collections::BinaryHeap};

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        // Use Dijkstra's algorithm to find the shortest path to every node in the graph
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        // Build a adjacency list
        for road in &roads {
            let (u, v, cost) = (road[0] as usize, road[1] as usize, road[2]);
            graph[u].push((v, cost));
            graph[v].push((u, cost));
        }

        // To store the minimal distances to each node and the ways to reach the nodes with minimal cost
        let mut distances = vec![i64::MAX; n];
        let mut ways = vec![0; n];

        distances[0] = 0;
        ways[0] = 1;

        // Priority Queue (Min Heap) for looking at the node with minimal costs next
        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0 ,0)));

        // Iterate through the nodes and neighbors and check if the distance must be updated
        while let Some(Reverse((current_distance, node))) = pq.pop() {
            if current_distance > distances[node] {
                continue;
            }

            for &(neighbor, cost) in &graph[node] {
                let new_distance = current_distance + cost as i64;

                if new_distance < distances[neighbor] { 
                    // Found a shorter path to node the neighbor
                    // Update the distance and set the amount of ways to the ways one can reach
                    // 'node' because only via 'node' does one reach the node 'neighbor' with minimal cost
                    distances[neighbor] = new_distance;
                    ways[neighbor] = ways[node];
                    pq.push(Reverse((new_distance, neighbor)));
                } else if new_distance == distances[neighbor] {
                    // If the found distance is equal update the amount of ways since we just found new ways
                    // with minimal distance to 'neighbor' via 'node' 
                    ways[neighbor] = (ways[neighbor] + ways[node]) % 1_000_000_007;
                }
            }
        }
        
        ways[n-1]
    }
}

#[test]
pub fn test_count_paths() {
    let roads1: Vec<Vec<i32>> = vec![
        vec![0, 6, 7], 
        vec![0, 1, 2],
        vec![1, 2, 3], 
        vec![1, 3, 3], 
        vec![6, 3, 3], 
        vec![3, 5, 1], 
        vec![6, 5, 1], 
        vec![2, 5, 1],
        vec![0, 4, 5], 
        vec![4, 6, 2]
    ];

    let roads2 = vec![vec![1, 0, 10]];

    assert_eq!(Solution::count_paths(7, roads1), 4);
    assert_eq!(Solution::count_paths(2, roads2), 1);
}
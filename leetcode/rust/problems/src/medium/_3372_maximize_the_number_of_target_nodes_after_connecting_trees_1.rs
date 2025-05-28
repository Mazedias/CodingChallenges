// Leetcode Problem 3372

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // Get the amount of reachable nodes in the first tree
        let mut ans = Self::in_range_k(&edges1, k);
        
        if k > 0 {
            // Calculate the node of the second graph with the most reachable nodes within depth k-1
            let d = Self::in_range_k(&edges2, k-1).into_iter().max().unwrap();
            
            // Add this amount to each node
            ans.iter_mut().for_each(|a| *a+=d);
        }
        ans
    }

    // Method to prepare a adjacency vector and then call DFS and collection a vector containing the amount of reachable nodes for each node
    fn in_range_k(edges: &[Vec<i32>], k: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut neis = vec![vec![]; n];
        for e in edges {
            neis[e[0] as usize].push(e[1]);
            neis[e[1] as usize].push(e[0]);
        }
        (0..n as i32).map(|i| Self::dfs(i, -1, k, &neis)).collect()
    }

    // DFS that searches how meany nodes are reachable from the starting node and only searches to depth k
    fn dfs(i: i32, previous: i32, k: i32, neis: &[Vec<i32>]) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut reachable = 1;
        for &ne in &neis[i as usize] {
            if ne != previous {
                reachable += Self::dfs(ne, i, k-1, neis);
            }
        }
        reachable
    }
}


#[test]
fn test_max_target_nodes() {
    // TODO: Add test cases
}
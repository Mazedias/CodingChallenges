// Leetcode Problem 909

use std::collections:: VecDeque;

struct Solution;

#[allow(dead_code)]
impl Solution {
    // My solution, works but exceeds memory on leetcode
    pub fn my_snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        // Since from each node there is only one path we can go we can use a vector for the adjacency map
        let mut map = vec![-1; n*n];

        // Build the adjacency map by iterating through the board in Boustrophedon style
        for (i, row) in board.iter().rev().enumerate() {
            if i % 2 != 0 {
                for (j, col) in row.iter().rev().enumerate() {
                    let node = i * n + j;
                    if *col != -1 {
                        map[node] = *col - 1;
                        continue;
                    }
                    map[node] = node as i32 + 1;
                }
            } else {
                for (j, col) in row.iter().enumerate() {
                    let node = i * n + j;
                    if *col != -1 {
                        map[node] = *col - 1;
                        continue;
                    }
                    map[node] = node as i32 + 1;
                }
            }
        }

        // Now we preform BFS on the adjacency map to find the least amount of rolls we need to get to the final square
        let mut visited = vec![false; n*n];
        let mut q = VecDeque::new();

        q.push_back((0, 0));

        while let Some((node, rolls)) = q.pop_front() {
            // If we reached the final square return the amount of needed rolls
            if node == n*n - 1 { // n*n-1 since the board is 1-indexed but here i worked 0-indexed
                return rolls;
            }

            // Else queue the next 6 (or less) squares that we could reach from the current square with one roll
            for i in 1..=6 {
                let mut next = node + i;
                if next >= n * n {
                    break;
                }

                // Check if we would land on a snake/ladder square and have to jump
                if map[next] != -1 && map[next] != next as i32 + 1 {
                    next = map[next] as usize;
                }

                if visited[next] {
                    continue;
                }
                q.push_back((next, rolls+1));
            }
            visited[node] = true;
        }

        -1
    }

    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        /// Method to get the index into the board of size n for a specific square 
        /// Needed because the board is arranged following the Boustrophedon style
        fn calc_pos(square: usize, n:usize) -> (usize, usize) {
            let row = square / n;
            let col = square - row * n;
            return ((n-1)-row, if row % 2 == 0 {col} else {(n-1)-col});
        }

        // Initialize everything needed for BFS
        let n = board.len();
        let mut queue: VecDeque<(i32, i32)> = VecDeque::from_iter(vec![(1, 0)]);
        let mut visited = vec![false; n*n+1];

        while let Some((square, dist)) = queue.pop_front() {
            if square as usize == n*n {
                return dist;
            }

            for idx in 1..=6 {
                if square as usize + idx > n*n {
                    break;
                }

                // Get the position of the square in the board matrix
                let (row, col) = calc_pos(square as usize + idx - 1, n);

                // Calculate the next square we would land on
                let dest = if board[row][col] == -1 {square + idx as i32} else {board[row][col]};

                if visited[dest as usize] {
                    continue;
                }

                queue.push_back((dest, dist + 1));
                visited[dest as usize] = true;
            }
        }

        -1
    }
}

#[test]
fn test_snakes_and_ladders() {
    let board1 = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1]
    ];

    let board2 = vec![
        vec![-1, -1],
        vec![-1, 3]
    ];

    let board3 = vec![
        vec![1, -1],
        vec![-1, -1]
    ];

    let board4 = vec![
        vec![-1, 1, 1, 1, 1, 1, 1],
        vec![-1, -1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1, -1],
    ];

    let board5 = vec![
        vec![-1, 1, 2, -1],
        vec![2, 13, 15, -1],
        vec![-1, 10, -1, -1],
        vec![-1, 6, 2, 8]
    ];

    let board6 = vec![
        vec![-1,-1,-1,30,-1,144,124,135,-1,-1,-1,-1,-1],
        vec![-1,-1,-1,-1,-1,-1,167,93,-1,-1,-1,-1,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,77,-1,-1,90,-1,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,122,-1],
        vec![-1,-1,-1,23,-1,-1,-1,-1,-1,155,-1,-1,-1],
        vec![-1,-1,140,29,-1,-1,-1,-1,-1,-1,-1,-1,-1],
        vec![-1,-1,-1,-1,-1,115,-1,-1,-1,109,-1,-1,5],
        vec![-1,57,-1,99,121,-1,-1,132,-1,-1,-1,-1,-1],
        vec![-1,48,-1,-1,-1,68,-1,-1,-1,-1,31,-1,-1],
        vec![-1,163,147,-1,77,-1,-1,114,-1,-1,80,-1,-1],
        vec![-1,-1,-1,-1,-1,57,28,-1,-1,129,-1,-1,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,-1,-1,87,-1,-1,-1]
    ];

    assert_eq!(Solution::snakes_and_ladders(board1), 4);
    assert_eq!(Solution::snakes_and_ladders(board2), 1);
    assert_eq!(Solution::snakes_and_ladders(board3), -1);
    assert_eq!(Solution::snakes_and_ladders(board4), -1);
    assert_eq!(Solution::snakes_and_ladders(board5), 2);
    assert_eq!(Solution::snakes_and_ladders(board6), 6);
}
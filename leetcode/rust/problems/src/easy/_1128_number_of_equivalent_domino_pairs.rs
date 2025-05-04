// Leetcode Problem 1128

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; 100];
        let mut res = 0;
        for d in dominoes {
            let val = if d[0] < d[1] { d[0] * 10 + d[1] } else { d[1] * 10 + d[0] };
            res += count[val as usize];
            count[val as usize] += 1;
        }
        res
    }
}


#[test]
fn test_num_equiv_domino_pairs() {
    let dominoes1 = vec![vec![1,2],vec![2,1],vec![3,4],vec![5,6]];
    let dominoes2 = vec![vec![1,2],vec![1,2],vec![1,1],vec![1,2],vec![2,2]];

    assert_eq!(Solution::num_equiv_domino_pairs(dominoes1), 1);
    assert_eq!(Solution::num_equiv_domino_pairs(dominoes2), 3);
}
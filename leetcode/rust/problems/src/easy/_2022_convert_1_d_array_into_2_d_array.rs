// Problem 2022
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn construct2_d_array (original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if n * m != (original.len() as i32) {
            return Vec::new();
        }

        let mut solution: Vec<Vec<i32>> = Vec::new();
        let mut lower: usize = 0;
        let mut upper: usize = n as usize;

        for i in 1..=m {
            solution.push(original[lower..upper].to_vec());

            lower = upper;
            upper = ((i + 1) * n) as usize; 
        }


        solution
    }
}

#[test]
pub fn test_construct2_d_array () {
    let a = vec![1, 2, 3, 4];
    let a_sol = vec![[1,2], [3,4]];
    
    let b = vec![1, 2, 3];
    let b_sol = vec![[1,2,3]];

    let c = vec![1, 2];
    let c_sol: Vec<[i32; 0]> = vec![];

    let d = vec![1, 1, 1, 1];
    let d_sol = vec![[1], [1], [1], [1]];

    assert_eq!(Solution::construct2_d_array(a, 2, 2), a_sol);
    assert_eq!(Solution::construct2_d_array(b, 1, 3), b_sol);
    assert_eq!(Solution::construct2_d_array(c, 1, 1), c_sol);
    assert_eq!(Solution::construct2_d_array(d, 4, 1), d_sol);
}
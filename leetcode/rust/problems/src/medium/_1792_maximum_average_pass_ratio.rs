// Leetcode Problem 1792
use std::{cmp::Ordering, collections::BinaryHeap};

struct Class {
    gain: f64,
    pass: i32, 
    total: i32
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gain.partial_cmp(&other.gain)
    }
}

impl Ord for Class {
    fn cmp (&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        // Calculate how much ratio we would gain for a specific setup of pass/total 
        fn gain(pass: i32, total: i32) -> f64 {
            (pass + 1) as f64 / (total + 1) as f64 - pass as f64 / total as f64
        }

        let mut heap = BinaryHeap::new();
        for class in &classes {
            heap.push(Class {gain: gain(class[0], class[1]), pass: class[0], total: class[1]});
        }

        let mut extra = extra_students;
        while extra > 0 {
            let mut top = heap.pop().unwrap();
            top.pass += 1;
            top.total += 1;
            top.gain = gain(top.pass, top.total);
            heap.push(top);
            extra -= 1;
        }

        let mut sum = 0.0;
        while let Some(class) = heap.pop() {
            sum += class.pass as f64 / class.total as f64;
        }
        sum / classes.len() as f64
    }
}


#[test]
fn test_max_average_ratio() {
    let tolerance = 1e-5;

    let classes1 = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
    let expected1 = 0.78333;
    let classes2 = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
    let expected2 = 0.53485;

    assert!((Solution::max_average_ratio(classes1, 2) - expected1).abs() < tolerance);
    assert!((Solution::max_average_ratio(classes2, 4) - expected2).abs() < tolerance);
}
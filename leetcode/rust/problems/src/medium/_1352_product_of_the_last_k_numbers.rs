// Leetcode Problem 1352

struct ProductOfNumbers {
    nums: Vec<i32>
}

#[allow(dead_code)]
impl ProductOfNumbers {
    
    fn new() -> Self {
        ProductOfNumbers {
            nums: Vec::new()
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
            return;
        }

        let last_product = if self.nums.is_empty() { &1 } else { self.nums.last().unwrap() };

        self.nums.push(last_product * num);
    }

    fn get_product(&self, k: i32) -> i32 {
        if k as usize > self.nums.len() {
            return 0;
        }

        if k as usize == self.nums.len() {
            return *self.nums.last().unwrap();
        }

        *self.nums.last().unwrap() / self.nums[self.nums.len() - 1 - k as usize]
    }

    // My Solution, but is very slow
    fn add_bruteforce_approch(&mut self, num: i32) {
        self.nums.push(num);
    }

    // My Solution, but is very slow
    fn get_product_bruteforce_approach(&self, k: i32) -> i32 {
        let mut product = 1;

        for i in self.nums.len().saturating_sub(k as usize)..self.nums.len() {
            product *= self.nums[i as usize];
        }

        product
    }
}

#[test]
pub fn test_product_of_numbers() {
    let mut p1 = ProductOfNumbers::new();
    p1.add(3);
    p1.add(0);
    p1.add(2);
    p1.add(5);
    p1.add(4);
    assert_eq!(p1.get_product(2), 20);
    assert_eq!(p1.get_product(3), 40);
    assert_eq!(p1.get_product(4), 0);
    p1.add(8);
    assert_eq!(p1.get_product(2), 32);
}
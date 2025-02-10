// Leetcode Probelm 2349

use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    index_to_number: HashMap<i32, i32>,
    number_to_indices: HashMap<i32, BTreeSet<i32>>,
}

#[allow(dead_code)]
impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            index_to_number: HashMap::new(),
            number_to_indices: HashMap::new(),
        }
    }
    
    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old_number) = self.index_to_number.get(&index) {
            let indices = self.number_to_indices.get_mut(&old_number).unwrap();
            indices.remove(&index);
            if indices.is_empty() {
                self.number_to_indices.remove(&old_number);
            }
        }
        
        self.index_to_number.insert(index, number);
        self.number_to_indices
            .entry(number)
            .or_insert_with(BTreeSet::new)
            .insert(index);
    }
    
    fn find(&self, number: i32) -> i32 {
        self.number_to_indices
            .get(&number)
            .and_then(|indices| indices.iter().next())
            .copied()
            .unwrap_or(-1)
    }
}
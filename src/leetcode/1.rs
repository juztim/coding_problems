use std::collections::HashMap;
use std::io;

fn main() {
    two_sum(vec![3, 2, 4], 6);
}

// easier solution 23ms 2.3MB Memory
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (idx, i) in nums.iter().enumerate() {
        for (ndx, n) in nums.iter().enumerate() {
            if i + n == target && idx != ndx {
                return vec![idx as i32, ndx as i32];
            }
        }
    }
    unreachable!()
}

// faster solution with hashmaps 0.1ms 5.6MB Memory
pub fn two_sum_fast(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sum_to_idx = HashMap::<i32, usize>::new();
    for (idx, num) in nums.into_iter().enumerate() {
        let expected_sum = target - num;

        match sum_to_idx.get(&expected_sum) {
            Some(&prev_idx) => return vec![idx as i32, prev_idx as i32],
            _ => {
                sum_to_idx.insert(num, idx);
            }
        }
    }
    unreachable!()
}

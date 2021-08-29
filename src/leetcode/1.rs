use std::io;

fn main() {
    two_sum(vec![3, 2, 4], 6);
}

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

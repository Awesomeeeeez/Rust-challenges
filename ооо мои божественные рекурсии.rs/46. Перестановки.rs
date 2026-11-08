use std::{println};

fn main() {
    let nums = vec![1,2,3];
    println!("{:?}", permute(nums));
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();

    helper(&nums, &mut current, &mut result);
    result
}

fn helper(nums: &Vec<i32>, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }
    for j in 0..nums.len() {
        if !current.contains(&nums[j]) {
            current.push(nums[j]);
            helper(nums, current, result);
            current.pop();
        }
    } 
}
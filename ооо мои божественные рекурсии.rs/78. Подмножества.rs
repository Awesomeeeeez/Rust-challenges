use std::{println};

fn main() {
    let nums = vec![1,2,3];
    println!("{:?}", subsets(nums));
}

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    helper(&nums, 0, &mut current, &mut result);
    result
}

fn helper(nums: &Vec<i32>, i: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if i == nums.len() {
        result.push(current.clone());
        return;
    }
    current.push(nums[i]);
    helper(nums, i+1, current, result); 
    current.pop();
    helper(nums, i+1, current, result); 
}
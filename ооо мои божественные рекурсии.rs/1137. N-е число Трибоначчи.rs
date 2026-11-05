use std::println;

fn main() {
    println!("{}", tribonacci(37));
}

pub fn tribonacci(n: i32) -> i32 {
    let mut nums = vec![None; 38];
    checker(&mut nums, n)
}

fn checker(nums: &mut Vec<Option<i32>>, n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    } else {
        if nums[n as usize].is_none() {
            nums[n as usize] = Some(checker(nums, n-1) + checker(nums, n-2) + checker(nums, n-3));
            return nums[n as usize].unwrap();
        } else {
            return nums[n as usize].unwrap();
        }
    }
}
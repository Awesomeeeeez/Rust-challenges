use std::println;

fn main() {
    println!("{}", climb_stairs(45));
}

fn climb_stairs(n: i32) -> i32 {
    let mut nums = vec![None; 45];
    checker(&mut nums, n)
}

fn checker(nums: &mut Vec<Option<i32>>, n: i32) -> i32 {
    if n == 1 {
        return 1;
    } 
    if n == 2 {
        return 2;
    }
    else {
        if nums[n as usize - 1].is_none() {
            nums[n as usize - 1] = Some(checker(nums, n-1) + checker(nums, n-2));
            return nums[n as usize - 1].unwrap();
        } else {
            return nums[n as usize - 1].unwrap();
        }
    }
}

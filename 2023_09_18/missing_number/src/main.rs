// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

fn missing_number(nums: Vec<i32>) -> i32 {
    (1..=nums.len() as i32).sum::<i32>() - nums.iter().sum::<i32>()
}

fn main() {
    let nums = vec![3, 0, 1];
    println!("{}", missing_number(nums));
}

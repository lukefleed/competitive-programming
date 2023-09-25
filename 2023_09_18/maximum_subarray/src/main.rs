// Given an integer array nums, find the subarray  with the largest sum, and return its sum.
// https://leetcode.com/problems/maximum-subarray/description/

fn max_sub_array(nums: &[i32]) -> i32 {
    let mut max_ending_here = nums[0];
    let mut max_so_far = nums[0];

    for &num in nums.iter().skip(1) {
        // What is the maximum sum of the subarray ending at this point?
        max_ending_here = i32::max(num, max_ending_here + num);
        // What is the maximum sum of any subarray we've seen so far?
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}

fn main() {
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let res = maxSubArray(&nums);
    println!("{}", res);
}

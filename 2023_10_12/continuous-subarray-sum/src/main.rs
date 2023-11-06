use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, -1);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if k != 0 {
                sum %= k;
            }
            match map.get(&sum) {
                Some(prev) => {
                    if i as i32 - *prev > 1 {
                        return true;
                    }
                },
                None => {
                    map.insert(sum, i as i32);
                },
            }
        }
        false
    }
}

struct Solution;

fn main() {
    let nums = vec![23, 2, 4, 6, 7];
    let k = 6;
    println!("{}", Solution::check_subarray_sum(nums, k));
}

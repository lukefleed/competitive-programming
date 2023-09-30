use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn binary_search(&self, nums: &Vec<i32>, target: i32, left_bias: bool) -> i32 {
        if nums.is_empty() {
            return -1; // Handle the case where the vector is empty.
        }

        let (mut l, mut r) = (0, nums.len() as i32 - 1); // l = left, r = right
        let mut i = -1; // i = index

        while l <= r {
            let m = l + (r - l) / 2; // m = middle for the binary search
            match target.cmp(&nums[m as usize]) {
                Ordering::Less => r = m - 1,
                Ordering::Greater => l = m + 1,
                Ordering::Equal => {
                    i = m;
                    if left_bias {
                        r = m - 1;
                    } else {
                        l = m + 1;
                    }
                }
            }
        }
        return i;
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let solution = Solution {}; // Create an instance of the Solution struct
        let left = solution.binary_search(&nums, target, true);
        let right = solution.binary_search(&nums, target, false);
        vec![left, right]
    }
}

fn main() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let result = Solution::search_range(nums, target);
    println!("{:?}", result);
}

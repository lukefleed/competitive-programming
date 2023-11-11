// impl Solution {
//     pub fn can_partition(nums: Vec<i32>) -> bool {
//         let sum: i32 = nums.iter().sum(); // sum of all elements
//         if sum % 2 != 0 { // if sum is odd, we can't partition into two equal subsets
//             return false;
//         }
//         let target = sum / 2; // target sum of each subset
//         let mut dp = vec![false; (target + 1) as usize]; // dp[i] = true if we can get sum i, false otherwise. First we initialize all to false. The size of dp is target + 1 because we need to include 0.
//         dp[0] = true; // we can always get sum 0
//         for num in nums {
//             for i in (num..=target).rev() { // we iterate from target to num because if we iterate from num to target, we will be using the same element multiple times
//                 dp[i as usize] = dp[i as usize] || dp[(i - num) as usize]; // if we can get sum i - num, we can get sum i by adding num to it
//             }
//         }
//         dp[target as usize] // return the last element of dp

//     }
// }

impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        if nums[0] > target {
            return false;
        }
        let mut dp = vec![false; (target + 1) as usize];
        dp[0] = true;
        for num in nums {
            for i in (num..=target).rev() {
                dp[i as usize] = dp[i as usize] || dp[(i - num) as usize];
            }
            if dp[target as usize] {
                return true;
            }
        }
        dp[target as usize]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans: Vec<i32> = Vec::new();
        ans.push(nums[0]);

        for &num in nums[1..].iter() {
            if num > *ans.last().unwrap() {
                ans.push(num);
            } else {
                let mut low = 0;
                let mut high = ans.len() - 1;
                while low < high {
                    let mid = low + (high - low) / 2;
                    if ans[mid] < num {
                        low = mid + 1;
                    } else {
                        high = mid;
                    }
                }
                ans[low] = num;
            }
        }
        ans.len() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jumps = 0;
        let mut cur_end = 0;
        let mut cur_farthest = 0;
        for i in 0..nums.len() - 1 {
            cur_farthest = cur_farthest.max(i + nums[i] as usize);
            if i == cur_end {
                jumps += 1;
                cur_end = cur_farthest;
            }
        }
        jumps
    }
}

struct Solution;

// write tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::jump(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::jump(vec![1, 2, 1, 1, 1]), 3);
    }
}

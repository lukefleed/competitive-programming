struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // 1. Call recursive function
        return Self::find_peak_element_recursive(&nums, 0, nums.len() - 1) as i32;
    }

    // 2. Recursive function: take the vector, and the start and end indexes as arguments
    pub fn find_peak_element_recursive(nums: &Vec<i32>, left: usize, right: usize) -> usize {
        // 3. Base case: if left and right are the same, return left
        if left == right {
            return left;
        }
        // 4. Find the middle index
        let mid = left + (right - left) / 2;
        // 5. If mid is less than mid + 1, return the recursive function, with start at mid + 1
        if nums[mid] < nums[mid + 1] {
            return Self::find_peak_element_recursive(nums, mid + 1, right);
        } else {
            // 6. Else, return the recursive function, with end at mid
            return Self::find_peak_element_recursive(nums, left, mid);
        }
    }
}

fn main() {
    println!("{}", Solution::find_peak_element(vec![1, 2, 3, 1]));
}

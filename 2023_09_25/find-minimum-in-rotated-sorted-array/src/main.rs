fn findMin(nums: Vec<i32>) -> i32 {
    let mut res = nums[0] as i32;
    // The left and right pointers of the binary search
    let (mut l, mut r) = (0, nums.len() - 1);

    // Binary search
    while l <= r {
        // If the left value is smaller than the right value,
        // then the left value is the smallest
        if nums[l] <= nums[r] {
            res = res.min(nums[l]);
            break;
        }
        // The middle value
        let mid = l + (r - l) / 2;
        // Update the smallest value
        res = res.min(nums[mid]);

        // If the left value is smaller than the middle value,
        // then the smallest value is on the right side
        if nums[l] <= nums[mid] {
            l = mid + 1;
        } else {
            // Otherwise, the smallest value is on the left side
            r = mid - 1;
        }
    }
    return res as i32;
}

fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    let res = findMin(nums);
    println!("{}", res);
}

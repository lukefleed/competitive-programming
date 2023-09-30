use std::cmp::Ordering;

fn findMin(nums: Vec<i32>) -> i32 {
    let mut res = nums[0] as i32;
    // The left and right pointers of the binary search
    let (mut l, mut r) = (0, nums.len() - 1);

    // Binary search
    while l <= r {
        match nums[l].cmp(&nums[r]) {
            Ordering::Less | Ordering::Equal => { // The array is sorted
                res = res.min(nums[l]);
                break;
            }
            Ordering::Greater => { // The array is rotated
                let mid = l + (r - l) / 2; // The middle index
                res = res.min(nums[mid]); // Update the minimum
                if nums[l] <= nums[mid] { // The left half is sorted
                    l = mid + 1;
                } else { // The right half is sorted
                    r = mid - 1;
                }
            }
        }
    }
    return res as i32;
}

fn main() {
    let nums = vec![3, 4, 5, 1, 2];
    let res = findMin(nums);
    println!("{}", res);
}

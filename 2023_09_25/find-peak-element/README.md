> **Problem**: https://leetcode.com/problems/find-peak-element/submissions/

# Comments on the solution

We can use two different approach, that are asymptotically equivalent in terms of space complexity, but in the reality the second one is more efficient, as we can see in the leetcode results

## First approach

```rust
fn find_peak_element(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left<right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[mid + 1] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left as i32;
}
```

In this case we are doing a simple binary search where we are just checking if the next element is bigger then the middle. If it is bigger, we can discard the left part of the array, otherwise we can discard the right part of the array. In this way we are sure that we will find a peak, as we are always moving towards the direction of the peak. The only case where we can have a problem is when we are in the peak, but in this case we will just return the left index, as it will be equal to the right index.

Note that it's important that $\forall i$ `nums[i] != nums[i+1]` because otherwise we can have a problem in the algorithm. This is specified in the problem statement at the end of the description

Also note that `nums[-1] = nums[n] = -∞`. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

![](https://i.imgur.com/GBiAC3c.png)

### Complexity

- **Time complexity**: `O(log(n))` because we are doing a binary search
- **Space complexity**: `O(1)` because we are not using any extra space

## Second approach

```rust
fn find_peak_element(nums: Vec<i32>) -> i32 {
    return Self::find_peak_element_recursive(&nums, 0, nums.len() - 1) as i32;
}

fn find_peak_element_recursive(nums: &Vec<i32>, left: usize, right: usize) -> usize {
    if left == right {
        return left;
    }

    let mid = left + (right - left) / 2;

    if nums[mid] < nums[mid + 1] {
        return Self::find_peak_element_recursive(nums, mid + 1, right);
    } else {
        return Self::find_peak_element_recursive(nums, left, mid);
    }
}
```

This implementation uses a recursive helper function that takes the input vector, the left and right indices of the current subarray being searched. The helper function returns the index of the peak element. The main function simply calls the helper function with the initial left and right indices. This implementation has the same time complexity as the previous one, but may use less memory due to the reduced number of variables used. However, the difference in memory usage is negligible and I did this just for obtaining an higher score in leetcode

![](https://i.imgur.com/5LfVVJu.png)

### Complexity

- **Time complexity**: `O(log(n))` because we are doing a binary search
- **Space complexity**: `O(1)` because we are not using any extra space

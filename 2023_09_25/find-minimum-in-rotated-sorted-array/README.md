> Problem: https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/submissions/

# Comments on the solution

We first initialize the minimum element as the first element of the array. Then we use binary search to find the minimum element. We compare the first element with the last element to check if the array is rotated or not. If the array is not rotated, the first element is the minimum element. If the array is rotated, we use binary search to find the minimum element. We compare the middle element with the first and last elements to determine which half of the array to search. We continue this process until we find the minimum element.

### Complexity

- **Time Complexity:** `O(log(n))`. Just binary search
- **Space Complexity:** `O(1)`. No extra space is used.

---

### Rust solution

```rust
fn findMin(nums: Vec<i32>) -> i32 {
    let mut res = nums[0] as i32;
    let (mut l, mut r) = (0, nums.len() - 1);

    while l <= r {
        if nums[l] <= nums[r] {
            res = res.min(nums[l]);
            break;
        }
        let mid = l + (r - l) / 2;
        res = res.min(nums[mid]);

        if nums[l] <= nums[mid] {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    res
}
```

### Leetcode result

![](https://i.imgur.com/JHStQsr.png)

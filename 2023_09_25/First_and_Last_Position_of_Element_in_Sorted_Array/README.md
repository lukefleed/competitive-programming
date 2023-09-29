# Comments on the solution

We want to use twice a modified version of the standard binary search algorithm in order to have the desired time complexity of `O(log n)`.

The `binary_search` function takes a reference to a vector of integers `nums`, an integer `target`, and a boolean `left_bias` as input parameters. It performs a binary search on the `nums` vector to find the index of the target value. If the target value is not found in the `nums` vector, the function returns `-1`. If the target value is found in the `nums` vector, the function returns the index of the target value. The `left_bias` parameter is used to determine whether the function should return the leftmost or rightmost index of the target value if there are multiple occurrences of the target value in the vector.

The `search_range` calls the `binary_search` function twice with different values of the `left_bias` parameter to find the leftmost and rightmost indices of the target value in the nums vector. It returns a vector containing the leftmost and rightmost indices of the target value.

### Complexity

- **Time Complexity:** `O(log n)` given by the two calls to the `binary_search` function.
- **Space Complexity:** `O(1)` since we are not using any extra space.

---

### Solution in Rust

```rust
impl Solution {
    fn binary_search(&self, nums: &Vec<i32>, target: i32, left_bias: bool) -> i32 {
        if nums.is_empty() {
            return -1; // Handle the case where the vector is empty.
        }

        let (mut l, mut r) = (0, nums.len() as i32 - 1); // l = left, r = right
        let mut i = -1; // i = index

        while l <= r {
            let m = l + (r - l) / 2; // m = middle for the binary search
            if target > nums[m as usize] {
                l = m + 1;
            } else if target < nums[m as usize] {
                r = m - 1;
            } else {
                i = m;
                if left_bias {
                    r = m - 1;
                } else {
                    l = m + 1;
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
```

### Leetcode result

![](https://i.imgur.com/A4RzXUA.png)

> **Problem**: https://leetcode.com/problems/trapping-rain-water/

# Comments on the solution

The strategy used to solve this problem is the two-pointer approach. We use two pointers, one at the beginning and one at the end of the array. We also keep track of the maximum height of the left and right sides. We then move the pointers towards each other, updating the maximum height as we go. If the maximum height of the left side is less than the maximum height of the right side, we know that we can trap water on the left side. Similarly, if the maximum height of the right side is less than the maximum height of the left side, we know that we can trap water on the right side. We keep adding the trapped water to the result until the pointers meet.

### Complexity analysis

* **Time complexity:** `O(n)` since we are iterating over the array once.
* **Space complexity:** `O(1)` since we are not using any extra space.

---

### Rust solution

```rust
fn trap(height: Vec<i32>) -> i32 {
    if height.len() == 0 {
        return 0;
    }

    let (mut l, mut r) = (0, height.len() - 1);
    let (mut l_max, mut r_max) = (height[l], height[r]);
    let mut res = 0;

    while l < r {
        if l_max < r_max {
            l += 1;
            l_max = std::cmp::max(l_max, height[l]);
            res += l_max - height[l];
        } else {
            r -= 1;
            r_max = std::cmp::max(r_max, height[r]);
            res += r_max - height[r];
        }
    }
    res
}
```

### Leetcode result

![](https://i.imgur.com/jzsT7bu.png)

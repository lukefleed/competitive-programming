# Comments on the solution

We are using a simple mathematical approach that it's pretty much self-explanatory. We can express the sum of all numbers from `0` to `n` as `n * (n + 1) / 2`. We can then subtract the sum of the array from the sum of all numbers from `0` to `n` to get the missing number.

### Complexity

- **Time Complexity:** `O(n)` - We are iterating over the array once.
- **Space Complexity:** `O(1)` - We are not using any extra space.

---

### Solution in Rust

```rust
fn missing_number(nums: Vec<i32>) -> i32 {
    (1..=nums.len() as i32).sum::<i32>() - nums.iter().sum::<i32>()
}
```

where we use the `..=` range operator to create a range from `1` to the length of the input vector, and then sum the values in that range using the `sum` method

### Leetcode result

![](https://i.imgur.com/qW2tNVm.png)

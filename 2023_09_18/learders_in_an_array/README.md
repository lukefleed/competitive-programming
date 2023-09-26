# Comments on the solution

We know for sure that there will be at least one leader in the array (the last one). We can start from the end and keep track of the maximum value we have seen so far. If the current value is greater than the maximum value, then we have found a new leader. We can store it in a list and update the maximum value. In this way, we will have the leaders in the reverse order.

So after we have iterated through all the array, we can reverse the list and return it.

### Complexity

- **Time Complexity:** `O(n)` since we iterate through the array once and reverse the list once.
- **Space Complexity:** `O(n)` since we store the leaders in a list.

---

### Solution in Rust

```rust
fn leaders_in_an_array(arr: &[i32]) -> Vec<i32> {
    let mut leaders = Vec::new();
    let mut max = arr[arr.len() - 1];
    leaders.push(max);
    for i in (0..arr.len() - 1).rev() {
        if arr[i] > max {
            max = arr[i];
            leaders.push(max);
        }
    }
    leaders.reverse();
    leaders
}
```

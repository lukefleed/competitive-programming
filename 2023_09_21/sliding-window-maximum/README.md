# Comments on the solution

The problem is to find the maximum value in a sliding window of size `k` over a given array of integers. We can solve this problem using a double-ended queue (deque) to store the indices of the elements in the sliding window. The deque will always store the indices of the elements in decreasing order of their values, with the maximum value at the front of the deque.

We start by initializing an empty deque and two pointers `l` and `r` to the left and right ends of the sliding window. We then iterate over the elements of the array, adding each element to the deque and removing the smaller elements from the back of the deque. We also remove the leftmost element from the deque if it's out of the window. Finally, we append the maximum value to the output if the window size is `k`.


### Complexity analysis

* **Time complexity**: `O(n)`, where `n` is the length of the input array. This is because each element is added to and removed from the deque at most once, and each element is processed by the while loop at most once.

* **Space complexity**: `O(k)`, where `k` is the size of the sliding window, as the deque can store at most `k` elements at any time.

---

### Rust solution

```rust
fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut q: VecDeque<usize> = VecDeque::new();
    let (mut l, mut r) = (0, 0);

    while r < nums.len() {
        while !q.is_empty() && nums[*q.back().unwrap()] < nums[r] {
            q.pop_back();
        }
        q.push_back(r);

        if l > q[0] {
            q.pop_front();
        }

        if (r + 1) as usize >= k as usize {
            output.push(nums[*q.front().unwrap()]);
            l += 1;
        }
        r += 1;
    }
    output
}
```

### Leetcode result

![](https://i.imgur.com/OR15Xve.png)

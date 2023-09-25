# Comments on the solution

We want to use Kadane's Algorithm: a dynamic programming algorithm used to find the maximum subarray sum in an array of integers. It works by iterating through the array and keeping track of the maximum sum seen so far and the maximum sum ending at the current index.

At each index, the algorithm compares the current element with the maximum sum ending at the previous index plus the current element. If the current element is greater, then the maximum sum ending at the current index is just the current element. Otherwise, the maximum sum ending at the current index is the sum of the maximum sum ending at the previous index and the current element.

The algorithm also keeps track of the maximum sum seen so far, which is the maximum of all the maximum sums ending at each index. The final result is the maximum sum seen so far.


### Coding
```rust
fn max_sub_array(nums: &[i32]) -> i32 {
    let mut max_ending_here = nums[0];
    let mut max_so_far = nums[0];

    for &num in nums.iter().skip(1) {
        max_ending_here = i32::max(num, max_ending_here + num);
        max_so_far = i32::max(max_so_far, max_ending_here);
    }

    max_so_far
}
```
### Leetcode result

![](https://i.imgur.com/enTIFEs.png)

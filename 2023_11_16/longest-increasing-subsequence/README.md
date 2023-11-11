# Longest Increasing Subsequence

Let's have a look at how this algorithm for finding the longest increasing subsequence works:

```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut ans: Vec<i32> = Vec::new();
        ans.push(nums[0]);

        for &num in nums[1..].iter() {
            if num > *ans.last().unwrap() {
                ans.push(num);
            } else {
                let mut low = 0;
                let mut high = ans.len() - 1;
                while low < high {
                    let mid = low + (high - low) / 2;
                    if ans[mid] < num {
                        low = mid + 1;
                    } else {
                        high = mid;
                    }
                }
                ans[low] = num;
            }
        }
        ans.len() as i32
    }
}
```

* It initializes an empty vector `ans` and pushes the first element of the input vector into it.

* It then iterates over the rest of the input vector. For each number:
   - If the number is greater than the last number in `ans`, it pushes the number into `ans`.
   - If the number is not greater, it performs a binary search in `ans` to find the first number that is not less than the current number and replaces it with the current number. This is done using a while loop that adjusts the `low` and `high` indices until `low` is no longer less than `high`. The loop invariant is that `ans[low]` is the first number in `ans` that is not less than the current number. The loop terminates when `low` and `high` are equal, and `low` is the index of the first number in `ans` that is not less than the current number. The current number is then inserted into `ans` at index `low`, replacing the existing number which is larger.
* Finally, it returns the length of `ans` as the length of the longest increasing subsequence.

This algorithm works because `ans` always contains the smallest tail elements for all increasing subsequences of the same length. When a new number comes in, if it is larger than all tail elements, it extends the longest increasing subsequence. If it is not, it can potentially become a tail element of an increasing subsequence of a certain length, replacing the existing larger one.

### Complexity Analysis

* Time complexity : $O(n \log n)$. Binary search takes $\log n$ time and it is called $$n$$ times.
* Space complexity : $O(n)$. The size of `ans` can grow up to $n$.

![](https://i.imgur.com/koJfK3t.png)

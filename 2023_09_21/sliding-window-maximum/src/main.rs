use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut output = Vec::new();
    let mut q: VecDeque<usize> = VecDeque::new(); // A double-ended queue implemented with a growable ring buffer.
    let (mut l, mut r) = (0, 0);

    while r < nums.len() {
        // pop smaller values from q
        while !q.is_empty() && nums[*q.back().unwrap()] < nums[r] { // q.back() is the index of the min val
            q.pop_back(); // remove smaller values
        }
        q.push_back(r); // push current val to q

        // remove left val from window
        if l > q[0] { // q[0] is the index of the max val
            q.pop_front(); // remove left val
        }

        if (r + 1) as usize >= k as usize { // window size is k
            output.push(nums[*q.front().unwrap()]); // append max val
            l += 1; // move left pointer
        }
        r += 1; // move right pointer
    }
    output
}

fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let output = max_sliding_window(nums, k);
    println!("{:?}", output);
}

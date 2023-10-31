// This first solution is O(n) time and O(n) space

pub fn subarray_sum_1(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0; // count of subarray
    let mut sum = 0; // sum of subarray
    let mut map = std::collections::HashMap::new(); // sum -> count, like Counter in python
    map.insert(0, 1); // init sum = 0, count = 1
    for i in 0..nums.len() { // iterate nums
        sum += nums[i]; // sum of subarray
        if let Some(v) = map.get(&(sum - k)) { // if sum - k in map, add count
            count += v;
        }
        *map.entry(sum).or_insert(0) += 1; // add sum to map or add count
    }
    count
}

// This second solution is O(n^2) time and O(n) space and uses prefix sums. It is slower than the first solution but more didactic and efficient memory-wise.

pub fn subarray_sum2(nums: Vec<i32>, k: i32) -> i32 {
    let mut count = 0; // count of subarrays with sum equal to k
    let prefix_sums = nums
        .iter()
        .scan(0, |sum, &element| {
            *sum += element;
            Some(*sum)
        })
        .collect::<Vec<i32>>();

    for i in 0..prefix_sums.len() {
        for j in i..prefix_sums.len() {
            let sum = if i == 0 { prefix_sums[j] } else { prefix_sums[j] - prefix_sums[i - 1] };
            if sum == k {
                count += 1;
            }
        }
    }

    count
}

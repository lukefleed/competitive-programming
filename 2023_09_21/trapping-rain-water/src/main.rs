use std::cmp::Ordering;

// Function to calculate the total amount of water that can be trapped
fn trap(height: Vec<i32>) -> i32 {
    // If the height vector is empty, return 0
    if height.len() == 0 {
        return 0;
    }

    // Initialize left and right pointers
    let (mut l, mut r) = (0, height.len() - 1);
    // Initialize the maximum height from the left and right
    let (mut l_max, mut r_max) = (height[l], height[r]);

    // Initialize the result
    let mut res = 0;

    // Loop until the left pointer is less than the right pointer
    while l < r {
        // If the maximum height from the left is less than the maximum height from the right
        if l_max < r_max {
            // Move the left pointer to the right
            l += 1;
            // Update the maximum height from the left
            l_max = std::cmp::max(l_max, height[l]);
            // Add the difference between the maximum height from the left and the current height to the result
            res += l_max - height[l];
        } else {
            // Move the right pointer to the left
            r -= 1;
            // Update the maximum height from the right
            r_max = std::cmp::max(r_max, height[r]);
            // Add the difference between the maximum height from the right and the current height to the result
            res += r_max - height[r];
        }
    }

    // Return the result
    res
}

fn main() {
    // Initialize the height vector
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    // Call the trap function and store the result
    let res = trap(height);
    // Print the result
    println!("res = {}", res);
}

use std::cmp::Ordering;

fn trap(height: Vec<i32>) -> i32 {
    // base case
    if height.len() == 0 {
        return 0;
    }

    let (mut l, mut r) = (0, height.len() - 1); // left and right pointer
    let (mut l_max, mut r_max) = (height[l], height[r]); // left and right max height of current pointer

    let mut res = 0;

    while l < r { // loop until left and right pointer meet
        match l_max.cmp(&r_max) { // compare left and right max height
            Ordering::Less => { // if left max height is less than right max height means we can trap water in left side
                l += 1; // move left pointer
                l_max = std::cmp::max(l_max, height[l]); // update left max height
                res += l_max - height[l]; // add water
            },
            _ => { // if right max height is less than left max height means we can trap water in right side
                r -= 1; // move right pointer
                r_max = std::cmp::max(r_max, height[r]); // update right max height
                res += r_max - height[r]; // add water
            }
        }
    }

    res
}

fn main() {
    let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    let res = trap(height);
    println!("res = {}", res);
}

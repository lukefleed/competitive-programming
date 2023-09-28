// Given an array A of positive integers. Your task is to find the leaders in the array. An element of array is leader if it is greater than or equal to all the elements to its right side. The rightmost element is always a leader.

fn leaders_in_an_array(arr: &[i32]) -> Vec<i32> {
    // Empty vector to store the leaders
    let mut leaders = Vec::new();
    // Set the max value to the last element of the array
    let mut max = arr[arr.len() - 1];
    // Add the max value to the leaders vector
    leaders.push(max);
    // Iterate through the array in reverse order
    for i in (0..arr.len() - 1).rev() {
        if arr[i] > max {
            // Set the max value to the current element
            max = arr[i];
            // Add the max value to the leaders vector
            leaders.push(max);
        }
    }
    leaders.reverse();
    leaders
}

fn main() {
    let arr = [16, 17, 4, 3, 5, 2];
    let leaders = leaders_in_an_array(&arr);
    println!("{:?}", leaders);
}

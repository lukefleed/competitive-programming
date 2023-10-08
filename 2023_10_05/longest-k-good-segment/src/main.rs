fn main() {
    // reads two integers n and k from the standard input.
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap(); // read line from stdin
    let mut iter = input.split_whitespace(); // split input string by whitespace
    let n: usize = iter.next().unwrap().parse().unwrap(); // parse first element to usize
    let k: usize = iter.next().unwrap().parse().unwrap(); // parse second element to usize

    // reads n integers from the standard input and stores them in an array a.
    let mut input = String::new(); // input string
    std::io::stdin().read_line(&mut input).unwrap(); // read line from stdin
    let mut iter = input.split_whitespace(); // split input string by whitespace
    let mut a: Vec<usize> = Vec::with_capacity(n); // create vector with capacity n
    for _ in 0..n {
        a.push(iter.next().unwrap().parse().unwrap()); // parse each element to usize and push to vector
    }

    let mut l = 0; // left index
    let mut r = 0; // right index
    let mut max = 0; // max length
    let mut count = 0; // count of different elements

    // We maintain a hashmap to store the count of each value in the current segment. We start with the left and right indices at 0 and keep incrementing the right index until the number of different values in the segment exceeds `k`. At this point, we increment the left index until the number of different values in the segment is less than or equal to `k`. We keep track of the maximum length of the segment seen so far and return the left and right indices of this segment.


}

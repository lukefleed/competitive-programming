fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut line = vec![0; 52];  //
    for r in ranges {
        line[r[0] as usize] += 1;
        line[r[1] as usize + 1] -= 1;
    }
    let mut overlaps = 0;
    for i in 1..=right {
        overlaps += line[i as usize];
        if i >= left && overlaps == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let left = 2;
    let right = 7;
    println!("{}", is_covered(ranges, left, right));
}

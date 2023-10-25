// https://codeforces.com/contest/616/problem/D?locale=en

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut lines = buffer.lines();
    let _ = lines.next();
    let a: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let k: i32 = lines.next().unwrap().parse().unwrap();

    let mut map = HashMap::new();
    let mut max = 0;
    let mut max_l = 0;
    let mut max_r = 0;
    let mut l = 0;
    let mut r = 0;
    while r < a.len() {
        let count = map.entry(a[r]).or_insert(0);
        *count += 1;
        while map.len() > k as usize {
            let count = map.get_mut(&a[l]).unwrap();
            *count -= 1;
            if *count == 0 {
                map.remove(&a[l]);
            }
            l += 1;
        }
        if r - l + 1 > max {
            max = r - l + 1;
            max_l = l;
            max_r = r;
        }
        r += 1;
    }
    println!("{} {}", max_l + 1, max_r + 1);
}

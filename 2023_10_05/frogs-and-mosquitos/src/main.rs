// Frogs and mosquitoes
use std::collections::HashSet;


#[derive(Debug)]
pub struct SegmentTree {
    start: usize,
    end: usize,
    max: i32,
    left: Option<Box<SegmentTree>>,
    right: Option<Box<SegmentTree>>,
}

impl SegmentTree {
    // Function to initialize a segment tree. The values of the nodes are the values of the array
    // Use the function new to create a segment tree from an array
    pub fn new(start: usize, end: usize, values: &[i32]) -> Option<Box<SegmentTree>> {
        if start > end {
            return None;
        }

        if start == end {
            // Leaf node
            return Some(Box::new(SegmentTree {
                start,
                end,
                max: values[start],
                left: None,
                right: None,
            }));
        }

        let mid = start + (end - start) / 2;

        let left_child = SegmentTree::new(start, mid, values);
        let right_child = SegmentTree::new(mid + 1, end, values);

        let max = match (&left_child, &right_child) {
            (Some(left), Some(right)) => left.max.max(right.max),
            (Some(left), None) => left.max,
            (None, Some(right)) => right.max,
            (None, None) => 0, // Default value for an empty range
        };

        Some(Box::new(SegmentTree {
            start,
            end,
            max,
            left: left_child,
            right: right_child,
        }))
    }

    // Function to initialize a segment tree with only one node with no children
    // Max value is 0
    pub fn segment_tree_no_child(start: usize, end: usize) -> Option<Box<SegmentTree>> {
        // function to initialize a segment tree. it has no children and the max value is 0
        if start > end {
            return None;
        }

        Some(Box::new(SegmentTree {
            start,
            end,
            max: 0,
            left: None,
            right: None,
        }))
    }

    // Function to insert a segment [l, r] in the SegmentTree tree
    // This function will buid a tree where each node has either 0 or 2 children
    // The max value of each node is the number of segments that overlap in the range of the node
    pub fn insert_segment(&mut self, l: usize, r: usize) {
        if r < self.start || l > self.end {
            return;
        }

        let mid = (self.start + self.end) / 2;

        // If the left and right child are not present and the range [l, r] is completely contained in the node, then we update the current node. This is the crucial part of the algorithm, since it's the only way to update the max value of the node
        if self.left.is_none() && self.right.is_none() && l <= self.start && r >= self.end {
            self.max += 1;
            return;
        }

        // If the range [l, r] is completely contained in the left child, then we insert the segment in the left child
        if r <= mid {
            match self.left.as_mut() {
                Some(left) => {
                    left.insert_segment(l, r);
                    self.max = std::cmp::max(self.max, left.max); // Update the max value of the current node
                }
                None => {
                    self.left = Some(Box::new(SegmentTree {
                        start: self.start,
                        end: mid,
                        max: self.max,
                        left: None,
                        right: None,
                    }));
                    self.left.as_mut().unwrap().insert_segment(l, r); // Insert the segment in the left child

                    // If the right child is not present, then we create it with max = self.max
                    if self.right.is_none() {
                        self.right = Some(Box::new(SegmentTree {
                            start: mid + 1,
                            end: self.end,
                            max: self.max,
                            left: None,
                            right: None,
                        }));
                    }
                    // Update the max value of the current node
                    self.max = std::cmp::max(self.max, self.left.as_ref().unwrap().max);
                }
            }
        }
        // If the range [l, r] is completely contained in the right child, then we insert the segment in the right child
        else if l > mid {
            match self.right.as_mut() {
                Some(right) => {
                    right.insert_segment(l, r);
                    self.max = std::cmp::max(self.max, right.max); // Update the max value of the current node
                }
                None => {
                    self.right = Some(Box::new(SegmentTree {
                        start: mid + 1,
                        end: self.end,
                        max: self.max,
                        left: None,
                        right: None,
                    }));
                    self.right.as_mut().unwrap().insert_segment(l, r); // Insert the segment in the right child

                    // If the left child is not present, then we create it with max = self.max
                    if self.left.is_none() {
                        self.left = Some(Box::new(SegmentTree {
                            start: self.start,
                            end: mid,
                            max: self.max,
                            left: None,
                            right: None,
                        }));
                    }
                    // Update the max value of the current node
                    self.max = std::cmp::max(self.max, self.right.as_ref().unwrap().max);
                }
            }
        }
        // If the range [l, r] is split between the left and right child, then we insert the segment in both the children split by mid
        // Same as before, if the left or right child is not present, then we create it with max = self.max
        else {
            match self.left.as_mut() {
                Some(left) => {
                    left.insert_segment(l, mid);
                    self.max = std::cmp::max(self.max, left.max);
                }
                None => {
                    self.left = Some(Box::new(SegmentTree {
                        start: self.start,
                        end: mid,
                        max: self.max,
                        left: None,
                        right: None,
                    }));
                    self.left.as_mut().unwrap().insert_segment(l, mid);

                    if self.right.is_none() {
                        self.right = Some(Box::new(SegmentTree {
                            start: mid + 1,
                            end: self.end,
                            max: self.max,
                            left: None,
                            right: None,
                        }));
                    }
                    self.max = std::cmp::max(self.max, self.left.as_ref().unwrap().max);
                }
            }
            match self.right.as_mut() {
                Some(right) => {
                    right.insert_segment(mid + 1, r);
                    self.max = std::cmp::max(self.max, right.max);
                }
                None => {
                    self.right = Some(Box::new(SegmentTree {
                        start: mid + 1,
                        end: self.end,
                        max: self.max,
                        left: None,
                        right: None,
                    }));
                    self.right.as_mut().unwrap().insert_segment(mid + 1, r);

                    if self.left.is_none() {
                        self.left = Some(Box::new(SegmentTree {
                            start: self.start,
                            end: mid,
                            max: self.max,
                            left: None,
                            right: None,
                        }));
                    }
                    self.max = std::cmp::max(self.max, self.right.as_ref().unwrap().max);
                }
            }
        }
    }

    // Function to check if in the range [i, j] there is a position p (in the range) that contains exactly k segments
    pub fn is_there(&self, i: usize, j: usize, k: i32) -> i32 {
        if i > self.end || j < self.start {
            0
        } else {
            match (&self.left, &self.right) {
                (Some(left), _) | (_, Some(left)) if left.is_there(i, j, k) == 1 => 1,
                (None, None) if self.max == k => 1,
                _ => 0,
            }
        }
    }
}


pub fn frogs_and_mosquitoes(frogs: &mut Vec<(u32, u32)>, mosquitoes: &mut Vec<(u32, u32)>) {
    // Let's maintain the set of not eaten mosquitoes
    let mut not_eaten_mosquitoes: HashSet<(u32, u32)> = HashSet::new();
    //  Also we will maintain the set of segments (ai, bi), where ai is the position of the i-th frog and bi = ai + li, where li is the current length of the tongue of the i-th frog.
    let mut segments: HashSet<(u32, u32)> = HashSet::new();
    // Let the current mosquito landed in the position x. Let's choose segment (ai, bi) with minimal ai such that bi ≥ x. If the value ai ≤ x we found the frog that will eat mosquito. Otherwise the current mosquito will not be eaten and we should add it to our set. If the i-th frog will eat mosquito then it's tongue length will be increased by the size of mosquito and we should update segment (ai, bi). After that we should choose the nearest mosquito to the right the from frog and if it's possible eat that mosquito by the i-th frog (this can be done with lower_bound in C++). Possibly we should eat several mosquitoes, so we should repeat this process several times.
    for i in 0..frogs.len() {
        segments.insert((frogs[i].0, frogs[i].0 + frogs[i].1));
    }
    for i in 0..mosquitoes.len() {
        let x = mosquitoes[i].0;
        let y = mosquitoes[i].1;
        let mut flag = false;
        for j in 0..frogs.len() {
            if frogs[j].0 <= x && x <= frogs[j].0 + frogs[j].1 {
                flag = true;
                frogs[j].1 += y;
                segments.remove(&(frogs[j].0, frogs[j].0 + frogs[j].1));
                segments.insert((frogs[j].0, frogs[j].0 + frogs[j].1));
                break;
            }
        }
        if !flag {
            not_eaten_mosquitoes.insert((x, y));
        }
    }

    // Segments (ai, bi) we can store in segment tree by position ai and value bi. Now to find segment we need we can do binary search by the value of ai and check the maximum bi value on the prefix to be at least x. This will work in O(nlog2n) time. We can improve this solution. Let's go down in segment tree in the following manner: if the maximum value bi in the left subtree of segment tree is at least x then we will go to the left, otherwise we will go to the right.

    // Let's build a segment tree with the segments
    let mut segments_tree = SegmentTree::new(0, 100000, &vec![0; 100001]).unwrap();
    for segment in segments.iter() {
        segments_tree.insert_segment(segment.0 as usize, segment.1 as usize);
    }

    // Now we can iterate over the not eaten mosquitoes and check if there is a frog that can eat them
    for mosquito in not_eaten_mosquitoes.iter() {
        let x = mosquito.0;
        let y = mosquito.1;
        if segments_tree.is_there(x as usize, x as usize, x as i32 + y as i32) == 1 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
    // The time complexity of this solution is O(nlog2n) for building the segment tree and O(n) for checking if a mosquito can be eaten by a frog

}

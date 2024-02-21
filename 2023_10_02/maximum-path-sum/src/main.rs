use std::cmp;

struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Node {
        Node {
            data: val,
            left: None,
            right: None,
        }
    }
}

fn max_path_sum_util(root: &Option<Box<Node>>, res: &mut i32) -> i32 {
    match root {
        Some(node) => {
            let ls = max_path_sum_util(&node.left, res); // computes the max path sum in the left subtree
            let rs = max_path_sum_util(&node.right, res); // computes the max path sum in the right subtree

            match (&node.left, &node.right) {
                (Some(_), Some(_)) => { // if both left and right subtrees are present
                    *res = cmp::max(*res, ls + rs + node.data); // update the result if needed as the max path sum can be in the left subtree, right subtree or the path can go through the root
                    cmp::max(ls, rs) + node.data // return the max path sum from the left or right subtree
                }
                (None, _) => rs + node.data, // if left subtree is not present, return the max path sum from the right subtree
                (_, None) => ls + node.data, // if right subtree is not present, return the max path sum from the left subtree
            }
        }
        None => 0, // if the root is None, return 0
    }
}
fn max_path_sum(root: &Option<Box<Node>>) -> i32 { // function to compute the max path sum
    let mut res = std::i32::MIN; // initialize the result with the minimum value of i32
    max_path_sum_util(root, &mut res); // call the utility function to compute the max path sum
    res // return the result
}

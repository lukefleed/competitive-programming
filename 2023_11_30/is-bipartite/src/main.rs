use std::collections::VecDeque;

struct Solution;

impl Solution {
    fn dfs(graph: &Vec<Vec<i32>>, odd: &mut Vec<i32>, i: usize) -> bool {
        if odd[i] != 0 {
            return true;
        }
        let mut q = VecDeque::new();
        odd[i] = -1; // this marks the node as visited

        while q.len() > 0 {
            let i = q.pop_front().unwrap();
            // go through all the neighbors
            for &nei in &graph[i] {
                if odd[i] == odd[nei as usize] {
                    return false;
                }
                else if odd[nei as usize] == 0 {
                    q.push_back(nei as usize);
                    odd[nei as usize] = -odd[i];
                }
            }
        }
        return true;
    }

    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut odd = vec![0; graph.len()]; // map node i to odd=1, even=-1

        for i in 0..graph.len() {
            if !Self::dfs(&graph, &mut odd, i) {
                return false;
            }
        }
        return true;
    }
}

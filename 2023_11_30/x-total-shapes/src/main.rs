struct Solution;

impl Solution {
    pub fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let rows = grid.len();
        let cols = grid[0].len();

        if i >= rows || j >= cols || grid[i][j] == '0' {
            return;
        }

        grid[i][j] = '0';

        if i > 0 { Self::dfs(grid, i - 1, j); }
        if j > 0 { Self::dfs(grid, i, j - 1); }
        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i, j + 1);
    }

    pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
         let mut count = 0;
         let rows = grid.len();
         let cols = grid[0].len();

         for i in 0..rows {
             for j in 0..cols {
                 if grid[i][j] == '1' {
                     Self::dfs(grid, i, j);
                     count += 1;
                 }
             }
         }
         count
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut grid1 = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(&mut grid1), 1);
    }

    #[test]
    fn test2() {
        let mut grid2 = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(&mut grid2), 3);
    }
}

fn main() {
}

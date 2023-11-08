impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        //  bottom-up dynamic programming approach
        // we just created a 2D grid of length len(text1)+1 x len(text2) + 1 and initializing it to all zeros
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        //  converting the strings to character vectors is to allow for easier indexing. In Rust, strings are not indexable due to their encoding in UTF-8, which means a single character can span more than one byte. When you try to index a string directly, you might end up indexing in the middle of a character, which would result in an error. By converting the string to a vector of characters, you ensure that each index corresponds to a complete character, avoiding this issue.
        let text1 = text1.chars().collect::<Vec<char>>();
        let text2 = text2.chars().collect::<Vec<char>>();

        for i in (0..text1.len()).rev() {
            for j in (0..text2.len()).rev() {
                if text1[i] == text2[j] {
                    dp[i][j] = 1 + dp[i+1][j+1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j+1], dp[i+1][j]);
                }
            }
        }

        dp[0][0]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
}

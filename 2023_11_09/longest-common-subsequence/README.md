## Technique

The technique used here is a bottom-up dynamic programming approach. The main steps are as follows:

1. **Initialization**: A 2D DP table `dp` is created with dimensions `(text1.len() + 1) x (text2.len() + 1)`. The extra row and column are for the base case when one of the strings is empty.

2. **Conversion to Character Vectors**: The strings `text1` and `text2` are converted to vectors of characters. This is done because strings in Rust are not indexable due to their UTF-8 encoding. Converting them to character vectors allows for easier indexing.

3. **Filling the DP Table**: The outer loop iterates over `text1` in reverse order, and the inner loop iterates over `text2` in reverse order. For each pair of characters `text1[i]` and `text2[j]`, it checks if they are equal. If they are equal, it increments the length of the current longest common subsequence, which is stored in `dp[i+1][j+1]`, by 1 and stores it in `dp[i][j]`. If they are not equal, it takes the maximum length of the common subsequence found so far, which is the maximum of `dp[i][j+1]` and `dp[i+1][j]`, and stores it in `dp[i][j]`.

4. **Result**: After all iterations, `dp[0][0]` will have the length of the longest common subsequence of `text1` and `text2`.

## Time and Space Complexity Analysis

**Time Complexity**: The time complexity of this approach is O(m*n), where m and n are the lengths of `text1` and `text2`, respectively. This is because each cell in the DP table is filled exactly once.

**Space Complexity**: The space complexity is also O(m*n) due to the 2D DP table. Each cell in the table stores an integer, and there are (m+1)*(n+1) cells in total.

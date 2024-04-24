# Solutions in Rust for some competitive programming problems

Created for the "Competitive Programming" course (academic year 23/24) at the Department of Computer Science of the University of Pisa.

> Course Page: https://pages.di.unipi.it/rossano/competitive/

Time spent on the project

[![wakatime](https://wakatime.com/badge/user/a3116382-7adb-43ba-9490-83130c4b22c5/project/c7f14dfa-7dc0-4759-a33e-6542dae0135e.svg)](https://wakatime.com/badge/user/a3116382-7adb-43ba-9490-83130c4b22c5/project/c7f14dfa-7dc0-4759-a33e-6542dae0135e)


## List of problems

* [x] [Kadane's Algorithm](https://leetcode.com/problems/maximum-subarray/)
  * **Idea:** We want to find the maximum subarray sum in an array of integers. It works by _iterating through the array and keeping track of the maximum sum seen so far and the maximum sum ending at the current index_. At each index, the algorithm compares the current element with the maximum sum ending at the previous index plus the current element. If the current element is greater, then the maximum sum ending at the current index is just the current element. Otherwise, the maximum sum ending at the current index is the sum of the maximum sum ending at the previous index and the current element. The algorithm also keeps track of the maximum sum seen so far, which is the maximum of all the maximum sums ending at each index. The final result is the maximum sum seen so far.

* [x] [Trapping Rain Water](https://leetcode.com/problems/trapping-rain-water/)
  * The strategy used to solve this problem is the two-pointer approach. We use _two pointers_, one at the beginning and one at the end of the array. We also keep track of _the maximum height_ of the left and right sides. We then move the pointers towards each other, updating the maximum height as we go. If the maximum height of the left side is less than the maximum height of the right side, we know that we can trap water on the left side. Similarly, if the maximum height of the right side is less than the maximum height of the left side, we know that we can trap water on the right side. We keep adding the trapped water to the result until the pointers meet.

* [x] [Search for a peak in an (unsorted) array](https://leetcode.com/problems/find-peak-element/)
  * The idea is to use a modified version of the binary search and observe that in there always will be a peak element (the worst case is a monotonically increasing/decreasing sequence, and in that case the peak will be the last/first element). So we just check if `nums[mid]` is greater than `nums[mid+1]` or not. If it is, we can discard the right part of the array, otherwise we can discard the left part of the array. We will always move towards the direction of the peak. It's important to note that `nums[-1] = nums[n] = -∞` and that `nums[i] != nums[i+1]` for all `i`.

* [ ] [Maximum Path Sum](http://practice.geeksforgeeks.org/problems/maximum-path-sum/1)
* [ ] [Hands-On 1](https://pages.di.unipi.it/rossano/blog/2023/handson12324/)
* [ ] [Frogs and Mosquitoes](http://codeforces.com/problemset/problem/609/B?locale=en)
* [ ] [Check if All the Integers in a Range Are Covered](https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/)
* [ ] [Longest k-Good Segment](https://codeforces.com/contest/616/problem/D?locale=en)
* [ ] [Continuous Subarray Sum](https://leetcode.com/problems/continuous-subarray-sum/)
* [ ] [Update the array](http://www.spoj.com/problems/UPDATEIT/)
* [ ] [Nested segments](http://codeforces.com/problemset/problem/652/D?locale=en)
* [ ] [Powerful array](http://codeforces.com/contest/86/problem/D?locale=en)
* [ ] [Longest Common Subsequence](https://leetcode.com/problems/longest-common-subsequence/)
* [ ] [Minimum Number of Jumps](https://practice.geeksforgeeks.org/problems/minimum-number-of-jumps/0)
* [ ] [Hands-On 2](https://pages.di.unipi.it/rossano/blog/2023/handson22324/)
* [ ] [Subset Sum](https://practice.geeksforgeeks.org/problems/subset-sum-problem/0)
* [ ] [Longest increasing subsequence](https://practice.geeksforgeeks.org/problems/longest-increasing-subsequence/0)
* [ ] [Longest bitonic Sequence](https://practice.geeksforgeeks.org/problems/longest-bitonic-subsequence/0)
* [ ] [N meetings in one room](http://practice.geeksforgeeks.org/problems/n-meetings-in-one-room/0)
* [ ] [Wilbur and array](http://codeforces.com/problemset/problem/596/B?locale=en)
* [ ] [Woodcutters](http://codeforces.com/problemset/problem/545/C?locale=en)
* [ ] [IsBipartite](http://practice.geeksforgeeks.org/problems/bipartite-graph/1)
* [ ] [Hands-On 3](https://pages.di.unipi.it/rossano/blog/2023/handson32324/)

## Theory and algorithms

* [ ] [Binary Search](https://pages.di.unipi.it/rossano/blog/2023/binarysearch/)
* [ ] Trees: representation, traversals, and Binary Search Tree
* [ ] [Sweep line algorithm](https://pages.di.unipi.it/rossano/blog/2023/sweepline)
* [ ] [Two Pointers Technique](https://www.geeksforgeeks.org/two-pointers-technique/)
* [ ] [Prefix Sum](https://pages.di.unipi.it/rossano/blog/2023/prefixsums)
* [ ] [Fenwick tree](https://pages.di.unipi.it/rossano/blog/2023/fenwick)
* [ ] [Range updates](https://blog.mitrichev.ch/2013/05/fenwick-tree-range-updates.html)
* [ ] [Segment Trees](https://en.wikipedia.org/wiki/Segment_tree)
* [ ] [Lazy Propagation](http://www.geeksforgeeks.org/lazy-propagation-in-segment-tree/)
* [ ] [Mo's Algorithm](https://pages.di.unipi.it/rossano/blog/2023/mosalgorithm)
* [ ] [DP](https://github.com/rossanoventurini/CompetitiveProgramming/blob/master/notes/DynamicProgramming.pdf)
* [ ] [Min cost path](https://pages.di.unipi.it/rossano/competitive/notes/Martin_Gardner_Aha_Insight_DP.pdf)
* [ ] [Greedy](https://jeffe.cs.illinois.edu/teaching/algorithms/book/04-greedy.pdf)
* [ ] [Fractional Knapsack](http://www.geeksforgeeks.org/fractional-knapsack-problem/)
* [ ] [Boxes and Heros](https://codeforces.com/blog/entry/63533)
* [ ] Graph algorithms: BFS and DFS
* [ ] Graph algorithms: Strongly Connected Components and Single-Source Shortest Path
* [ ] 	Graph algorithms: Minimum Spanning Tree (and Disjoint Sets data structures)

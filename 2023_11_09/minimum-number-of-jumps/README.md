The algorithm iterates over the `nums` vector exactly once. For each element in the `nums` vector, it performs a constant amount of work - it updates `cur_farthest` and checks if `i` is equal to `cur_end`. If `i` is equal to `cur_end`, it increments `jumps` and updates `cur_end`. All of these operations take constant time.

Therefore, the time complexity of the algorithm is O(n), where n is the length of the `nums` vector.

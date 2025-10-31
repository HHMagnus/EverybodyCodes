# Quest 1 - EniCode
Part 1: Solved directly with the given solution  
Part 2: Initial solution with a cycle detection. It can skip when a remainder has already been seen. This makes sense, since the input `mod` was always below `200`, so the pattern is expected to repeat within the first 200 runs.  
Part 3: Part 2 solution, but where the remainder for each step is memorized. When a cycle is found we add the sum of all the remainders of the cycle and multiply it by the amount of times the cycle was used.

# Quest 2 - Tangled Trees
Part 1: Solved by building the two binary trees  
Part 2: Solved by keeping a list of `swaps` that specify when two nodes are swapped.  
Part 3: Connected all connected nodes in the tree. Then traversing by going to the "partner" node if the swap has happened. This does not actually move anything, but just jumps between the two trees when swaps has happened.

# Quest 3 - The Conical Snail Clock
Part 1: Solved by going around using the `x = x+1` and `y = y-1` move. The snail is on the ring `x + y - 1`.  
Part 2: Solved by running previous until all hit `y = 1`.  
Part 3: Solved by finding the first day it hits `y = 1` then going through each set and finding the first day they both hit `y = 1` and finding the cycle by using Lowest Common Multiple.

# Run with

Run using RustRover commands or with:
```bash
cargo run --bin questX
```
Place input files in structure:
- input
  - questX
    - input1.txt
    - input2.txt
    - input3.txt
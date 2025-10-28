# Quest 1 - EniCode
Part 1: Solved directly with the given solution  
Part 2: Initial solution with a cycle detection. It can skip when a remainder has already been seen. This makes sense, since the input `mod` was always below `200`, so the pattern is expected to repeat within the first 200 runs.  
Part 3: Part 2 solution, but where the remainder for each step is memorized. When a cycle is found we add the sum of all the remainders of the cycle and multiply it by the amount of times the cycle was used.

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
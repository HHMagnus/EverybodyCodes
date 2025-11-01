# Quest 1 - Nail Down Your Luck
Part 1: Setup the grid as matrix of either a nail or not. Simulating the drop by using the coordinates given.  
Part 2: Tried the toss in all start locations and chooses the biggest one.  
Part 3: Simulate all the plays, then simulate all possible combinations of tosses, afterwards picking the min and max.  

# Quest 2 - The Pocket-Money Popper
Part 1: The balloons were setup as a stack where the top is popped. The order is setup and than it keeps popping the stacking until another balloon is found.  
Part 2: The balloons were setup in an array then it checks for the `n / 2`th element and pops if it the first balloon is the same as the fluff bolt.  
Part 3: The same solution as last time could not efficiently run as the remove `n / 2`th element will cause the memory to be resized. This can cause O(n) runtime for each remove operation. To counter this 2 x `VecDeque`s where used where the second one has the `n / 2`th element at the front. It will rebalance them when they drift in size. This required almost no new memory allocation, and runs in less than a millisecond in release mode.

# Quest 3 - The Dice that Never Lie (Unless I Tell Them To)
Part 1: Implemented the instructions as explained and rolled until the total was reached.  
Part 2: Rolled all dies until they had done the combination and sorted by the roll count.  
Part 3: Added a set of positions to represent the coins than made a set of possible moves for each die roll. In the end all the possible positions with a hit was tallied.

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
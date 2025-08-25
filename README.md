# Overview
- This repo is for personal use to create a statitically sized ring/circular buffer and queue.
- I have a use case where I want to hold N events in memory and perform some aggregations on them so often, but I dont need to hold onto to the items forever
- The idea is to preallocate some size, then push items, when capacity is reached, the tail pointer wraps around
- This inspired by rule number 3 in NASA's power of 10, where there are ho dynamic heap allocations after initilization
- Thus, the idea is allocate space upfront for some number of items, then overwrite space as the number of items becomes equal to the capacity and a statically sized window is kept without ever dynamically allocating memory.
- There is also a cache benefit here to, having all items in a contiguous memory region
- Currently, there is a buffer that leverages some unsafe regions to avoid paying for Drop when an item is overwritten, since we are ust overwriting that space anyways
- Also will see a initial attempt at a Queue.
- Both are work in progress

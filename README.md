# Advent of Code 2023

## Checking the memory address of a variable
```rust
use std::ptr;
println!("Address: {:p}", ptr::addr_of!(variable));
```

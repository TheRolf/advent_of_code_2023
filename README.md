# Advent of Code 2023

## Usage
To create empty txt files for data and template source file for day `[number]`:
```
cargo run --bin new [number]
```

To run the source file for day `[number]`:
```
cargo run --bin [number]
```

## Notes

### Checking the memory address of a variable
```rust
use std::ptr;
println!("Address: {:p}", ptr::addr_of!(variable));
```

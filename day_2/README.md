# Day 2

```bash
cargo run -- src/input.txt
```

## Part 1
Use the following sliding window:

```rust
row.windows(2).all(predicate)
```

to determine whether a row is safe.

For example, for a row to be in ascending order the predicate determines that the second value in the window is greater than the first, e.g.,

```rust
|value| value[1] >= value[0]
```

With predicates for all of the safe conditions we can iterate through all rows in the input and determine how many rows are safe.

## Part 2

Given our ability to tell whether a row is safe we can simply iterate over the row removing a given value once at a time - if the row becomes "safe" then we can stop and declare the row to be safe.

> I appreciate there is some duplicated work - a non brute-force approach would be interesting.

# Day 1

To run the solutions run the following command:

```
cargo run -- src/input.txt
```

## Part 1
1. Extract the two columns into vectors.
2. Sort the vectors.
3. Iterate through the vectors determining the difference between the two values.
4. Add the difference to a count variable `res`.

## Part 2
1. Extract the two columns into vectors.
2. Iterate through the first vector - for each value count the number of times the value appears in the vector.
3. Multiply the count by the current value.
4. Add the value to a count variable `res`.

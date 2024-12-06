# Day 6

## Part 1

Found this one fairly trivial:

1. Create functions to traverse up, down, left, and right in the grid.
2. If you successfully traverse through a position push it to a `visited` vector.
3. If you reach a position in the 0-th row or column then end the process.
4. Return the length of a set of the visited locations.

## Part 2

I found this one particularly not trivial but got there in the end:

The idea behind this is to iterate through the grid and place an obstacle if the position is in the visited list from part 1.

1. Get visited locations (same as [Part 1](#-part-1)).
2. Iterate through all positions in grid. If position was visited then place an obstacle.
3. For every grid with a new placed obstacle try and walk the new grid.
4. If you end up in a loop then this is a valid obstacle. Increment the counter by.

I naively treated a loop is instance where the obstacle had caused the path from part 1 to double in length... I was giving up....

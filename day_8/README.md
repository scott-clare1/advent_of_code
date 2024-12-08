# Day 8

Things starting to get a bit weird now...

Some nice stuff with calculating gradients and euclidian distance that was fun though.

## Part 1

1. Iterate through grid storing all points associated with a char in a hash map.
2. Perform a nested loop through each char's associated points calculating the distance between the current location (`a`) and all other locations in the vector (`b`).
3. With the distance between locations `a` and `b` move away from `a` by this distance and away from `b` by the same distance. These locations are the antinodes.

## Part 2

1. Using the same approach as [Part 1](#-part-1), except now continue moving away from points `a` and `b` until the point becomes invalid, e.g., outside of the grid.

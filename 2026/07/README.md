# Jane Street July 2026 Puzzle

## Description

The [July 2026 puzzle](https://www.janestreet.com/puzzles/pent-up-frustration-3-knight-moves-7-index/) gives an 8x8 grid, which is subdivided into twelve pentominal regions and one tetrominal region. Several squares of the grid are labeled with numbers.

The puzzle instructions indicate that the grid contains thirteen unlabeled towers, one in each region. The towers each have a height of one grid unit. A knight, starting from the bottom-left corner of the grid, has made a tour that visits each of these towers. The knight never visits a square on the grid more than once, and moves over the grid with knight's moves.

The knight keeps a score that starts from zero as it traverses the grid. On the knight's *N*<sup>th</sup> move, the score is incremented by *N* if the knight remains at the same altitude. If the knight ascends from the ground to a tower, the score is multiplied by *N*. If the knight descends from a tower to the ground, the score is divided by *N*. The knight can only descend from a tower if the current score is divisible by *N*.

The knight has recorded its score onto its position on the grid every third move for the first 18 moves, and every *K*<sup>th</sup> move for some larger value *K* subsequently, which are the numbers displayed on the grid. The puzzle asks for the knight's path to be reconstructed.

Once the path has been found, find each square that was not visited by the knight and sum the scores of the knight when it reached the orthogonally adjacent visited squares. The sum of these sums is the puzzle answer.

## Solution
 
 The solution is implemented in Rust.

 ### Usage

```console
$ cargo run --release
```

or 

```console
$ cargo build --release
$ ./target/release/jul2026
```

The solution will find and print the details of the reconstructed knight's path, along with the integer puzzle answer.

## Discussion

The most important wrinkle in the puzzle is that the knight moves over the grid in three dimensions rather than two. If the knight ascends or descends a tower, the knight's move includes the ascending or descending step as the short leg of the "L". The remaining part of the move is exactly two squares horizontally or vertically over the plane of the grid. If the knight moves from tower to tower in a single move, or from ground to ground, it remains at the same altitude and so moves in the standard "L" on grid the plane.

Directly searching for arbitrary paths and tower positions on the grid to find candidates that match the knight's scores would likely not be feasible. Instead, it's much simpler to first try to construct a list of arithmetic operations that produce the knight's known scores in some sequence. Since the scores would appear on every third operation for the first 18 and with a regular period *K* subsequently, a tree search across additions, divisions and multiplications is quickly able to find a unique list of operations where each recorded score is reached and appears at the expected steps, and the number of towers implied by the multiplication/division operations does not exceed 13. For this unique list of operations the period *K* is 7, the knight's starting position must be a tower, and only 12 towers are encountered in the steps to reach the final scored square, indicating that the list doesn't describe the complete knight's path.

Once the unique list of operations has been found, the order in which the scored squares on the board are visited is known as well as the number of moves between them. Short searches for possible sequences of moves between these scored positions can be completed quickly, and there are relatively few possibilities between each pair. Searches for sequences of moves need to take into account when a tower is ascended or descended to work properly, which can be determined from the multiplication and division operations in the operations list.

Once the complete sets of possible paths between each pair of scored positions have been found, another search can be made for constructing a full path from these potential paths in each segment of the knight's journey. Paths where any square is visited more than once can be ignored, as well as paths where the inferred tower positions place more than one tower in a single region on the grid.

For any candidate paths found between all the scored squares, it's then necessary to extend the path for the knight to reach one final tower. The possible positions of this tower will be the region where a tower has not been previously reached and in a square that the knight has not previously visited. Only one candidate path has remaining unvisited squares that make a path to this final tower possible, so there is a unique solution that reconstructs the knight's full path.

The steps of the reconstructed knight's path:

     .  . 10 14  8 15  . 53 
    11 13 41 16 37 23 35 24 
    40  . 38  9  7 25 52 54 
    12 42 17 26 22 36  . 34 
    20 39 21 47  6 49  . 51 
    43 46 27 18  2 32  3 33 
    28 19  1 45 48  5 50  . 
     0 44 29  . 30  . 31  4 



The tower positions on the grid:

    . . . T . . . . 
    . T . . . T . . 
    . . . . T . . T 
    T . . . T . . . 
    . . T . . . . . 
    . . . . T . . T 
    . . T . . . . . 
    T . . . T . . . 

The scores for each visited square:

        .     .    33   555    14    37     .  1100 
       44   541   530    53   372  2712   299   113 
      489     .   410    23   112   138  1047 59400 
      528   572    70   164  2689   335     .   264 
      127   449  2667   797    16   894     .   995 
      615   750   191    88     3   272     1  8976 
      219   107     1   704   845    10   944     . 
        0   659   248     .  7440     .   240     5 


The puzzle solution from the sums of the visited neighbors of each unvisited square is **33609**.
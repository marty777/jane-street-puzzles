# Jane Street September 2025 Puzzle

## Description

The [September 2025 puzzle](https://www.janestreet.com/puzzles/hooks-11-index/) asks for a unique arrangement of numbers on a grid within specific constraints.

Given a 9x9 grid, the puzzle asks for a partitioning into 9 L-shaped *hooks*, a 9x9 one, an 8x8 one, etc. Nine 9s, eight 8s, etc. must each be placed within separate hooks. The squares of the grid filled by digits must form a connected region, and each 2x2 region of the grid must contain at least one empty square.
 
It must also be possible to separate the filled squares of the grid into 9 distinct types of [pentomino](https://en.wikipedia.org/wiki/Pentomino) (where all pentomino shapes of a type are isomorphic up to rotation/reflection), and the sum of numbers within each pentomino must be a multiple of 5.
 
The grid gives several positions that must be filled with specific numbers. It also gives indicators in rows or columns for both numbers and pentomino types. When traversing over the row or column from the indicated starting border, the specified number or shape will be the first filled square encountered.
 
 Once a valid arrangement is found, the puzzle answer is given by taking the product of the areas of the empty regions in the grid.
 
 ## Solution
 
 The solution is implemented in Rust.

 ### Usage

```console
$ cargo build --release
$ ./target/release/sept2025 [OPTIONS]
or
$ cargo run --release -- [OPTIONS]

Options:
    -t, --threads <THREADS>  Set maximum number of worker threads. [default: 4]
    -h, --help               Print help
```
 
 ## Discussion
 
The total number of possible arrangements of hooks and positions of squares filled with numbers within hooks for a 9x9 grid is dauntingly large. However, the problem can be broken into several separate parts, and the conditions of the puzzle can be applied to reduce the search space substantially.
 
It can be observed that only the number 1 can be assigned to a 1x1 hook (since no other numbers could fit). Similarly, only the number 2 can be assigned to the three squares of the 2x2 hook (which will always wrap the filled 1x1 hook) because assigning 3 would break the 2x2 rule and no other number would fit. With these constraints, plus the specified position of a square that must be filled with a 1 and a row that must contain at least one 2, all possible hook arrangements can be iterated over quickly to find **2450** arrangements which do not violate the constraints. This number of arrangements can be further reduced by observing that 3 must be assigned to the five squares of the 3x3 hook because the specified squares for 4 and 5 and the specified column that must contain a 3 would otherwise conflict and no other numbers can fit, but this is not really required.
 
For each potential hook arrangement, all possible assignments of numbers to hooks can be iterated over. The squares on the grid occupied by specific numbers provide some required assignments, and the numbers indicated on rows or columns constrain the assignment options. Over the total 2450 possible hook arrangements, there are **339** combinations of hook arrangements and number assignments to hooks that are not ruled out by the constraints of the grid and where the assigned numbers will be able to fit into their corresponding hooks.
 
Given a specific arrangement of hooks and an assignment of numbers to those hooks, searching for combinations of nine pentominos on the grid such that the intersection of hooks and pentominos produces a pattern of digit-filled squares that satisfies the puzzle conditions is a much smaller problem space than considering all possible sequences of filled squares within each hook. The puzzle conditions specify 6 of the 9 pentominos directly and limit their possible positions, and other constraints limit the possibilities for the remaining pentominos. My approach was a depth first search across pentomino types, positions and orientations, where at each step the partial arrangement of pentominos can be checked with the intersection of the given hooks to determine the digits on the grid and prune any branches of the search that contradict the puzzle constraints.

Repeating this over each of the 339 potential hook/number combinations finds the puzzle answer relatively quickly, although the search space that must be explored for some combinations is still quite large. The solution speeds this up by parallelizing the tree search across multiple threads for lower branches of the tree.

The unique answer to the puzzle has the hook layout:

    6 6 6 6 6 6 7 8 9 
    5 5 5 5 5 6 7 8 9 
    5 4 4 4 4 6 7 8 9 
    5 4 3 3 3 6 7 8 9 
    5 4 3 2 1 6 7 8 9 
    5 4 3 2 2 6 7 8 9 
    7 7 7 7 7 7 7 8 9 
    8 8 8 8 8 8 8 8 9 
    9 9 9 9 9 9 9 9 9

The assignment of numbers to hooks:

    9x9 <- 9
    8x8 <- 8
    7x7 <- 7
    6x6 <- 5
    5x5 <- 4
    4x4 <- 6
    3x3 <- 3
    2x2 <- 2
    1x1 <- 1

The pentomino layout:

    . I I I I I U . U 
    . L . F . . U U U 
    . L F F F . . X . 
    . L . . F . X X X 
    N L L . T . . X . 
    N . T T T . . . . 
    N N . . T . Z . V 
    . N . . Z Z Z . V 
    . . . . Z . V V V 

The intersecting digit positions:

    . 5 5 5 5 5 7 . 9 
    . 4 . 4 . . 7 8 9 
    . 6 6 6 6 . . 8 . 
    . 6 . . 3 . 7 8 9 
    4 6 3 . 1 . . 8 . 
    4 . 3 2 2 . . . . 
    7 7 . . 7 . 7 . 9 
    . 8 . . 8 8 8 . 9 
    . . . . 9 . 9 9 9

And the sum of digits within each pentomino:

	I: 25
	U: 40
	Z: 40
	V: 45
	N: 30
	X: 40
	F: 25
	L: 25
	T: 15


The product of the areas of empty regions on the grid is 1<sup>4</sup> x 3 x 4 x 9 x 15  = **1620**.
 
 
 
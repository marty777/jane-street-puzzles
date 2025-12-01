# Jane Street November 2025 Puzzle

## Description

The [November 2025 puzzle](https://www.janestreet.com/puzzles/shut-the-box-index/) asks for a rectangular solid box to be constructed by cutting and folding a square grid under specific rules.

An image of a $20 \times 20$ grid is provided, where grid cells are either blank or contain one of several symbols. The puzzle instructions indicate that scissors should be used to cut away connected groups of orthogonally connected cells from the grid, each group having at least one cell on the grid border. The remaining *box cells* after the cut groups are removed must be connected and cannot contain any holes.

Once the box cells have been isolated, it must be possible to fold them along grid lines such that they form a rectangular solid box. 

Cells on the grid occupied by arrow symbols are not part of the box cell group. The arrows, which can point in multiple directions along rows and columns, indicate the direction of the closest box cell or cells.

Cells on the grid occupied by numbers are part of the box cell group. The number indicates how many cells within one king's move, including the numbered cell, are part of the box cell group.

If number on a cell is contained within a *circle*, it will be directly opposite another cell with a circle on the assembled box, on the opposing face.

If a numbered cell is contained within a *square*, it will be adjacent to one or more cells with squares on the same face of the assembled box.

Once the box is assembled, the puzzle answer is the product of the sums of numbers on each face of the box.

## Solution
 
 The solution is implemented in Rust.

### Usage

```console
$ cargo run --release -- [OPTIONS]
```

or 

```console
$ cargo build --release
$ ./target/release/nov2025 [OPTIONS]
```

```console
Options:
  -s, --sample   Solve the sample puzzle
  -v, --verbose  Display solution details
  -h, --help     Print help
```

### Examples

Output the puzzle solution

```console 
    $ cargo build --release
    $ ./target/release/nov2025
```

Output the box cells of the grid, a flattened view of the assembled box, and the solution for the sample puzzle

```console 
    $ cargo build --release
    $ ./target/release/nov2025 --sample --verbose
```

## Discussion

Trying all possible combinations of box cells on the grid and attempting to use them to construct arbitrary boxes is plainly not practical. However, the symbols on the board allow for only a small number of possible box cell layouts, and these can be found quickly. From there, wrappings of the box cells around a limited number of candidate box dimensions can be attempted until one that matches the puzzle requirements can be found.

I didn't submit a correct answer to this puzzle. The layouts of the solved box cells and the resulting box were correct, but I'd missed one number cell when entering the grid into the solver. This has been fixed in the included solution.

### Box cell layouts

A large proportion of the cells on the grid can be directly inferred as being part of the box cell and non-box cell groups based on the arrow and number labels.

- Any number cell is a member of the box cell group
- Any arrow cell is a member of the non-box cell group.
- In each direction pointed to by an arrow cell, there will be some fixed distance $d \ge 1$ where there are $d-1$ non-box cells in the direction followed by $1$ box cell. In the directions not pointed to by the arrow cell, there must be $d$ non-box cells (or else the arrow cell would point in those directions). This has the consequences:
    - The first cell in any direction not pointed to by an arrow cell must be a non-box cell (since $d \ge 1$).
    - Iterating over possible $d$ up to the bounds of the board, if there is only one potential distance that does not violate previous assignments of cells on the board, $d$ is unambigious for the arrow cell and cells up to distance $d$ in each direction can be assigned to the box cell and non-box cell groups.
- If two arrow cells point towards each other, and there is one unassigned cell, no box cells, and any number of non-box cells between them, then the unassigned cell must be a member of the box cell group. The value of $d$ for both arrows can also be inferred from this box cell.
- If any number cell with value $n$ is surrounded by $n$ box cells within a king's move distance (including itself), any unassigned cells within that distance must be non-box cells.
- If any number cell with value $n$ is surrounded by $9 - n$ non-box cells within a king's move distance, any unassigned cells within that distance must be box cells.
- The box cells must be connected, so any regions of unassigned cells surrounded by non-box cells (or the grid border) must be non-box cells.
- There are no holes in the box cells, so any regions of unassigned cells surrounded by box cells and not adjacent to the grid border must be box cells.

By repeatedly applying these rules, 322 out of the 400 cells on the grid can be immediately assigned to the box and non-box groups. 

For the remaining cells, a search for valid arrangements must be conducted. One way to accomplish this is to collect any number cells where the surrounding cells haven't been fully assigned, and any arrow cells where the distance to the closest box cells has not been uniquely determined, and attempt speculative assignments involving these cells to find arrangements that don't violate the rules of the board. After each speculative assignment for a number or arrow cell, any applicable inference rules may be used to assign further cells. A small number of cells will still remain unassigned at the end of this process for each viable arrangment, and all combinations of assignments on these remaining cells can be iterated over. Any arrangements with an odd number of box cells can be discarded, because the rectangular solid must have an even number of face cells. This arrives at 36 distinct potential arrangements of box cells.

### Building the box

Given a full arrangement of box cells, a short list of possible candidate dimensions of boxes that they might be able to form can be narrowed down based on $b$, the number of box cells in the arrangement. Given $b$, the box can only have integer dimensions, measured in face cells, $x,y,z \gt 0$ such that $b = 2xy + 2yz + 2xz$. For a given arrangement of box cells and a candidate box, wrappings of the grid cells around the box can be attempted.

- Pick one box cell on the grid as a *root cell*.
- For each coordinate on the surface of a box with dimensions $x,y,z$, assume the root cell maps to this position on the box.
    - For each other box cell in the arrangement, find a path between it and the root cell on the grid. Move over the surface of the box using the directions of the grid path starting from the assumed root cell position. Record the mapping of the other box cell to the resulting position on the surface of the box.
    - If any two box cells map to the same position on the box surface, the root cell position is incorrect.
    - If a complete mapping of box cells to the box surface is possible, test if the positions of the circle and square numbers on the grid when mapped to the box surface satisfy the puzzle constraints.

If no satisfying mappings are possible across any candidate box dimensions, the box cell arrangement is not correct and the next can be tested. 

Eventually, a solution can be found, with 136 box cells on the grid. 

    . . . . . . # # . . . . . . . . . # # .
    . . . . . . . # # . # # . . . # # # # #
    . . . . . . . # . . . # # . # # . # . .
    . . . . . . . # # . # # # # # . . # # #
    . . . . . . . . # # # . # # . . . # . #
    . . . . . . # # # . . . # # . . . # . .
    . . . . . . # # # # . # # . . . . # # .
    . . # # # # # # # . . # # # . . . # # #
    . # # # . . # # . . . # # # # # . # # .
    . . # # . # # # # . . # . # # . . # . .
    . . . . . # # # # # . # . . . . # # # .
    . . # # . . . # . # . # # . . . # . . .
    . . . # . . . # . . . # # . . . . . . .
    . . . # # . # # # # . # . . . . . . . .
    . . . # # # # # . . . . . . . . . . . .
    . . . . . . # # # . . # # . . . . . . .
    . . . . . . . . # # # # # . . . . . . .
    . . . . . . . . . # . . # # # . . . . .
    . . . . . . . . # # # . . # . . . . . .
    . . . . . . . . . # . . . . . . . . . .

These cells form a $7 \times 6 \times 2$ box. Looking at a flattened view of the box, the circle number positions on each face are:

                +-------------+
                |. . . . . . 5|
                |. . . . . . .|
                |. . . . . . .|
                |. . . . 4 . .|
                |. . . . . . .|
                |. . . . . . .|
    +-----------+-------------+-----------+-------------+
    |. . . . . .|. . . . . . .|. . . . . .|. . . . . . .|
    |. . 7 . . .|. . . . . . .|. . . 5 . .|. . . . . . .|
    +-----------+-------------+-----------+-------------+
                |. . . . . . .|
                |. . . . . . .|
                |. . . . 4 . .|
                |. . . . . . .|
                |. . . . . . .|
                |. . . . . . 4|
                +-------------+

The square number positions on each face are:

                +-------------+
                |. . . . . . .|
                |5 5 4 . . . .|
                |. . . . . . .|
                |. . . . . . .|
                |. . 7 3 . . .|
                |. . . . . . .|
    +-----------+-------------+-----------+-------------+
    |. . . . . .|. . . . . . .|. . . . . .|. . . . . . .|
    |. . . . . .|. . . . 6 5 .|. . . . . .|. . . . . . .|
    +-----------+-------------+-----------+-------------+
                |. . . . . . .|
                |. . . . . . .|
                |. . . . . . .|
                |. . . . . . .|
                |. . . . . . .|
                |. . . . . . .|
                +-------------+

And all number positions on each face are:

                +-------------+
                |. . . . . 4 5|
                |5 5 4 . . . .|
                |. . . . . . 7|
                |. . . . 4 . .|
                |4 . 7 3 . . .|
                |. 9 . . . . .|
    +-----------+-------------+-----------+-------------+
    |. . . . . .|. . . . . . .|. . 7 . . 5|. . . . . . .|
    |. . 7 . 4 .|. . . . 6 5 .|. . . 5 . .|. . . . . . 5|
    +-----------+-------------+-----------+-------------+
                |. . . . . . .|
                |. 7 . . . . 5|
                |. . . . 4 . .|
                |. . . . . . .|
                |6 . . 2 . . .|
                |. . . . . . 4|
                +-------------+

The product of the sums of numbers on each face of the box is $57 \times 11 \times 11 \times 5 \times 17 \times 28 = 16414860$.

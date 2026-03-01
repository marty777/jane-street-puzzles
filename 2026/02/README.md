# Jane Street February 2026 Puzzle

## Description

The [February 2026 puzzle](https://www.janestreet.com/puzzles/subtiles-2-index/) presents a grid of cells, some containing formulas, and asks for assignments of regions of incrementing integers.

An image of a $13 \times 13$ grid is provided, where grid cells are either blank or contain a formula using the variables $a,b,c$. The puzzle asks that positive integers be assigned to the cells of the grid such that:

1. There must be one $1$ cell, two $2$ cells, etc. up to some maximum $N$.
2. The formulas, when the variables are solved for, indicate integer cells that must be included in the assignment. 
3. The cells assigned with each integer K must form an orthogonally connected shape (a K-omino). Starting from the 2-omino, each K-omino must contain (up to rotation and reflection) the shape for the (K-1)-omino integer cells.

Once a valid assignment is found, the puzzle answer is the product of the minimum and maximum row sums of the integers on the grid.

## Solution
 
 The solution is implemented in Rust.

 ### Usage

```console
$ cargo run --release -- [OPTIONS]
```

or 

```console
$ cargo build --release
$ ./target/release/feb2026 [OPTIONS]
```

```console
Options:
  -v, --verbose  Display solution details
```

### Examples

Calculate the puzzle solution

```console 
$ cargo build --release
$ ./target/release/feb2026 --verbose
```

## Discussion

### Solving the formulas

- The grid is $13 \times 13$, meaning at most $169$ integers can be assigned to cells. $171$ cells would need to be assigned integers if $N = 18$. The maximum possible $N$ for the grid must therefore be $17$, which would require $153$ cells. 
- Each formula must evaluate to an integer in $[1,17]$. Several formulas indicate the possible values of $b$. The formulas $8 - b$ and $11 - b$ imply $b$ is an integer. The formulas $b^2$ and $(b-1)^2$ must mean that $|b| \le 4$, $b \not = 0$, $b \not = 1$ and $b \not = -4$. This gives possible values $[-3, -2, -1, 2, 3, 4]$.
- For each possible value of $b$, the equation $c^b = x$ for some $x \in [1,17]$ can be used to solve for possible $c$ values corresponding to each possible $b$.
- For each possible $b$ and $c$, the equation $\log_{c}a = x$ for some $x \in [1,17]$ can solve for possible $a$ values. 
- For each possible $a,b,c$, each formula on the grid can be evaluated.

There is only one combination of possible $a,b,c$ where each formula evaluates to an integer in $[1,17]$: $a = \frac{1}{4}, b = -3, c = \frac{1}{2}$

This gives the following grid after evaluating each formula, which has at least one value for each integer in $[1,16]$:

    .  .  .  .  15 .  .  .  .  .  .  .  .  
    .  .  .  .  .  .  .  11 .  .  .  .  .  
    .  15 .  5  .  .  15 .  11 .  11 .  .  
    .  .  .  .  15 .  .  8  .  12 .  12 .  
    .  16 .  .  .  8  .  .  .  .  6  .  .  
    .  .  .  16 .  .  .  .  .  .  .  .  6  
    .  .  16 .  3  .  16 .  1  .  12 .  .  
    13 .  .  .  .  .  .  .  .  4  .  .  .  
    .  .  7  .  .  .  .  12 .  .  .  10 .  
    .  2  .  13 .  16 .  .  14 .  .  .  .  
    .  .  13 .  14 .  14 .  .  14 .  10 .  
    .  .  .  .  .  9  .  .  .  .  .  .  .  
    .  .  .  .  .  .  .  .  9  .  .  .  .

### K-omino positions

Building all possible shapes up to 16 or 17 cells, and attempting arrangements of combinations of these shapes would not be practical. However, not all K-omino shapes would be individually viable on the grid. Each shape must have a position/rotation/reflection that allows it to cover all the formula cells on the grid corresponding to its size, not cover any formula cells for other sizes, and not fall outside the bounds of the grid[^1]. It also must have a viable "parent" (K-1)-omino. Starting from the 1-omino, it's possible to build an index of viable shapes descended from viable shapes of the preceding size and record the inheritance relationships for each.

There is a notable bottleneck of only two viable 12-ominos when the shape index is constructed in this way. Restricting candidate K-ominos to predecessors and descendants of these shapes in a search for potential assignments of shapes to the grid significantly limits the search space. A tree search of arrangements starting from the limited number of potential 12-omino placements, and continuing with all possible placements of related shapes in each size is able to find a unique solution relatively quickly.

The solved assignment of 16 K-ominos to the formula grid is:

    .  5  5  5  15 15 .  11 .  .  .  .  .
    .  .  .  5  .  15 .  11 .  .  11 11 11
    15 15 15 5  .  15 15 11 11 11 11 .  11
    15 16 15 15 15 15 8  8  8  12 12 12 11
    15 16 .  .  8  8  8  .  8  12 6  6  6
    15 16 .  16 16 16 16 .  8  12 12 .  6
    .  16 16 16 3  3  16 16 1  4  12 6  6
    13 13 13 13 14 3  16 4  4  4  12 10 10
    7  7  7  13 14 16 16 12 12 12 12 10 .
    7  2  2  13 14 16 14 14 14 14 .  10 .
    7  7  13 13 14 14 14 .  9  14 14 10 10
    .  7  13 9  9  9  9  .  9  14 .  .  10
    .  .  13 13 13 13 9  9  9  14 10 10 10

With no remaining space for a 17-omino, the solution is complete. The minimum and maximum row sums are **56** and **162**, with the product **9072**

[^1]: Another filtering criteria for viable shapes is that any placement position/rotation/reflection on the grid must allow paths between pairs of formula cells of other sizes, up to a path length of the other size. I added testing for this initially, and it slightly improves the overall speed of the solution, but elected to remove it for simplicity.
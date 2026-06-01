# Jane Street May 2026 Puzzle

## Description

The [May 2026 puzzle](https://www.janestreet.com/puzzles/arch-madness-index/) presents a grid of square cells. Cells are colored either green or white and some cells contain numbers.

The puzzle indicates that some set of 90-degree arcs (quarter-circles with a radius equal to a cell side, connecting one corner of the cell to the opposite corner) should be placed on the grid, in white cells only, to divide the grid into regions. All of the resulting regions must have integer areas. 

Each region is given an integer score which is the product of its area and the number of "smooth" segments of its border, including any parts of the border which are on the horizontal and vertical edges of the grid. After the score for the region has been determined, the grid is marked with that number in all cells which are mostly inside the region (i.e. cells without arcs, or cells where the region is in the "inside" of the arc).

The puzzle asks for an assignment of arcs to the grid so that each cell initially containing a number is marked with a region score equal to that number. Once a valid assignment is found, the puzzle answer, using the marked scores of each cell on the grid, is the sum of the squares of the row sums plus the sum of the squares of the column sums.

## Solution
 
 The solution is implemented in Rust.

 ### Usage

```console
$ cargo run --release -- [OPTIONS]
```

or 

```console
$ cargo build --release
$ ./target/release/may2026 [OPTIONS]
```

```console
Options:
  -s, --sample  Solve the sample grid
  -h, --help    Print help
```

## Discussion

If a region is bordered by an arc, only a portion of the cell containing the arc's area is included in the region. The proportion is determined by whether the region is on the "inside" or "outside" of the circular arc, with areas $\frac{\pi}{4}$ or $1 - \frac{\pi}{4}$ respectively. Since regions must have integer areas, the border of a region which includes arcs must have an equal number of inside and outside arcs.

For an arc to form part of a region border, it must have adjoining arcs or grid borders at the relevant corners of the arc. A border is "smooth" between two adjoining arcs or an arc and the grid border if the line segments are both horizontal or both vertical at the joining corner. Setting up a framework to interate over possible valid borders to produce regions and correctly score those regions is one of the main challenges in this puzzle, the other being finding an efficient way to search for an arrangement of arcs that satisfies the requirements.

For an arbitrary set of arcs applied to the grid, my process for finding regions was to list all whole cells and arc cells on the grid, with arc cells listed twice: once as interior and once as exterior. Regions were produced by picking one unassigned cell from the list and flood filling to find all cells within the region. If an arc was found to have both its interior and exterior sides included in the same region, this would indicate that the arc assignments did not produce valid regions. Once all cells that were part of a region were found, the cells scored by the region could be listed, the area could be calculated and the arcs and grid border segments that bordered the region could be iterated over to count the smooth sections and produce the score. The contributing whole cells and arcs could then be removed from the unassigned list and the process repeated until no unassigned cells remained.

With a method to score any set of regions produced by a set of arcs established, it was then possible to search for sets of arcs that would satisfy the numbered cells on the grid. My approach was equivalent to a breadth-first search across possible divisions of existing regions, starting from the empty grid containing no arcs (forming a single candidate region) :

1. Of all current regions in a given candidate assignment, find the largest one that contains any numbered cells that do not match the region's score.
2. Search across all valid connected "strings" of arcs (up to a limited number of arcs) that could be drawn between two points on the region's border, dividing the region into two new regions. Any new arc assignments where at least one of the new regions would have all numbered cells within it scored correctly would be a candidate arc assignment for further subdivisions.
3. Continue across all candidates until either none remain or a full solution is found.

This process would not have worked if there were any "floating" regions within puzzle solution, with a border unconnected to other region borders, or if there were regions in the solution that did not contain a numbered cell. Fortunately this was not a problem.

A search for valid strings of arcs that subdivide large regions produces a great number of candidates if a large maximum number of steps is permitted. For this reason, my eventual solution tries to establish borders for regions in a specific order. Candidate borders for smaller regions that neighbor larger regions are established first using small search limits. Larger regions neighboring the small regions could then be searched for more easily because portions of their candidate borders were already established, and subsequent searches for subdividing borders have smaller search spaces. The order and grouping of cells was determined by experimentation, and arrives at an answer relatively quickly. This approach greatly reduces the overall search space, but as a result the optimised solver isn't generally applicable for other instances of this sort of puzzle.

The layout of arcs satisfying the puzzle requirements (crudely rendered) is:

    ◼ ◻ ◝ ◼ ◝ ◼ ◞ ◼ ◜
    ◻ ◻ ◞ ◼ ◞ ◜ ◼ ◜ ◼
    ◼ ◜ ◻ ◜ ◞ ◟ ◞ ◼ ◜
    ◜ ◟ ◻ ◟ ◝ ◻ ◝ ◞ ◼
    ◼ ◻ ◝ ◞ ◼ ◟ ◻ ◝ ◞
    ◜ ◟ ◜ ◜ ◟ ◻ ◝ ◼ ◼
    ◜ ◝ ◜ ◻ ◜ ◜ ◝ ◟ ◞
    ◼ ◜ ◻ ◜ ◜ ◝ ◻ ◟ ◞
    ◜ ◼ ◜ ◞ ◼ ◜ ◼ ◻ ◼

Which assigns scores to cells:

    21   21   21   27   27   288  288  15   25
    21   21   21   27   27   15   15   25   25
    21   27   27   288  288  15   15   25   9
    25   27   27   288  288  45   45   25   9
    25   25   25   27   288  45   45   45   9
    288  25   288  63   288  288  288  45   45
    9    9    63   63   288  35   35   45   45
    9    63   63   288  9    9    35   288  288
    63   63   288  288  9    35   35   35   35

The puzzle solution, the sum of the squares of each column and row sum, is **13682882**.

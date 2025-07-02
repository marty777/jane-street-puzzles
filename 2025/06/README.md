# Jane Street June 2025 Puzzle

## Description

The [June 2025 puzzle](https://www.janestreet.com/puzzles/some-ones-somewhere-index/) presents nine distinct photographs, laid out in a $3 \times 3$ grid, of a square board partly covered with arrangements of translucent colored squares. Scrabble tiles are visible in several images. The puzzle description indicates that the answer is a sentence.

## Solution

The solution is implemented in Python 

### Usage

```console
$ python jun2025.py
```

The script will solve each board and print the letters derived from those solutions, as well as the completed sentence that answers the puzzle.

## Discussion

### Partridge Puzzles

I was stuck on this until a helpful (and since deleted) [r/csMajors](https://www.reddit.com/r/csMajors/) thread pointed out that the Scrabble tiles arranged along the top and left edges of the board in several photos spell out "PARTRIDGE TILING". I'd noticed the tiles, but completely failed to put that together.

This leads to the [Partridge Puzzle](https://pyrigan.com/2017/02/17/the-partridge-puzzle/) invented by Robert T. Wainwright, where in the original formulation square tiles of varying integer sizes are arranged to form a completed larger square. The puzzle name comes from "The Twelve Days of Christmas" due to the number of tiles included of each size. For a puzzle of order $N$, there is $1 \times (1 \times 1)$ tile, $2 \times (2 \times 2)$ tiles etc. up to $N \times (N \times N)$ tiles. The total area of all the tiles is a sum of cubes which has a well known [identity](https://en.wikipedia.org/wiki/Squared_triangular_number):

$$\sum_{k=1}^{N} k^3 = \left(\sum_{k=1}^{N} k\right)^2 = \left(\frac{N(N+1)}{2}\right)^2$$

The total area is a square integer, suggesting that it might be possible to arrange the tiles to perfectly fill a square region. The challenge is finding such an arrangement, and for several $N$ no arrangements are possible. [OEIS A381976](https://oeis.org/A381976) gives the number of distinct possible arrangements of tiles on Partridge Puzzle boards up to $N=9$.

Examining the photos of the puzzle, we can see that there are 8 distinct sizes of tile visible. However, since the largest type appears on the board 9 times in several photos and the board measures 5 of the largest tiles to a side, this must be a Partridge Puzzle for $N = 9$ with the smallest tile not shown. Note that the total area of the board for $N=9$ is $45^2 = 2025$, fitting for a puzzle this year.

### Solving the boards

With 9 distinct partial Partridge Puzzle tiling arrangements presented, it seems reasonable to attempt to complete them. Solving these is straightforward, although entering the partial arrangements into a format that can be used by a solver is time-consuming. Once the information has been input for each board, a tree search (I used a breadth-first search) of possible placements of the remaining tiles reaches solutions quickly. For each board, there is exactly one possible complete arrangement.

Since the 1-tile is conspicuously absent in the images, and since the title of the puzzle is "Some Ones, Somewhere", it makes sense that the position of the 1-tile on each board is important for the puzzle solution. If we treat each board as a $45 \times 45$ grid with the upper-left cell as position $(0,0)$, the 1-tiles for each solved board are at the following $(x,y)$ coordinates:

$$(20, 18), (21, 12), (16, 31)$$
$$(27, 27), (25, 11), (32, 15)$$
$$(16,  6), (33,  8), (18,  5)$$

### Solving the puzzle

Knowing that the puzzle solution is a sentence, it's plausible that these coordinates might be used to indicate letters or words somehow. The Scrabble tiles on the edges of some boards spelling "PARTRIDGE TILING" appear to be deliberately and oddly placed, offering a clue. If the positions of each Scrabble tile are mapped onto a larger $135 \times 135$ grid based on the $3 \times 3$ position of the board they appear on, each tile appears at a horizontal or vertical position $x$ such that $x \pmod{26}$ corresponds perfectly to the index of the tile's letter in the alphabet if A is taken as index $0$.

For example, the "G" in "PARTRIDGE" appears at horizontal position $20$ on the top-right board. Mapping that position to a $135 \times 135$ grid gives it a horizontal position of $45 + 45 +20 = 110$. The "G" in "TILING" appears at a vertical position of $39$ on the leftmost board in the middle row. Mapping that position to a $135 \times 135$ grid gives it a vertical position of $45 + 39 = 84$.

$$ 110 \pmod{26} \equiv 6 = \text{G} $$
$$ 84 \pmod{26} \equiv 6 = \text{G} $$

Exactly how to apply this letter mapping to the coordinates of the 1-tiles isn't immediately clear, but with a bit of experimentation the $(x,y)$ coordinates of each 1-piece, mapped to $(x', y')$ in the coordinate system of the larger $135 \times 135$ grid formed by the 9 boards, can be ordered as $(y' \pmod{26}, x' \pmod{26})$ and then mapped to the alphabet. This yields:

$$(\text{S},\text{U}), (\text{M},\text{O}), (\text{F},\text{C})$$
$$(\text{U},\text{B}), (\text{E},\text{S}), (\text{I},\text{S})$$
$$(\text{S},\text{Q}), (\text{U},\text{A}), (\text{R},\text{E})$$

Taking this together with the Scrabble tiles spelling "THE" and "A" to the side of the board in two images, we get the completed sentence "THE SUM OF CUBES IS A SQUARE", referring to the formula above for the total tile area of a Partridge Puzzle.


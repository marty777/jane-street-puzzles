# Jane Street August 2026 Puzzle

## Description

In the [August 2026 puzzle](https://www.janestreet.com/puzzles/andys-afternoon-amble-index/), Andy the ant lives on a simple sphere with four white hexagonal regions and four black triangular regions, where each hexagon is neighbors with three other hexagons and three triangles alternating around the border.

Andy percieves his sphere as a plane, and will only move between the white hexagons. Every afternoon, Andy marks his home hexagon and goes on a walk. At every step of his walk, he will move to one of the neighboring white hexagons from his current one with equal probability, and the walk ends when Andy returns to his home hexagon. Andy cannot distinguish between the edges of his home hexagon's borders, so he doesn't know if he returned from the same edge he started from.

One day, Andy unwittingly falls off his sphere onto an infinite kitchen floor tiled with black and white hexagons, where each white hexagon is neighbors with three white hexagons and three black hexagons alternating around the border. Andy lands on a white hexagon, marks it as his home, and goes for his usual walk.

Andy remembers the turns he takes on his walk. The puzzle asks for the probability $p$, in exact terms, that Andy realizes he's not on his sphere over the course of his walk.

## Solution

### Initial observations

There are conceptual simplifications that can be made to Andy's environments, since he doesn't enter the black cells. The sphere can be treated as a tetrahedron, and the kitchen floor plane can be treated as a triangular grid. Navigation models in these schemes that take into account Andy's turns are relatively straightforward to construct.

The only way Andy will realize that he has left his sphere is if:

 - He encounters his home cell when he wouldn't expect to on the sphere during his walk on the plane.
 - He doesn't encounter his home cell when he would expect to on the sphere during his walk on the plane.
 
 Experimenting with simulations of Andy's possible walks, turn-by-turn, it becomes evident that there is no way for Andy to encounter his home cell on the plane when he wouldn't expect to reach it on the sphere. Only the probability that a walk on the plane doesn't return to the home cell when it would on the sphere needs to be considered.

### Probabilities by walk length

Taking the sphere as a tetrahedron, one can imagine one side as the home cell. Each non-home cell is neighbors with the home cell and the other two non-home cells, and Andy chooses the next cell in his walk with equal probability. Andy's walk leaves the home cell for some non-home cell with probability $\frac{3}{3}$ on the first step, moves between the non-home cells for zero or more steps with probability $\frac{2}{3}$ at each step, and then returns home with probability $\frac{1}{3}$ on the final step. The probability $l_n$ that Andy's walk on the sphere has length $n$ is:

$$l_n = \dfrac{1}{3} \cdot \left(\dfrac{2}{3}\right)^{n-2}$$
 
Given $w_n$, the total number of distinct possible walks of length $n$ on the sphere, and $s_n$, the number of distinct possible walks of length $n$ on the sphere that don't return to the home cell on the plane, the probability $p_n$ that Andy notices he isn't on the sphere on a walk of length $n$ is:

$$p_n = \dfrac{s_n}{w_n}$$

And the overall probability $p$ that Andy notices he isn't on the sphere on a walk of any length is:

$$p = \sum_{n}^{\infty} p_n \cdot l_n$$

 Through exhaustive enumerations of Andy's walks (up to rotational symmetry) for small lengths $n$ the following pattern can be observed:

| $n$ | $s_n$  | $w_n$  | $\frac{s_n}{w_n}$|
|-----|--------|--------|------------------|
| 2   | 0      | 1      | 0                |
| 3   | 2      | 2      | 1                |
| 4   | 2      | 4      | 0.5              |
| 5   | 8      | 8      | 1                |
| 6   | 10     | 16     | 0.625            |
| 7   | 32     | 32     | 1                |
| 8   | 42     | 64     | 0.65625          |
| 9   | 128    | 128    | 1                |
| 10  | 170    | 256    | 0.6640625        |
| 11  | 512    | 512    | 1                |
| 12  | 682    | 1024   | 0.666015625      |
| 13  | 2048   | 2048   | 1                |
| 14  | 2730   | 4096   | 0.666503906      |
| 15  | 8192   | 8192   | 1                |
| 16  | 10922  | 16384  | 0.666625977      |
| 17  | 32768  | 32768  | 1                |
| 18  | 43690  | 65536  | 0.666656494      |
| 19  | 131072 | 131072 | 1                |
| 20  | 174762 | 262144 | 0.666664124      |

Paths of length one cannot return home and all walks of length two return to the home cell in both the plane and the sphere, so $p_n = 0$ for $n < 3$. Paths of odd length on the plane cannot return home, while paths of any length on the sphere can return home, so $p_n = 1$ for any walk of odd length $n \ge 3$. 

For walks of even length $n \ge 3$, the number of paths $s_n$ that return to the home cell on the sphere but not the plane (up to rotational symmetry) is the sum of odd powers of two, which has a convenient closed form:

$$s_n = \sum_{i=1}^{n/2} 2^{2i - 1} = \dfrac{2}{3} \cdot (4^{(n/2-1)} - 1)$$

And the total number of paths $w_n$ that return to the home cell on the sphere up to rotational symmetry (which is equal to $s_n$ for walks of odd length) is:

$$w_n = 2^{n - 2}$$

For $n \ge 3$, the probability $p_n$ of Andy realizing he isn't on the sphere if the walk on the sphere that would be expected to return home has length $n$ is therefore:

$$p_n = \begin{cases}
     \dfrac{2}{3} \cdot \dfrac{4^{(n/2-1)} - 1}{2^{n - 2}}  & \text{if } n \text{ even} \\
     1 & \text {if } n \text{ odd}
     \end{cases}$$

### Calculating $p$

The sum of probabilities that Andy realizes he isn't on the sphere for walks of any length $n \ge 3$ can be split into two series, one for the even walk lengths and one for the odd walk lengths. The series for the walks of even length is:

$$p_\text{even} = \sum_{i=2}^{\infty} \dfrac{2}{3} \cdot \dfrac{4^{(i-1)} - 1}{2^{2i - 2}} \cdot \dfrac{1}{3} \cdot \left(\dfrac{2}{3}\right)^{2i - 2} $$

And the series for the walks of odd length is:

$$p_\text{odd} = \sum_{i=1}^{\infty} \dfrac{1}{3} \cdot \left(\dfrac{2}{3}\right)^{2i - 1}$$

These series are convergent, to values $\frac{3}{20}$ and $\frac{2}{5}$ respectively. The probability that Andy realizes he is not on his sphere is:

$$p = p_\text{even} + p_\text{odd} = \dfrac{3}{20} + \dfrac{2}{5} = \dfrac{11}{20}$$

use crate::coord::Coord;

/// Enum representing cardinal directions on the grid.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Dir {
    N,
    E,
    S,
    W
}
impl Dir {
    /// Return a `Vec` of all `Dir`s.
    pub fn dirs() -> Vec<Dir> {vec![Dir::N, Dir::E, Dir::S, Dir::W]}
    /// Returns a `Coord` of a vector 1 cell in the given direction.
    pub fn coord(&self) -> Coord {
        match self {
            Dir::N => Coord::new(0,-1),
            Dir::E => Coord::new(1,0),
            Dir::S => Coord::new(0,1),
            Dir::W => Coord::new(-1,0)
        }
    }
    /// Returns true if `dir1` and `dir2` are opposite directions.
    pub fn are_opposite(dir1:&Dir, dir2:&Dir) -> bool {
        match dir1 {
            Dir::N => *dir2 == Dir::S,
            Dir::E => *dir2 == Dir::W,
            Dir::S => *dir2 == Dir::N,
            Dir::W => *dir2 == Dir::E
        }
    }
}
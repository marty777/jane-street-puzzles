
/// 2D integer coordinate struct
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    pub x:isize,
    pub y:isize
}
impl Coord {
    /// Construct a new `Coord` with the given `x` and `y` values.
    pub fn new(x:isize, y:isize) -> Coord {
        Coord{x:x, y:y}
    }
    /// Construct a new `Coord` with the given `x` and `y` values as `usize`s.
    pub fn newu(x:usize, y:usize) -> Coord {
        Coord{x:x as isize, y:y as isize}
    }
    /// Add the given `x` and `y` values to the `Coord`.
    pub fn add(&self, x:isize, y:isize) -> Coord {
        *self + Coord::new(x,y)
    }
    /// Return true if the coordinate is within the bounds of a zero-based 
    /// rectangle with dimensions `rect_dim`.
    pub fn in_bounds(self, rect_dim:&Coord) -> bool {
        self.x >= 0 && self.x < rect_dim.x && self.y >= 0 && self.y < rect_dim.y
    }
}
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{})", self.x, self.y)
    }
}
impl std::ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs:Coord) -> Coord {
        return Coord{x:self.x + rhs.x, y:self.y + rhs.y};
    }
}
impl std::ops::Mul<isize> for Coord {
    type Output = Coord;
    fn mul(self, rhs:isize) -> Coord {
        return Coord{x:self.x * rhs, y:self.y * rhs};
    }
}
/// 2D Vector struct
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Coord {
    pub x:isize,
    pub y:isize
}
impl Coord {
    pub fn new(x:isize, y:isize) -> Coord {
        return Coord{x:x, y:y};
    }
    pub fn newu(x:usize, y:usize) -> Coord {
        return Coord{x:x as isize, y:y as isize};
    }
    /// Tests if the coordinate is within the bounds of a zero-based rectangle 
    /// with dimensions `dim_x`, `dim_y`.
    pub fn in_bounds(self, dim_x:usize, dim_y:usize) -> bool {
        return self.x >= 0 && self.x < dim_x as isize && self.y >= 0 && self.y < dim_y as isize;
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
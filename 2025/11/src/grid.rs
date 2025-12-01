use std::collections::{HashMap,HashSet};

// For the arrow grids below, directions are indicated in base 16, with the 
// pointed-to directions OR'd. 1 = N, 2 = E, 4 = S, 8 = W
pub const SAMPLE_GRID_NUMBERS:&str = r#"
. . . . . . . .
. . . 5 . . . .
2 5 . . . 4 . .
. . . . . . . 2
2 . . 5 . . . .
. . . . . . . .
. . . 6 6 . . .
. 4 . . . 4 . ."#;
pub const SAMPLE_GRID_ARROWS:&str = r#"
. . 4 . . . 4 .
. . . . c . . 4
. . . . . . . .
. . . . b . . .
. . . . . . . .
. . 7 . . . 4 1
4 . . . . . . .
. . . . . . . ."#;
pub const SAMPLE_GRID_SQUARES: [Vec2;2] = [Vec2{x:1,y:2},Vec2{x:7,y:3}];
pub const SAMPLE_GRID_CIRCLES: [Vec2;4] = [Vec2{x:3,y:4},Vec2{x:4,y:6},Vec2{x:1,y:7},Vec2{x:5,y:7}];

pub const MAIN_GRID_NUMBERS:&str = r#"
. . . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . 4 . . . 4 . . . .
. . . . . . . 5 . . . . . . . . . . . .
. . . . . . . . . . . . 7 . 5 . . . . .
. . . . . . . . . . 4 . . 7 . . . 4 . .
. . . . . . 4 . 7 . . . . . . . . . . .
. . . . . . . 9 . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . 6 . .
. . 7 . . . . . . . . 5 . . . . . . . .
. . . . . . . . . . . . . . 5 . . . . .
. . . . . 4 . 7 . . . . . . . . . . 3 .
. . . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . . .
. . . 5 . . 6 . . 2 . . . . . . . . . .
. . . . . . . . . . . . . . . . . . . .
. . . . . . 5 . . . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . . .
. . . . . . . . . . . . 5 5 . . . . . .
. . . . . . . . 4 . . . . . . . . . . .
. . . . . . . . . . . . . . . . . . . ."#;
pub const MAIN_GRID_ARROWS:&str = r#"
. 2 . . . . . . c . . . 4 . . 4 . . . c
. . . . 2 . . . . . . . . e . . . . . .
. . . . . . 2 . . e . . . . . . . . d .
. 4 . 6 . . . . . . . . . . . . . . . .
2 . . . . 6 . . . . . . . . . b . . . .
. . . . . . . . . . . 6 . . . . . . . .
. . . 4 . . . . . . a . . d . . 2 . . c
. 6 . . . . . . . . . . . . c . . . . .
. . . . . 7 . . . f . . . . . . . . . .
. 3 . . . . . . . . . . b . . . 6 . d .
. 1 . 5 . . . . . . . . . . . . . . . .
. . . . . . . . b . a . . . b . . 9 . .
. . 3 . . f . . . . . . . . . . . . 1 .
2 . . . . . . . . . . . 9 . . . 1 . . .
. . . . . . . . d . . 5 . 4 . . . . . .
. . 3 . 1 . . . . c . . . . c . . . . 8
. . . . . 1 . 3 . . . . . . . . 8 . 9 .
. 2 . . . . . . . . d . . . . . . . . .
. . . . 3 . 2 . . . . . . . 9 . . . . .
2 . . . 3 . . 2 . . . 8 . . . . . . 9 ."#;
pub const MAIN_GRID_SQUARES: [Vec2;7] = [Vec2{x:7,y:2}, Vec2{x:8,y:5}, Vec2{x:17,y:7}, Vec2{x:11,y:8}, Vec2{x:18,y:10}, Vec2{x:3,y:13}, Vec2{x:8,y:18}];
pub const MAIN_GRID_CIRCLES: [Vec2;6] = [Vec2{x:15,y:1}, Vec2{x:14,y:3}, Vec2{x:10,y:4}, Vec2{x:17,y:4}, Vec2{x:2,y:8}, Vec2{x:12,y:17}];

/// Characters to crudely display multi-direction arrows in terminal output.
pub const ARROW_STRINGS:[&str;16] = ["","↑","→","╚","↓","↕","╔","╠", "←","╝","↔","╩","╗","╣","╦","╬"];

pub const NORTH:usize = 1;
pub const EAST:usize = 2;
pub const SOUTH:usize = 4;
pub const WEST:usize = 8;
pub const ARROW_DIRS:[usize;4] = [NORTH, EAST, SOUTH, WEST];
/// Mapping of ARROW_DIRS to directional vectors
pub const ARROW_DELTAS:[Vec2;9] = [Vec2{x:0,y:0},Vec2{x:0,y:-1},Vec2{x:1,y:0},Vec2{x:0,y:0},Vec2{x:0,y:1},Vec2{x:0,y:0},Vec2{x:0,y:0},Vec2{x:0,y:0},Vec2{x:-1,y:0}];
/// Mapping of bit indexes to relative positions about a number cell
pub const NUMBER_DELTAS:[Vec2;9] = [Vec2{x:-1, y:-1},Vec2{x:0, y:-1},Vec2{x:1, y:-1},Vec2{x:-1, y:0},Vec2{x:0, y:0},Vec2{x:1, y:0},Vec2{x:-1, y:1},Vec2{x:0, y:1},Vec2{x:1, y:1}];

/// The 6 faces of a box
/// ```
///   |A|
/// |C|B|E|D|
///   |F|
/// ```
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BoxFaceType {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5
}
/// Array of `BoxFaceType` enums for iteration
pub const BOXFACETYPES:[BoxFaceType;6] = [BoxFaceType::A,BoxFaceType::B,BoxFaceType::C,BoxFaceType::D,BoxFaceType::E,BoxFaceType::F];

/// Axes of rotation for rotating vectors in 3d.
#[derive(Copy, Clone)]
pub enum RotationAxis {
    AboutX,
    AboutY,
    AboutZ
}
/// Directions of rotation for rotating vectors in 3d.
#[derive(Copy, Clone)]
pub enum RotationDirection {
    Positive,
    Negative
}
/// 2D vector for representing grid cell positions and directions.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vec2 {
    pub x:isize,
    pub y:isize
}
impl Vec2 {
    pub fn new(x:isize, y:isize) -> Vec2 {
        return Vec2{x:x, y:y};
    }
    pub fn newu(x:usize, y:usize) -> Vec2 {
        return Vec2{x:x as isize, y:y as isize};
    }
    /// Returns true if the coordinate is within a grid of the given `dim`.
    pub fn in_bounds(self, dim:usize) -> bool {
        return self.x >= 0 && self.x < dim as isize && self.y >= 0 && self.y < dim as isize;
    }
}
impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{})", self.x, self.y)
    }
}
impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs:Vec2) -> Vec2 {
        return Vec2{x:self.x + rhs.x, y:self.y + rhs.y};
    }
}
impl std::ops::Mul<isize> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs:isize) -> Vec2 {
        return Vec2{x:self.x * (rhs as isize), y:self.y * (rhs as isize)};
    }
}

/// 3D vector for representing box cell positions and directions.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Vec3 {
    pub x:isize,
    pub y:isize,
    pub z:isize,
}
impl Vec3 {
    pub fn new(x:isize, y:isize, z:isize) -> Vec3 {
        return Vec3{x:x, y:y, z:z};
    }
    pub fn newu(x:usize, y:usize, z:usize) -> Vec3 {
        return Vec3{x:x as isize, y:y as isize, z:z as isize};
    }
    /// Returns true if the coordinate is within a box of the given dimensions.
    pub fn in_bounds(&self, dim_x:usize, dim_y:usize, dim_z:usize) -> bool {
        return self.x >= 0 && self.x < dim_x as isize && self.y >= 0 && self.y < dim_y as isize && self.z >= 0 && self.z < dim_z as isize;
    }
    /// Return a rotation of `self` about the given `axis` and in the given `direction`
    pub fn rot(&self, axis:RotationAxis, direction:RotationDirection) -> Vec3 {
        match axis {
            RotationAxis::AboutX => {
                return self.rot_x(direction);
            },
            RotationAxis::AboutY => {
                return self.rot_y(direction);
            },
            RotationAxis::AboutZ => {
                return self.rot_z(direction);
            },
        }
    }
    /// Return the rotation of `self` by 90 degrees positive or negative about 
    /// the x axis
    fn rot_x(&self, rotation:RotationDirection) -> Vec3 {
        // Rx = 1 0   0
        //      0 cos -sin
        //      0 sin cos
        let sin = match rotation {
            RotationDirection::Positive => 1,
            RotationDirection::Negative => -1
        };
        let cos = 0;
        return Vec3{    x:1*self.x + 0*self.y + 0*self.z, 
                        y:0*self.x + cos*self.y + -sin*self.z, 
                        z:0*self.x + sin*self.y + cos*self.z
                    };
    }
    /// Return the rotation of `self` by 90 degrees positive or negative about 
    /// the y axis
    fn rot_y(&self, rotation:RotationDirection) -> Vec3 {
        // Ry = cos  0 sin
        //      0    1 0
        //      -sin 0 cos
        let sin = match rotation {
            RotationDirection::Positive => 1,
            RotationDirection::Negative => -1
        };
        let cos = 0;
        return Vec3{    x:cos*self.x + 0*self.y + sin*self.z, 
                        y:0*self.x + 1*self.y + 0*self.z, 
                        z:-sin*self.x + 0*self.y + cos*self.z
                    };
    }
    /// Return the rotation of `self` by 90 degrees positive or negative about 
    /// the z axis
    fn rot_z(&self, rotation:RotationDirection) -> Vec3 {
        // Rz = cos -sin 0
        //      sin cos  0
        //      0   0    1
        let sin = match rotation {
            RotationDirection::Positive => 1,
            RotationDirection::Negative => -1
        };
        let cos = 0;
        return Vec3{    x:cos*self.x + -sin*self.y + 0*self.z, 
                        y:sin*self.x + cos*self.y + 0*self.z, 
                        z:0*self.x + 0*self.y + 1*self.z
                    };
    }
    /// Determine the axis and direction of a single rotation by 90 degrees 
    /// about one primary axis such that `self` would match `dest` if possible.
    pub fn rotation_to(&self, dest:Vec3) -> Option<(RotationAxis, RotationDirection)> {
        // Could be done by inverting the rotation matrixes for x,y,z, but this works
        for rotation_axis in vec![RotationAxis::AboutX, RotationAxis::AboutY, RotationAxis::AboutZ] {
            for rotation_direction in vec![RotationDirection::Positive, RotationDirection::Negative] {
                if self.rot(rotation_axis, rotation_direction) == dest {
                    return Some((rotation_axis, rotation_direction));
                }
            }
        }
        return None;
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{},{})", self.x, self.y, self.z)
    }
}
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs:Vec3) -> Vec3 {
        return Vec3{x:self.x + rhs.x, y:self.y + rhs.y, z:self.z + rhs.z};
    }
}
impl std::ops::Mul<isize> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs:isize) -> Vec3 {
        return Vec3{x:self.x * rhs, y:self.y * rhs, z:self.z * rhs};
    }
}

/// Structure for storing arrow cell information on the grid.
#[derive(Copy, Clone, Hash)]
pub struct BoxArrow {
    pub pos:Vec2,
    pub val:usize,
    pub dist:usize,
    pub solved:bool
}
impl BoxArrow {
    /// Returns true if the arrow points in the given direction.
    pub fn has_dir(&self, dir:usize) -> bool {
        return self.val & dir != 0;
    }
    /// Returns all possible distances between the arrow cell and the nearest
    /// box cells in the pointed-to directions that don't contradict already
    /// known assignments of cells on the grid.
    pub fn possible_distances(self, dim:usize, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>) -> Vec<usize> {
        let mut possible_distances:HashMap<usize, Vec<bool>> = HashMap::new();
        for dir in ARROW_DIRS {
            let mut dir_possible_distances = vec![false;dim];
            if self.has_dir(dir) {
                // Possible distances are any unassigned cells in a pointed-to 
                // direction, up to first known_good cell. Distances off the 
                // board are not possible
                let mut x: usize = 1;
                while x < dim {
                    let coord = self.pos + (ARROW_DELTAS[dir] * x  as isize);
                    if !coord.in_bounds(dim) {
                        break;
                    }
                    if known_good.contains(&coord) {
                        dir_possible_distances[x] = true;
                        break;
                    }
                    else if !ruled_out.contains(&coord) {
                        dir_possible_distances[x] = true;
                    }
                    x += 1;
                }
            }
            else {
                // Possible distances are any distances less than first 
                // known_good cell in any non-pointed-to direction
                let mut x:usize = 1;
                while x < dim {
                    let coord = self.pos + (ARROW_DELTAS[dir] * x  as isize);
                    if !known_good.contains(&coord) {
                        dir_possible_distances[x] = true;
                    }
                    else {
                        dir_possible_distances[x] = false;
                        break;
                    }
                    x += 1;
                }
            }
            possible_distances.insert(dir, dir_possible_distances);
        }
        let mut result:Vec<usize> = Vec::new();
        for dist in 1..dim {
            let mut dist_okay = true;
            for dir in ARROW_DIRS {
                if !possible_distances.get(&dir).unwrap()[dist] {
                    dist_okay = false;
                    break;
                }
            }
            if dist_okay {
                result.push(dist);
            }
        }
        return result;
    }
    /// Test if current cell assignments violate the constraints of this arrow.
    pub fn cell_assignment_validate(self, dim:usize, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>) -> bool {
        // If there are possible distances for the arrow, the current assignments are invalid
        let valid_dists = self.possible_distances(dim, known_good, ruled_out);
        if valid_dists.len() == 0 {
            return false;
        }
        return true;
    }
    /// If a single possible distance is found in one direction that isn't 
    /// contradicted by the others, the arrow is solved.
    pub fn try_solve(&mut self, dim:usize, known_good:&mut HashSet<Vec2>, ruled_out:&mut HashSet<Vec2>) -> (bool, HashSet<Vec2>, HashSet<Vec2>)  {
        let mut newly_ruled_out:HashSet<Vec2> = HashSet::new();
        let mut newly_known_good:HashSet<Vec2> = HashSet::new();
        if self.solved {
            return (false, newly_known_good, newly_ruled_out);
        }
        let valid_dists = self.possible_distances(dim, known_good, ruled_out);
        if valid_dists.len() == 1 {
            let single_dist = valid_dists[0];
            self.solved = true;
            for dir in ARROW_DIRS {
                if self.has_dir(dir) {
                    for x in 1..single_dist {
                        let coord = self.pos + (ARROW_DELTAS[dir] * x as isize);
                        if coord.in_bounds(dim) && !ruled_out.contains(&coord) {
                            newly_ruled_out.insert(coord);
                        }
                    }
                    let final_coord = self.pos + (ARROW_DELTAS[dir] * single_dist as isize);
                    if final_coord.in_bounds(dim) && !known_good.contains(&final_coord) {
                        newly_known_good.insert(final_coord);
                    }
                }
                else {
                    for x in 1..=single_dist {
                        let coord = self.pos + (ARROW_DELTAS[dir] * x as isize);
                        if coord.in_bounds(dim) && !ruled_out.contains(&coord) {
                            newly_ruled_out.insert(coord);
                        }
                    }
                }
            }
            return (true, newly_known_good, newly_ruled_out);
        }
        return (false, newly_known_good, newly_ruled_out);
    }
}
impl std::fmt::Display for BoxArrow {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"{}", ARROW_STRINGS[self.val])
    }
}

/// Structure for storing number cell information on the grid.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct BoxNumber {
    pub pos:Vec2,
    pub val:usize,
    pub is_circle:bool,
    pub is_square:bool,
    pub solved:bool
}
impl BoxNumber {
    /// Test if the current cell assignments violate the contraints of this number.
    pub fn cell_assignment_validate(self, dim:usize, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>) -> bool  {
        let mut total_count = 0;
        let mut assigned_count = 0;
        let mut known_good_count = 0;
        for delta in NUMBER_DELTAS {
            let coord = self.pos + delta;
            if coord.in_bounds(dim) {
                total_count += 1;
            }
            if known_good.contains(&coord) {
                known_good_count += 1;
                assigned_count += 1;
            }
            else if ruled_out.contains(&coord) {
                assigned_count += 1;
            }
        }
        if known_good_count > self.val {
            return false
        }
        if assigned_count == total_count && known_good_count != self.val {
            return false
        }            
        return true;
    }
    /// The BoxNumber can be solved if there is an unambigious way to assign 
    /// the remaining cells around it so that the count matches the number's 
    /// value
    pub fn try_solve(&mut self, dim:usize, known_good:&mut HashSet<Vec2>, ruled_out:&mut HashSet<Vec2>) -> (bool, HashSet<Vec2>, HashSet<Vec2>)  {
        let mut newly_ruled_out:HashSet<Vec2> = HashSet::new();
        let mut newly_known_good:HashSet<Vec2> = HashSet::new();
        if self.solved {
            return (false, newly_known_good, newly_ruled_out);
        }
        let mut total_count = 0;
        let mut assigned_count = 0;
        let mut known_good_count = 0;
        let mut ruled_out_count = 0;
        for delta in NUMBER_DELTAS {
            let coord = self.pos + delta;
            if coord.in_bounds(dim) {
                total_count += 1;
            }
            if known_good.contains(&coord) {
                known_good_count += 1;
                assigned_count += 1;
            }
            else if ruled_out.contains(&coord) {
                ruled_out_count += 1;
                assigned_count += 1;
            }
        }
        // If all kings-move distant cells have been previously assigned, the 
        // number is already solved
        if assigned_count == total_count {
            self.solved = true;
            return (false, newly_known_good, newly_ruled_out);
        }
        // If the number of kings-move distant cells box cells that have been 
        // previously assigned and are box cells matches the number of the 
        // cell, the remaining unassigned cells must not be box cells 
        if known_good_count == self.val {
            self.solved = true;
            for delta in NUMBER_DELTAS {
                let coord = self.pos + delta;
                if coord.in_bounds(dim) && !known_good.contains(&coord) && !ruled_out.contains(&coord) {
                    ruled_out.insert(coord);
                    newly_ruled_out.insert(coord);
                }
            }
            return (true, newly_known_good, newly_ruled_out);
        }
        // If the number of kings-move distant cells box cells that have been 
        // previously assigned and are non-box cells matches the 9 - the number 
        // of the cell, the remaining unassigned cells must be box cells. 
        else if ruled_out_count == total_count - self.val {
            self.solved = true;
            for delta in NUMBER_DELTAS {
                let coord = self.pos + delta;
                if coord.in_bounds(dim) && !known_good.contains(&coord) && !ruled_out.contains(&coord) {
                    known_good.insert(coord);
                    newly_known_good.insert(coord);
                }
            }
            return (true, newly_known_good, newly_ruled_out);
        }
        // Otherwise, the number cell cannot be currently solved
        return (false, newly_known_good, newly_ruled_out);
    }
}
impl std::fmt::Display for BoxNumber {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"{}", self.val)
    }
}

/// Structure for storing grid cell information
#[derive(Clone)]
pub struct BoxGrid {
    pub dim: usize,
    pub arrows:HashMap<Vec2,BoxArrow>,
    pub numbers:HashMap<Vec2,BoxNumber>
}
impl BoxGrid {
    /// Create a new box grid, using the sample grid information if `sample` is
    /// true and using the main puzzle grid information otherwise. 
    pub fn new(sample:bool) -> BoxGrid{
        let numbers_string = if sample { SAMPLE_GRID_NUMBERS.trim().to_string() } else { MAIN_GRID_NUMBERS.trim().to_string() };
        let arrows_string = if sample { SAMPLE_GRID_ARROWS.trim().to_string() } else { MAIN_GRID_ARROWS.trim().to_string() };
        let numbers_lines:Vec<&str> = numbers_string.split("\n").collect();
        let arrows_lines:Vec<&str> = arrows_string.split("\n").collect();
        let dim = if sample {8} else {20};
        assert!(dim == numbers_lines.len(), "Grid number string does not have dimension {} vertically", dim);
        assert!(dim == arrows_lines.len(), "Grid arrow string does not have dimension {} vertically", dim);
        let mut numbers:HashMap<Vec2, BoxNumber> = HashMap::new();
        let mut arrows:HashMap<Vec2, BoxArrow> = HashMap::new();
        for y in 0..numbers_lines.len() {
            let numbers_line_cols:Vec<&str> = numbers_lines[y].split(" ").collect();
            assert!(numbers_line_cols.len() == dim, "Grid numbers string does not have dimension {} on row {}", dim, y);
            for x in 0..numbers_line_cols.len() {
                
                if numbers_line_cols[x] == "." {
                    continue;
                }
                else {
                    let val = usize::from_str_radix(numbers_line_cols[x], 10).expect("not a hex number");
                    let coord = Vec2::newu(x,y);
                    let is_circle = if sample {SAMPLE_GRID_CIRCLES.contains(&coord)} else {MAIN_GRID_CIRCLES.contains(&coord)};
                    let is_square = if sample {SAMPLE_GRID_SQUARES.contains(&coord)} else {MAIN_GRID_SQUARES.contains(&coord)};
                    numbers.insert(coord, BoxNumber{pos:coord, val:val, is_circle:is_circle, is_square:is_square, solved:false});
                }
            }
        }
        for y in 0..arrows_lines.len() {
            let arrows_line_cols:Vec<&str> = arrows_lines[y].split(" ").collect();
            assert!(arrows_line_cols.len() == dim, "Grid arrows string does not have dimension {} on row {}", dim, y);
            for x in 0..arrows_line_cols.len() {
                
                if arrows_line_cols[x] == "." {
                    continue;
                }
                else {
                    let val = usize::from_str_radix(arrows_line_cols[x], 16).expect("not a hex number");
                    let coord = Vec2::newu(x,y);
                    arrows.insert(coord, BoxArrow{pos:coord, val:val, dist:0, solved:false});
                }
            }
        }
        return BoxGrid{dim:dim, arrows:arrows, numbers:numbers};
    }

    /// Flood fill to find all cells of the specified types orthogonally 
    /// connected to `coord`.
    pub fn flood_fill(&self, coord:Vec2, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>, include_known_good:bool, include_ruled_out:bool, include_unassigned:bool) -> HashSet<Vec2> {
        assert!(coord.in_bounds(self.dim), "Flood fill pos {} is not in bounds", coord);
        let mut group:HashSet<Vec2> = HashSet::new();
        let mut frontier:Vec<Vec2> = Vec::new();
        if known_good.contains(&coord) {
            if include_known_good {
                frontier.push(coord);
            }
        }
        else if ruled_out.contains(&coord) {
            if include_ruled_out {
                frontier.push(coord);
            }
        }
        else {
            if include_unassigned {
                frontier.push(coord);
            }
        }
        while frontier.len() > 0 {
            let mut frontier_next:Vec<Vec2> = Vec::new();
            while frontier.len() > 0 {
                let frontier_coord = frontier.pop().unwrap();
                if group.contains(&frontier_coord) {
                    continue;
                }
                group.insert(frontier_coord);
                for dir in ARROW_DIRS {
                    let neighbor_coord = frontier_coord + ARROW_DELTAS[dir];
                    if !neighbor_coord.in_bounds(self.dim) || group.contains(&neighbor_coord){
                        continue;
                    }
                    if known_good.contains(&neighbor_coord) {
                        if include_known_good {
                            frontier_next.push(neighbor_coord);
                        }
                    }
                    else if ruled_out.contains(&neighbor_coord) {
                        if include_ruled_out {
                            frontier_next.push(neighbor_coord);
                        }
                    }
                    else {
                        if include_unassigned {
                            frontier_next.push(neighbor_coord);
                        }
                    }
                }
            }
            frontier.extend(&frontier_next);
        }
        return group;
    }
    
    /// Returns true if the current assignments of cells on the grid don't 
    /// violate the contraints for any number or arrow cells on the grid.
    /// If all cells have been assigned, verify that the box cell group is 
    /// fully connected, that all non-box cell groups are connected to the grid
    /// border, and that the number of box cells is even since an odd number of
    /// cells could not form the faces of a rectangular solid.
    pub fn cell_assignment_validate(&self, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>) -> bool {
        let set_intersection = known_good.intersection(&ruled_out);
        assert!(set_intersection.clone().count() == 0, "Intersection found between known_good and ruled_out {:?}", set_intersection);
        // Test the contraints of all number cells
        for number_pos in self.numbers.keys() {
            if !self.numbers.get(number_pos).unwrap().cell_assignment_validate(self.dim, known_good, ruled_out) {
                return false;
            }     
        }
        // Test the contraints of all arrow cells
        for arrow_pos in self.arrows.keys() {
            if !self.arrows.get(arrow_pos).unwrap().cell_assignment_validate(self.dim, known_good, ruled_out) {
                return false;
            }     
        }
        // Test for any groups of box cells fully isolated by non-box cells.
        let mut visited_known_good_ambigious:HashSet<Vec2> = HashSet::new();
        let mut known_good_ambigious_groups:Vec<HashSet<Vec2>> = Vec::new();
        for good_pos in known_good.iter() {
            if visited_known_good_ambigious.contains(good_pos) {
                continue;
            }
            let group:HashSet<Vec2> = self.flood_fill(*good_pos, known_good, ruled_out, true, false, true);
            visited_known_good_ambigious.extend(&group);
            known_good_ambigious_groups.push(group);
        }
        if known_good_ambigious_groups.len() > 1 {
            return false;
        }
        // If all cells are assigned test:
        // - The number of box cells is even and non-zero.
        // - All box cells are fully connected
        // - All groups of non-box cells are adjacent to the grid border
        if known_good.len() + ruled_out.len() == self.dim * self.dim {
            if known_good.len() == 0 {
                return false;
            }
            if known_good.len() % 2 != 0 {
                return false;
            }
            let start_coord = *known_good.iter().next().unwrap();
            let good_group = self.flood_fill(start_coord, known_good, ruled_out, true, false, false);
            if good_group.len() != known_good.len() {
                return false;
            }
            let mut visited_ruled_out:HashSet<Vec2> = HashSet::new();
            for ruled_out_pos in ruled_out.iter() {
                if visited_ruled_out.contains(ruled_out_pos) {
                    continue;
                }
                let ruled_out_group = self.flood_fill(*ruled_out_pos, known_good, ruled_out, false, true, false);
                let mut on_border = false;
                for pos in ruled_out_group.iter() {
                    if pos.x == 0 || pos.y == 0 || pos.x == (self.dim - 1) as isize || pos.y == (self.dim - 1) as isize {
                        on_border = true;
                        break;
                    }
                }
                if on_border {
                    for pos in ruled_out_group.iter() {
                        visited_ruled_out.insert(*pos);
                    }
                }
                else {
                    return false;
                }
            }
        }
        return true;
    }

    /// Infer assignments to box cell and non-box cell groups for as many 
    /// unassigned cells on the board as possible using several rules:
    /// - If there is a single possible distance for any arrow, assign box and 
    ///   non-box cells up to that distance from the arrow as appropriate.
    /// - If the remaining ambigous cells within a kings-move distance of a 
    ///   number would come to the correct total if they were all box cells or 
    ///   all non-box cells, assign as appropriate.
    /// - If two arrows that haven't been previously solved are pointing at 
    ///   each other and there are no assigned box cells between them and a 
    ///   single unassigned cell, that cell must be a box cell (which then 
    ///   gives the distances for both arrows).
    /// - If there are any groups of unassigned cells that are surrounded by 
    ///   box cells cells (or the grid border), they must be non-box cells 
    ///   because all box cells must be connected.
    /// - If there are any groups of unassigned cells that are surrounded by 
    ///   box-cells cells (and not the border), they must be box cells because 
    ///   the box cells cannot contain any holes.
    pub fn inference(&mut self, known_good:&mut HashSet<Vec2>, ruled_out:&mut HashSet<Vec2>) {
        // For arrows pointing at each other
        let inverse_directions = HashMap::from([
            (NORTH, SOUTH),
            (EAST, WEST),
            (SOUTH, NORTH),
            (WEST, EAST),
        ]);
        // If not previously added to the box cell and non-box cell sets, add 
        // the grid numbers and arrows to each set.
        for number_pos in self.numbers.keys() {
            if !known_good.contains(number_pos) {
                known_good.insert(*number_pos);
            }
        }
        for arrow_pos in self.arrows.keys() {
            if !ruled_out.contains(arrow_pos) {
                ruled_out.insert(*arrow_pos);
            }
        }
        // For each arrow, the first cell in each not-pointed-to direction must
        // be a non-box cell.
        for arrow_pos in self.arrows.keys() {
            for dir in ARROW_DIRS {
                if !self.arrows.get(arrow_pos).unwrap().has_dir(dir) {
                    let coord = *arrow_pos + ARROW_DELTAS[dir];
                    if !coord.in_bounds(self.dim) {
                        continue;
                    }
                    if !known_good.contains(&coord) && !ruled_out.contains(&coord) {
                        ruled_out.insert(coord);
                    }
                }
            }
        }
        if !self.cell_assignment_validate(known_good, ruled_out) {
            return;
        }
        // Continually apply inference rules until no more cells can be 
        // assigned to the box cell and non-box cell sets.
        loop {
            let mut newly_known_good:HashSet<Vec2> = HashSet::new();
            let mut newly_ruled_out:HashSet<Vec2> = HashSet::new();
            // Solve any arrow cells that can be solved from the current state 
            // of assigned cells
            for arrow in self.arrows.values_mut() {
                let (success, arrow_newly_known_good, arrow_newly_ruled_out) = arrow.try_solve(self.dim, known_good, ruled_out);
                if success {
                    let mut okay_to_add = true;
                    for pos in arrow_newly_known_good.iter() {
                        if ruled_out.contains(&pos) {
                            okay_to_add = false;
                            break;
                        }
                    }
                    for pos in arrow_newly_ruled_out.iter() {
                        if known_good.contains(&pos) {
                            okay_to_add = false;
                            break;
                        }
                    }
                    if okay_to_add {
                        known_good.extend(&arrow_newly_known_good);
                        ruled_out.extend(&arrow_newly_ruled_out);
                        newly_known_good.extend(&arrow_newly_known_good);
                        newly_ruled_out.extend(&arrow_newly_ruled_out);
                    }
                }
            }
            // Solve any number cells that can be solved from the current state
            // of assigned cells
            for number in self.numbers.values_mut() {
                let (success, number_newly_known_good, number_newly_ruled_out) = number.try_solve(self.dim, known_good, ruled_out);
                if success {
                    let mut okay_to_add = true;
                    for pos in number_newly_known_good.iter() {
                        if ruled_out.contains(&pos) {
                            okay_to_add = false;
                            break;
                        }
                    }
                    for pos in number_newly_ruled_out.iter() {
                        if known_good.contains(&pos) {
                            okay_to_add = false;
                            break;
                        }
                    }
                    if okay_to_add {
                        known_good.extend(&number_newly_known_good);
                        ruled_out.extend(&number_newly_ruled_out);
                        newly_known_good.extend(&number_newly_known_good);
                        newly_ruled_out.extend(&number_newly_ruled_out);
                    }
                }
            }
            // Look for two unsolved arrows pointing at each other with one 
            // unassigned cell and no assigned box cells between them.
            for arrow_pos in self.arrows.keys() {
                let arrow = self.arrows.get(arrow_pos).unwrap();
                if arrow.solved {
                    continue;
                }
                for dir in ARROW_DIRS {
                    if arrow.has_dir(dir) {
                        let mut ambigious_cells:Vec<Vec2> = Vec::new();
                        let mut non_ambigious_cells_count = 0;
                        let mut x = 1;
                        loop {
                            let coord = *arrow_pos + (ARROW_DELTAS[dir] * x);
                            if !coord.in_bounds(self.dim) {
                                break;
                            }
                            if !known_good.contains(&coord) && !ruled_out.contains(&coord) {
                                ambigious_cells.push(coord);
                            }
                            else {
                                non_ambigious_cells_count += 1;
                            }
                            if self.arrows.contains_key(&coord) && self.arrows.get(&coord).unwrap().has_dir(*inverse_directions.get(&dir).unwrap()) {
                                if ambigious_cells.len() == 1 && non_ambigious_cells_count == 0 {
                                    known_good.insert(ambigious_cells[0]);
                                    newly_known_good.insert(ambigious_cells[0]);
                                }
                                break;
                            }
                            x += 1;
                        }

                    }
                }
            }
            // Look for groups of unassigned cells isolated by non-box cells
            let mut added_to_ruled_out_from_isolated_groups:HashSet<Vec2> = HashSet::new();
            let mut ruled_out_isolated_visited:HashSet<Vec2> = HashSet::new();
            let mut ruled_out_isolated_groups:Vec<HashSet<Vec2>> = Vec::new();
            for y in 0..self.dim {
                for x in 0..self.dim {
                    let start_coord = Vec2::newu(x,y);
                    if known_good.contains(&start_coord) || newly_known_good.contains(&start_coord) || ruled_out.contains(&start_coord) || newly_ruled_out.contains(&start_coord){
                        continue;
                    }
                    if ruled_out_isolated_visited.contains(&start_coord) {
                        continue;
                    }
                    let group:HashSet<Vec2> = self.flood_fill(start_coord, known_good, ruled_out, true, false, true);
                    ruled_out_isolated_visited.extend(&group);
                    let mut is_isolated = true;
                    for coord in group.iter() {
                        if known_good.contains(coord) {
                            is_isolated = false;
                        }
                    }
                    if is_isolated {
                        ruled_out_isolated_groups.push(group);
                    }
                }
            }
            for group in ruled_out_isolated_groups {
                for coord in group {
                    ruled_out.insert(coord);
                    newly_ruled_out.insert(coord);
                    added_to_ruled_out_from_isolated_groups.insert(coord);
                }
            }
            // Look for groups of ambigious cells isolated by known_good cells 
            // (and not the grid border)
            let mut added_to_known_good_from_isolated_groups:HashSet<Vec2> = HashSet::new();
            let mut known_good_isolated_visited:HashSet<Vec2> = HashSet::new();
            let mut known_good_isolated_groups:Vec<HashSet<Vec2>> = Vec::new();
            for y in 0..self.dim {
                for x in 0..self.dim {
                    let start_coord = Vec2::newu(x,y);
                    if known_good.contains(&start_coord) || ruled_out.contains(&start_coord) {
                        continue;
                    }
                    if known_good_isolated_visited.contains(&start_coord) {
                        continue;
                    }
                    let group:HashSet<Vec2> = self.flood_fill(start_coord, known_good, ruled_out, false, true, true);
                    known_good_isolated_visited.extend(&group);
                    let mut is_isolated = true;
                    let mut touches_border = false;
                    for coord in group.iter() {
                        if ruled_out.contains(coord) {
                            is_isolated = false;
                        }
                        if coord.x == 0 || coord.y == 0 || coord.x == (self.dim - 1) as isize || coord.y == (self.dim - 1) as isize {
                            touches_border = true;
                        }
                    }
                    if is_isolated && !touches_border {
                        known_good_isolated_groups.push(group);
                    }
                }
            }
            for group in known_good_isolated_groups {
                for coord in group {
                    known_good.insert(coord);
                    newly_known_good.insert(coord);
                    added_to_known_good_from_isolated_groups.insert(coord);
                }
            }
            // Validate current cell assignments
            if !self.cell_assignment_validate(known_good, ruled_out) {
                return;
            }
            // If no further assignments have been made, exit the loop.
            if newly_known_good.len() + newly_ruled_out.len() == 0 {
                break;
            }
        }
    }
    /// Given inferred partial assignments to box cells and non-box cells, 
    /// recursively try speculative assignments for remaining unsolved numbers, 
    /// arrows and extra cells and return all valid full assignments of cells 
    /// on the board.
    pub fn speculation(&self, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>) -> Vec<(HashSet<Vec2>, HashSet<Vec2>)> {
        let mut results:Vec<(HashSet<Vec2>, HashSet<Vec2>)> = Vec::new();
        if !self.cell_assignment_validate(known_good, ruled_out) {
            return results;
        }
        let mut unsolved_numbers:Vec<Vec2> = Vec::new();
        let mut unsolved_arrows:Vec<Vec2> = Vec::new();
        for y in 0..self.dim {
            for x in 0..self.dim {
                let coord = Vec2::newu(x,y);
                if self.numbers.contains_key(&coord) && !self.numbers.get(&coord).unwrap().solved {
                    unsolved_numbers.push(coord);
                }
                if self.arrows.contains_key(&coord) && !self.arrows.get(&coord).unwrap().solved {
                    unsolved_arrows.push(coord);
                }
            }
        }
        if unsolved_numbers.len() > 0 {
            let number_pos = unsolved_numbers[0];
            // Try all possibilities for cell assignments within a kings-move
            // distance for the first unsolved number.
            for bits in 0..512 {
                let mut number_known_good = known_good.clone();
                let mut number_ruled_out = ruled_out.clone();
                let mut contradiction = false;
                for index in 0..NUMBER_DELTAS.len() {
                    let coord = number_pos + NUMBER_DELTAS[index];
                    if !coord.in_bounds(self.dim) {
                        continue;
                    }
                    let bit = bits >> index & 1;
                    if bit == 1 {
                        if number_ruled_out.contains(&coord) {
                            contradiction = true;
                            break;
                        }
                        else {
                            number_known_good.insert(coord);
                        }
                    }
                    else {
                        if number_known_good.contains(&coord) {
                            contradiction = true;
                            break;
                        }
                        else {
                            number_ruled_out.insert(coord);
                        }
                    }
                }
                if contradiction {
                    continue;
                }
                let mut number_grid = self.clone();
                // If the assignment is valid, perform inference step for any 
                // further assignments that can be made and then recurse.
                if number_grid.cell_assignment_validate(&number_known_good, &number_ruled_out) {
                    number_grid.numbers.get_mut(&number_pos).unwrap().solved = true;
                    number_grid.inference(&mut number_known_good, &mut number_ruled_out);
                    let number_results = number_grid.speculation(&number_known_good, &number_ruled_out);
                    for number_result in number_results {
                        results.push(number_result);
                    }
                }
            }
        }
        else if unsolved_arrows.len() > 0 {
            // Try all possible distance assigments for the first unsolved 
            // arrow
            let arrow_pos = unsolved_arrows[0];
            let possible_distances = self.arrows.get(&arrow_pos).unwrap().possible_distances(self.dim, known_good, ruled_out);
            for dist in possible_distances {
                let mut arrow_known_good = known_good.clone();
                let mut arrow_ruled_out = ruled_out.clone();
                // Assign box and non-box cells based on the speculative 
                // distance
                for dir in ARROW_DIRS {
                    if self.arrows.get(&arrow_pos).unwrap().has_dir(dir) {
                        for x in 1..dist {
                            let coord = arrow_pos + (ARROW_DELTAS[dir] * x as isize);
                            assert!(coord.in_bounds(self.dim), "Invalid assignment of bad cell in pointed-to direction at dist {} dir {} coord {} for arrow {}: out of bounds", dist, dir, coord, arrow_pos);
                            assert!(!known_good.contains(&coord), "Invalid assignment of ruled_out cell at pos {} for arrow in pointed-to dir {} at x {} for dist {}", coord, dir, x, dist);
                            if !arrow_ruled_out.contains(&coord) {
                                arrow_ruled_out.insert(coord);
                            }
                        }
                        let coord = arrow_pos + (ARROW_DELTAS[dir] * dist as isize);
                        assert!(coord.in_bounds(self.dim), "Invalid assignment of good cell in pointed-to direction at dist {} dir {} coord {} for arrow {}: out of bounds", dist, dir, coord, arrow_pos);
                        assert!(!ruled_out.contains(&coord), "Invalid assignment of known_good cell at pos {} for arrow in pointed-to dir {} at dist {}", coord, dir, dist);
                        if !arrow_known_good.contains(&coord) {
                            arrow_known_good.insert(coord);
                        }
                    }
                    else {
                        for x in 1..dist + 1 {
                            let coord = arrow_pos + (ARROW_DELTAS[dir] * x as isize);
                            if !coord.in_bounds(self.dim) {
                                continue;   
                            }
                            assert!(!known_good.contains(&coord), "Invalid assignment of ruled_out cell at pos {} for arrow in unpointed-to dir {} at x {} for dist {}", coord, dir, x, dist);
                            if !arrow_ruled_out.contains(&coord) {
                                arrow_ruled_out.insert(coord);
                            }
                        }
                    }
                }
                let mut arrow_grid = self.clone();
                // If the assignment is valid, perform inference step for any 
                // further assignments that can be made and then recurse.
                if arrow_grid.cell_assignment_validate(&arrow_known_good, &arrow_ruled_out) {
                    arrow_grid.arrows.get_mut(&arrow_pos).unwrap().solved = true;
                    arrow_grid.inference(&mut arrow_known_good, &mut arrow_ruled_out);
                    let arrow_results = arrow_grid.speculation(&arrow_known_good, &arrow_ruled_out);
                    for arrow_result in arrow_results {
                        results.push(arrow_result);
                    }
                }
            }

        }
        else {
            let mut remaining:Vec<Vec2> = Vec::new();
            for y in 0..self.dim {
                for x in 0..self.dim {
                    let coord = Vec2::newu(x,y);
                    if !known_good.contains(&coord) && !ruled_out.contains(&coord) {
                        remaining.push(coord);
                    }
                }
            }
            if remaining.len() > 0 {
                // Try both possible assignments for first remaining unassigned
                // cell. If the assignments are valid, perform inference step 
                // for any further assignments that can be made and then 
                // recurse.
                let mut good_known_good = known_good.clone();
                let mut good_ruled_out = ruled_out.clone();
                good_known_good.insert(remaining[0]);
                let mut remaining_good_grid = self.clone();
                if remaining_good_grid.cell_assignment_validate(&good_known_good, &good_ruled_out) {
                    remaining_good_grid.inference(&mut good_known_good, &mut good_ruled_out);
                    let good_results = remaining_good_grid.speculation(&good_known_good, ruled_out);
                    for good_result in good_results {
                        results.push(good_result);
                    }
                }
                let mut bad_known_good = known_good.clone();
                let mut bad_ruled_out = ruled_out.clone();
                bad_ruled_out.insert(remaining[0]);
                let mut remaining_bad_grid = self.clone();
                if remaining_bad_grid.cell_assignment_validate(&bad_known_good, &bad_ruled_out) {
                    remaining_bad_grid.inference(&mut bad_known_good, &mut bad_ruled_out);
                    let bad_results = remaining_bad_grid.speculation(known_good, &bad_ruled_out);
                    for bad_result in bad_results {
                        results.push(bad_result);
                    }
                }
            }
            else {
                // If the assignment of cells is complete and valid, return the
                // assignment
                if self.cell_assignment_validate(known_good, ruled_out) {
                    results.push((known_good.clone(), ruled_out.clone()));
                }
            }
        }
        return results;
    }
    /// Find a path within the box cell set on the grid between `start_coord` 
    /// and `end_coord` and return the path as a sequence of N/E/S/W directions
    pub fn grid_traverse(known_good:&HashSet<Vec2>, start_coord:Vec2, end_coord:Vec2) -> Vec<usize> {
        let mut reached:HashMap<Vec2, usize> = HashMap::new();
        let mut frontier:Vec<(Vec2, Vec<usize>)> = Vec::new();
        frontier.push((start_coord, Vec::new()));
        while frontier.len() > 0 {
            let mut frontier_next:Vec<(Vec2, Vec<usize>)> = Vec::new();
            while frontier.len() > 0 {
                let path = frontier.pop().unwrap();
                if reached.contains_key(&path.0) {
                    if *(reached.get(&path.0).unwrap()) <= path.1.len() {
                        continue;
                    }
                    else {
                        *reached.get_mut(&path.0).unwrap() = path.1.len();
                    }
                }
                else {
                    reached.insert(path.0, path.1.len());
                }
                if path.0 == end_coord {
                    return path.1;
                }
                for dir in ARROW_DIRS {
                    let next_coord = path.0 + ARROW_DELTAS[dir];
                    if known_good.contains(&next_coord) {
                        let mut next_path = path.1.clone();
                        next_path.push(dir);
                        frontier_next.push((next_coord, next_path));
                    }
                }
            }
            frontier.extend(frontier_next);
        }
        return Vec::new();
    }
    /// Search for a mapping of box cells onto a rectangular solid that 
    /// satisfies the puzzle constraints given a full assignment of cells.
    pub fn solidify(&self, known_good:&HashSet<Vec2>, ruled_out:&HashSet<Vec2>, verbose:bool) -> Result<Option<usize>, String> {
        if known_good.len() + ruled_out.len() != self.dim * self.dim { return Err("Incomplete cell assignments given".to_string()); }
        for good_pos in known_good.iter() {
            if ruled_out.contains(good_pos) { return Err(format!("Overlap between known_good and ruled_out for cell {}", good_pos));}
            if !good_pos.in_bounds(self.dim) { return Err(format!("known_good position {} out of bounds", good_pos));}
        }
        for bad_pos in ruled_out.iter() {
            if known_good.contains(bad_pos) { return Err(format!("Overlap between known_good and ruled_out for cell {}", bad_pos));}
            if !bad_pos.in_bounds(self.dim) { return Err(format!("ruled_out position {} out of bounds", bad_pos)); }
        }
        // Determine possible solid dimensions for this layout
        let mut possible_dimensions:Vec<Vec3> = Vec::new();
        for x in 1..=self.dim {
            for y in 1..=self.dim {
                for z in 1..=self.dim {
                    if 2*x*y + 2*y*z + 2*x*z == known_good.len() {
                        possible_dimensions.push(Vec3::newu(x,y,z));
                    }
                }
            }
        }
        // Pick a root cell
        let mut root_cell = Vec2::new(0,0);
        for y in 0..self.dim {
            for x in 0..self.dim {
                let coord = Vec2::newu(x,y);
                if known_good.contains(&coord) {
                    root_cell = coord;
                    break;
                }
            }
        }
        // For each possible solid, test the possibilty that the root cell
        // is mapped to each cell on the first face.
        for dimensions in possible_dimensions {
            let solid = BoxSolid::new(dimensions.x as usize,dimensions.y as usize,dimensions.z as usize);
            for a_x in 0..dimensions.x {
                for a_y in 0..dimensions.y {
                    let mut solid_mapping_okay = true;
                    let mut cell_mapping:HashMap<Vec2,BoxFaceCoord> = HashMap::new();
                    let mut inverse_cell_mapping:HashMap<BoxFaceCoord,Vec2> = HashMap::new();
                    let root_box_coord = BoxFaceCoord{coord: Vec3{x:a_x as isize, y:a_y as isize, z:0}, face:BoxFaceType::A};
                    cell_mapping.insert(root_cell, root_box_coord);
                    inverse_cell_mapping.insert(root_box_coord, root_cell);
                    // For each other box cell on the grid, find a mapping for 
                    // a face cell on the solid using its relative position to
                    // the root cell.
                    for cell in known_good.iter() {
                        if *cell == root_cell {
                            continue;
                        }
                        // Find a path from the root cell to the destination 
                        // cell in the grid
                        let path = BoxGrid::grid_traverse(known_good, root_cell, *cell);
                        if path.len() == 0 { return Err(format!("Path from root cell {} to position {} failed", root_cell, cell)); }
                        // Trace the path over the surface of the box to locate
                        // the corresponding location of destination cell
                        let mut path_box_coord = root_box_coord.clone();
                        let mut path_axis_a = Vec3::new(1,0,0);
                        let mut path_axis_b = Vec3::new(0,1,0);
                        for step in path {
                            match solid.traverse(step, path_box_coord, path_axis_a, path_axis_b) {
                                Ok(tuple) => {
                                    path_box_coord = tuple.0;
                                    path_axis_a = tuple.1;
                                    path_axis_b = tuple.2;
                                },
                            Err(e) => {
                                    return Err(format!("Error solving: {}", e));
                                }
                            }
                        }
                        // If there is a collision on the outcome face 
                        // coordinate, the root cell face position is incorrect
                        if inverse_cell_mapping.contains_key(&path_box_coord) {
                            solid_mapping_okay = false;
                            break;
                        }
                        cell_mapping.insert(*cell, path_box_coord);
                        inverse_cell_mapping.insert(path_box_coord, *cell);
                    }
                    if !solid_mapping_okay {
                        continue;
                    }
                    // Validate circle and square positions mapped onto the 
                    // solid
                    let mut circles_okay = true;
                    let mut squares_okay = true;
                    for number_pos in self.numbers.keys() {
                        let number = self.numbers.get(&number_pos).unwrap();
                        if number.is_circle {
                            let circle_box_coord = cell_mapping.get(&number.pos).unwrap();
                            let opposite_coord:BoxFaceCoord = match circle_box_coord.face {
                                BoxFaceType::A => BoxFaceCoord{coord:Vec3::new(circle_box_coord.coord.x, circle_box_coord.coord.y, (solid.z - 1) as isize), face:BoxFaceType::F},
                                BoxFaceType::B => BoxFaceCoord{coord:Vec3::new(circle_box_coord.coord.x, 0, circle_box_coord.coord.z as isize), face:BoxFaceType::D},
                                BoxFaceType::C => BoxFaceCoord{coord:Vec3::new((solid.x - 1) as isize, circle_box_coord.coord.y, circle_box_coord.coord.z  as isize), face:BoxFaceType::E},
                                BoxFaceType::D => BoxFaceCoord{coord:Vec3::new(circle_box_coord.coord.x, (solid.y - 1) as isize, circle_box_coord.coord.z as isize), face:BoxFaceType::B},
                                BoxFaceType::E => BoxFaceCoord{coord:Vec3::new(0, circle_box_coord.coord.y, circle_box_coord.coord.z), face:BoxFaceType::C},
                                BoxFaceType::F => BoxFaceCoord{coord:Vec3::new(circle_box_coord.coord.x, circle_box_coord.coord.y, 0), face:BoxFaceType::A}
                            };
                            let opposite_cell = inverse_cell_mapping.get(&opposite_coord).unwrap();
                            if !self.numbers.contains_key(&opposite_cell) || !self.numbers.get(&opposite_cell).unwrap().is_circle {
                                circles_okay = false;
                                break;
                            }
                        }
                        else if number.is_square {
                            let square_box_coord = cell_mapping.get(&number.pos).unwrap();
                            let mut square_within_range = false;
                            for other_number_pos in self.numbers.keys() {
                                if other_number_pos == number_pos {
                                    continue;
                                }
                                let other_number = self.numbers.get(&other_number_pos).unwrap();
                                if !other_number.is_square {
                                    continue;
                                }
                                let other_number_coord = cell_mapping.get(&other_number.pos).unwrap();
                                if square_box_coord.face == square_box_coord.face {
                                    let manhattan_distance = (other_number_coord.coord.x - square_box_coord.coord.x).abs() + (other_number_coord.coord.y - square_box_coord.coord.y).abs() + (other_number_coord.coord.z - square_box_coord.coord.z).abs();
                                    if manhattan_distance == 1 {
                                        square_within_range = true;
                                        break;
                                    }
                                }
                            }
                            if !square_within_range {
                                squares_okay = false;
                                break;
                            }
                        }
                    }
                    if circles_okay && squares_okay {
                        if verbose {
                            println!("Box cells on the grid:");
                            self.print_set(false, false, known_good);
                            println!("Box dimensions: {}", dimensions);
                            println!("Circle positions on the flattened box:");
                            solid.print(&inverse_cell_mapping, self, true, false);
                            println!("Square positions on the flattened box:");
                            solid.print(&inverse_cell_mapping, self, false, true);
                            println!("Number positions on the flattened box:");
                            solid.print(&inverse_cell_mapping, self, false, false);
                        }
                        let mut face_product = 1;
                        for face in BOXFACETYPES {
                            let mut face_sum = 0;
                            for face_coord in inverse_cell_mapping.keys() {
                                if face_coord.face == face {
                                    let cell_pos = inverse_cell_mapping.get(face_coord).unwrap();
                                    if self.numbers.contains_key(&cell_pos) {
                                        face_sum += self.numbers.get(&cell_pos).unwrap().val;
                                    }
                                }
                            }
                            face_product *= face_sum;
                        }
                        return Ok(Some(face_product));
                    }
                }
            }
        }
        return Ok(None);
    }
    /// Print the given `set` of coordinates on the grid, with arrow or number 
    /// cells optionally included.
    pub fn print_set(&self, print_arrows:bool, print_numbers:bool, set:&HashSet<Vec2>) {
        for y in 0..self.dim {
            for x in 0..self.dim {
                let coord = Vec2::newu(x,y);
                if print_arrows && self.arrows.contains_key(&coord) {
                    print!("{} ", self.arrows.get(&coord).unwrap());
                }
                else if print_numbers && self.numbers.contains_key(&coord) {
                    print!("{} ", self.numbers.get(&coord).unwrap());
                }
                else if set.contains(&coord) {
                    print!("# ");
                }
                else {
                    print!(". ");
                }
            }
            println!("");
        }
    }
}

/// Structure for storing a 3d position and box face representing a cell on the
/// surface of a box.
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct BoxFaceCoord {
    pub coord:Vec3,
    pub face:BoxFaceType
}
impl std::fmt::Display for BoxFaceCoord {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({} {:?})", self.coord, self.face)
    }
}

/// Structure for storing spatial information about a face of a box.
pub struct BoxFace {
    pub face:BoxFaceType,
    pub vec_a:Vec3,
    pub vec_b:Vec3,
    pub vec_normal:Vec3
}
impl BoxFace {
    /// Render the box face using the given `mapping` to the `grid` as part of 
    /// an unfolded diagram of the box. 
    pub fn to_lines(&self, box_dim: Vec3, mapping:&HashMap<BoxFaceCoord, Vec2>, grid:&BoxGrid, circles_only:bool, squares_only:bool) -> Vec<String> {
        let mut lines:Vec<String> = Vec::new();
        let (corner_vec, vec_a, vec_b, dim_a, dim_b) = match self.face {
            BoxFaceType::A => {
                (Vec3::new(0,0,0),
                Vec3::new(1,0,0),
                Vec3::new(0,1,0),
                box_dim.x,
                box_dim.y)
            },
            BoxFaceType::B => {
                (Vec3::new(0,box_dim.y - 1, 0),
                Vec3::new(1,0,0),
                Vec3::new(0,0,1),
                box_dim.x,
                box_dim.z)
            },
            BoxFaceType::C => {
                (Vec3::new(0,0,0),
                Vec3::new(0,1,0),
                Vec3::new(0,0,1),
                box_dim.y,
                box_dim.z)
            },
            BoxFaceType::D => {
                (Vec3::new(box_dim.x - 1,0,0),
                Vec3::new(-1,0,0),
                Vec3::new(0,0,1),
                box_dim.x,
                box_dim.z)
            },
            BoxFaceType::E => {
                (Vec3::new(box_dim.x - 1,box_dim.y - 1,0),
                Vec3::new(0,-1,0),
                Vec3::new(0,0,1),
                box_dim.y,
                box_dim.z)
            },
            BoxFaceType::F => {
                (Vec3::new(0,box_dim.y - 1,box_dim.z - 1),
                Vec3::new(1,0,0),
                Vec3::new(0,-1,0),
                box_dim.x,
                box_dim.y)
            },
        };
        for b in 0..dim_b {
            let mut line:String = "".to_string();
            for a in 0..dim_a {
                let padding:String;
                if a == dim_a - 1 {
                    padding = "".to_string();
                }
                else {
                    padding = " ".to_string();
                }
                let box_pos = corner_vec + (vec_a * a) + (vec_b * b);
                let face_coord = BoxFaceCoord{coord:box_pos, face:self.face};
                let grid_coord = mapping.get(&face_coord).unwrap();
                if !grid.numbers.contains_key(&grid_coord) {
                    line.push_str(&format!(".{}",padding));
                }
                else {
                    let number = grid.numbers.get(&grid_coord).unwrap();
                    if circles_only {
                        if number.is_circle {
                            line.push_str(&format!("{}{}", number.val,padding));
                        }
                        else {
                            line.push_str(&format!(".{}",padding));
                        }
                    }
                    else if squares_only {
                        if number.is_square {
                            line.push_str(&format!("{}{}", number.val,padding));
                        }
                        else {
                            line.push_str(&format!(".{}",padding));
                        }
                    }
                    else {
                        line.push_str(&format!("{}{}", number.val,padding));
                    }
                }
            }
            lines.push(line);
        }
        return lines;
    }
}

/// Structure for storing a box of given dimensions
pub struct BoxSolid {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub faces:HashMap<BoxFaceType,BoxFace>
}
impl BoxSolid {
    pub fn new(x:usize, y:usize, z:usize) -> BoxSolid {
        let mut faces:HashMap<BoxFaceType, BoxFace> = HashMap::new();
        faces.insert(BoxFaceType::A, BoxFace{ face:BoxFaceType::A, 
                            vec_a:Vec3::new(1,0,0), 
                            vec_b:Vec3::new(0,1,0), 
                            vec_normal:Vec3::new(0,0,-1)});
        faces.insert(BoxFaceType::B, BoxFace{ face:BoxFaceType::B, 
                            vec_a:Vec3::new(1,0,0), 
                            vec_b:Vec3::new(0,0,1), 
                            vec_normal:Vec3::new(0,1,0)});
        faces.insert(BoxFaceType::C, BoxFace{ face:BoxFaceType::C, 
                            vec_a:Vec3::new(0,1,0), 
                            vec_b:Vec3::new(0,0,1), 
                            vec_normal:Vec3::new(-1,0,0)});
        faces.insert(BoxFaceType::D, BoxFace{ face:BoxFaceType::D, 
                            vec_a:Vec3::new(1,0,0), 
                            vec_b:Vec3::new(0,0,1), 
                            vec_normal:Vec3::new(0,-1,0)});
        faces.insert(BoxFaceType::E, BoxFace{ face:BoxFaceType::E, 
                            vec_a:Vec3::new(0,1,0), 
                            vec_b:Vec3::new(0,0,1), 
                            vec_normal:Vec3::new(1,0,0)});
        faces.insert(BoxFaceType::F, BoxFace{ face:BoxFaceType::F, 
                            vec_a:Vec3::new(1,0,0), 
                            vec_b:Vec3::new(0,1,0), 
                            vec_normal:Vec3::new(0,0,1)});
        return BoxSolid{x:x,y:y,z:z,faces:faces};
    }
    /// Print an unfolded diagram of the box with the given `mapping` from the `grid` 
    pub fn print(&self, mapping:&HashMap<BoxFaceCoord, Vec2>, grid:&BoxGrid, include_circles:bool, include_squares:bool){
        let corner = "+";
        let horizontal_separator = "|";
        let vertical_separator = "-";
        //   |A|
        // |C|B|E|D|
        //   |F|
        let mut horizontal_indent = "".to_string();
        let mut x_vertical_separator = "".to_string();
        let mut y_vertical_separator = "".to_string();
        for _ in 0..2*self.y {
            horizontal_indent.push_str(" ");
        }
        for _ in 0..2*self.x - 1 {
            x_vertical_separator.push_str(&format!("{}", vertical_separator));
        }
        for _ in 0..2*self.y - 1{
            y_vertical_separator.push_str(&format!("{}", vertical_separator));
        }
        let dimensions = Vec3::new(self.x as isize, self.y as isize, self.z as isize);
        // Block 1
        let a_lines = self.faces.get(&BoxFaceType::A).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        println!("{}{}{}{}", horizontal_indent, corner, x_vertical_separator, corner);
        for line in a_lines {
            println!("{}{}{}{}",horizontal_indent,horizontal_separator,line,horizontal_separator);
        }
        // Block 2
        let c_lines = self.faces.get(&BoxFaceType::C).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        let b_lines = self.faces.get(&BoxFaceType::B).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        let e_lines = self.faces.get(&BoxFaceType::E).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        let d_lines = self.faces.get(&BoxFaceType::D).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        println!("{}{}{}{}{}{}{}{}{}", corner, y_vertical_separator, corner, x_vertical_separator, corner, y_vertical_separator, corner, x_vertical_separator, corner);
        for i in 0..c_lines.len() {
            println!("{}{}{}{}{}{}{}{}{}", horizontal_separator, c_lines[i], horizontal_separator, b_lines[i], horizontal_separator, e_lines[i], horizontal_separator, d_lines[i], horizontal_separator);
        }
        println!("{}{}{}{}{}{}{}{}{}", corner, y_vertical_separator, corner, x_vertical_separator, corner, y_vertical_separator, corner, x_vertical_separator, corner);
        // Block 3
        let f_lines = self.faces.get(&BoxFaceType::F).unwrap().to_lines(dimensions, mapping, grid, include_circles, include_squares);
        for line in f_lines {
            println!("{}{}{}{}",horizontal_indent,horizontal_separator,line,horizontal_separator);
        }
        println!("{}{}{}{}", horizontal_indent, corner, x_vertical_separator, corner);
    }
    /// Given a move on the grid from a previously mapped position on the face 
    /// of the box, return the new position on the box and the potential 
    /// rotation of the mapping of the grid coordinate system to box space. 
    pub fn traverse(&self, dir:usize, start_face_coord:BoxFaceCoord, start_face_vec_a:Vec3, start_face_vec_b:Vec3) -> Result<(BoxFaceCoord, Vec3, Vec3), String> {
        let next_face_position:Vec3 = start_face_coord.coord + (start_face_vec_a  * ARROW_DELTAS[dir].x) + (start_face_vec_b  * ARROW_DELTAS[dir].y);
        // If a new face has not been reached
        if next_face_position.in_bounds(self.x, self.y, self.z) {
            return Ok((BoxFaceCoord{coord:next_face_position,face:start_face_coord.face}, start_face_vec_a, start_face_vec_b));
        }
        // If a new face has been reached, the coordinate remains the same but 
        // the face changes and vec_a and vec_b must be rotated relative to the 
        // change in new face
        let next_coord = start_face_coord.coord;
        let next_normal:Vec3 = Vec3::new(
            if next_face_position.x < 0 {-1} else if next_face_position.x >= self.x as isize {1} else {0},
            if next_face_position.y < 0 {-1} else if next_face_position.y >= self.y as isize {1} else {0},
            if next_face_position.z < 0 {-1} else if next_face_position.z >= self.z as isize {1} else {0}
        );
        let next_rotation = self.faces.get(&start_face_coord.face).unwrap().vec_normal.rotation_to(next_normal);
        let next_face_vec_a:Vec3;
        let next_face_vec_b:Vec3;
        match next_rotation {
            Some((axis, direction)) => {
                next_face_vec_a = start_face_vec_a.rot(axis, direction);
                next_face_vec_b = start_face_vec_b.rot(axis, direction);
            },
            None => {
                return Err(format!("Next rotation between normals {} and {} failed for start face coordinate {} dir {}", self.faces.get(&start_face_coord.face).unwrap().vec_normal, next_normal, start_face_coord, dir));
            }
        }
        let next_face:BoxFaceType;
        if next_face_position.x < 0 { next_face = BoxFaceType::C; }
        else if next_face_position.x >= self.x as isize { next_face = BoxFaceType::E; }
        else if next_face_position.y < 0 { next_face = BoxFaceType::D; }
        else if next_face_position.y >= self.y as isize { next_face = BoxFaceType::B; }
        else if next_face_position.z < 0 { next_face = BoxFaceType::A; }
        else if next_face_position.z >= self.z as isize { next_face = BoxFaceType::F; }
        else { return Err(format!("Unable to determine next face from position {}", next_face_position)); }
        return Ok((BoxFaceCoord{coord:next_coord, face:next_face}, next_face_vec_a, next_face_vec_b));
    }
}
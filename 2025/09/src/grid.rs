use std::collections::{HashMap,HashSet};
use std::hash::Hash;
use std::hash::Hasher;
use std::thread;
use std::sync::{Arc,Mutex};

pub const GRID_DIM:usize = 9;
pub const OCCUPIED_CELLS:usize = 45;

const ONE_POS:Coord = Coord{x:4,y:4};
const FOUR_POS:Coord = Coord{x:3,y:1};
const FIVE_POS:Coord = Coord{x:4,y:0};
const EIGHT_POS:Coord = Coord{x:5,y:7};
const NINE_POS:Coord = Coord{x:4,y:8};

const THREE_COL_LO:isize = 2;
const SEVEN_COL_HI:isize = 6;
const SIX_ROW_LEFT:isize = 3;
const TWO_ROW_RIGHT:isize = 5;
const I_ROW_LEFT:isize = 0;
const U_ROW_RIGHT:isize = 0;
const X_ROW_RIGHT:isize = 3;
const N_ROW_LEFT:isize = 5;
const Z_ROW_LEFT:isize = 8;
const V_ROW_RIGHT:isize = 8;

// Since the P shape contains a fully occupied 2x2 block, it can't be placed on the grid and is omitted
const F_BASE: &str = ".##\n##.\n.#.";
const I_BASE: &str = "#####";
const L_BASE: &str = "#.\n#.\n#.\n##";
const N_BASE: &str = "##..\n.###";
const T_BASE: &str = "###\n.#.\n.#.";
const U_BASE: &str = "#.#\n###";
const V_BASE: &str = "#..\n#..\n###";
const W_BASE: &str = "#..\n##.\n.##";
const X_BASE: &str = ".#.\n###\n.#.";
const Y_BASE: &str = "..#.\n####";
const Z_BASE: &str = "..#\n###\n#..";

/// 2D coordinate vector
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord {
    pub x:isize,
    pub y:isize
}
impl Coord {
    pub fn new(x:isize, y:isize) -> Coord {
        return Coord{x:x, y:y};
    }
}
impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
		write!(f,"({},{})", self.x, self.y)
    }
}
impl std::ops::Add<&Coord> for &Coord {
    type Output = Coord;
    fn add(self, rhs:&Coord) -> Coord {
        return Coord{x:self.x + rhs.x, y:self.y + rhs.y};
    }
}

/// Hook
#[derive(Copy, Clone)]
pub enum HookOrientation {
	NW,
	SW,
	SE,
	NE
}
#[derive(Copy, Clone)]
pub struct Hook {
	pub dim:usize,
    pub position:Coord,
    pub orientation: HookOrientation
}
impl Hook {
	pub fn new(dim:usize, position:Coord, orientation:HookOrientation) -> Hook {
		return Hook{dim:dim, position: position, orientation: orientation};
	}
	pub fn num_cells(&self) -> usize {
		return self.dim + (self.dim - 1);
	}
	pub fn cells(&self) -> HashSet<Coord> {
		let mut cells:HashSet<Coord> = HashSet::new();
		for i in 0..self.dim {
			match self.orientation {
				HookOrientation::NE => {
					cells.insert(Coord::new(self.position.x - (i as isize), self.position.y));
					cells.insert(Coord::new(self.position.x, self.position.y + (i as isize)));
				},
				HookOrientation::SE => {
					cells.insert(Coord::new(self.position.x - (i as isize), self.position.y));
					cells.insert(Coord::new(self.position.x, self.position.y - (i as isize)));
				},
				HookOrientation::SW => {
					cells.insert(Coord::new(self.position.x + (i as isize), self.position.y));
					cells.insert(Coord::new(self.position.x, self.position.y - (i as isize)));
				},
				HookOrientation::NW => {
					cells.insert(Coord::new(self.position.x + (i as isize), self.position.y));
					cells.insert(Coord::new(self.position.x, self.position.y + (i as isize)));
				},
			}
		}
		return cells;
	}
}

pub fn hook_print(hooks:&Vec<Hook>) {
    for y in 0..GRID_DIM {
        for x in 0..GRID_DIM {
            let coord = Coord::new(x as isize, y as isize);
            let mut found = false;
            for i in 0..hooks.len() {
                if hooks[i].cells().contains(&coord) {
                    print!("{} ", hooks[i].dim);
                    found = true;
                    break;
                }
                
            }
            if !found {
                print!(". ");
            }
        }
        println!();
    }
}

// Check hook arrangement against constraints on specified 1 and 2 cells
pub fn hook_check(hooks:&Vec<Hook>) -> bool {
	let verbose = false;
	for i in 0..hooks.len() {
		let cells = hooks[i].cells();
		for cell in cells.iter() {
			if cell.x < 0 || cell.y < 0 || cell.x >= (GRID_DIM as isize) || cell.y >= (GRID_DIM as isize){
				if verbose { println!("Cells of hook {} outside bounds", hooks[i].dim); }
				return false;
			}
		}
		// The number 2 must be assigned to the 2-hook (no other numbers work in that position,
		// so the 2-hook must overlap the 2 row
		if hooks[i].dim == 2 {
			let mut row_found = false;
			for cell in cells {
				if cell.y == TWO_ROW_RIGHT {
					row_found = true;
					break;
				}
			}
			if !row_found {
				if verbose { println!("Hook 2 does not overlap 2-row {}", TWO_ROW_RIGHT); }
				return false;
			}
		}
		// Check that the 1-hook is placed on the 1 position
		else if hooks[i].dim == 1 {
			if !cells.contains(&ONE_POS) {
				if verbose { println!("Hook 1 is not positioned on the 1-cell {:?}", ONE_POS); }
				return false;
			}
		}
	}
	return true;
}

/// DFS to find valid hook arrangements
pub fn hook_recurse(dim:usize, assigned:&mut Vec<Hook>, remaining_corner:Coord, solutions:&mut Vec<Vec<Hook>>) {
	if dim == 1 {
		assigned.push(Hook::new(dim, Coord::new(remaining_corner.x, remaining_corner.y), HookOrientation::NE)); // orientation doesn't matter for the 1-hook
		if hook_check(assigned) {
			let mut solved:Vec<Hook> = Vec::new();
			for i in 0..assigned.len() {
				solved.push(assigned[i].clone());
			}
			solutions.push(solved);
		}
		assigned.pop();
		return
	}
	for orientation in vec![HookOrientation::NE, HookOrientation::NW, HookOrientation::SE, HookOrientation::SW] {
		let next_hook:Hook;
		let next_corner:Coord;
		match orientation {
			HookOrientation::NE => {
				next_hook = Hook::new(dim, Coord::new(remaining_corner.x + (dim as isize) - 1, remaining_corner.y), orientation);
				next_corner = Coord::new(remaining_corner.x, remaining_corner.y + 1);
			},
			HookOrientation::SE => {
				next_hook = Hook::new(dim, Coord::new(remaining_corner.x + (dim as isize) - 1, remaining_corner.y + (dim as isize) - 1), orientation);
				next_corner = Coord::new(remaining_corner.x, remaining_corner.y);
			},
			HookOrientation::SW => {
				next_hook = Hook::new(dim, Coord::new(remaining_corner.x, remaining_corner.y + (dim as isize) - 1), orientation);
				next_corner = Coord::new(remaining_corner.x + 1, remaining_corner.y);
			},
			HookOrientation::NW => {
				next_hook = Hook::new(dim, Coord::new(remaining_corner.x, remaining_corner.y), orientation);
				next_corner = Coord::new(remaining_corner.x + 1, remaining_corner.y + 1);
			}
		}
		assigned.push(next_hook);
		if hook_check(assigned) {
			hook_recurse(dim - 1, assigned, next_corner, solutions);
		}
		assigned.pop();
	}
}

/// DFS to find valid assignments of numbers to hook arrangements
pub fn hook_number_assignment_recurse(hook_arrangement:&Vec<Hook>, number_assignments:&mut Vec<usize>, index:usize, solutions:&mut Vec<Vec<usize>>) {
	let verbose = false;
	// initialize hooks that correspond to known positions
	if index == 0 {
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].cells().contains(&NINE_POS) {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 9;
				break;
			}
		}
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].cells().contains(&EIGHT_POS) {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 8;
				break;
			}
		}
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].cells().contains(&FIVE_POS) {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 5;
				break;
			}
		}
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].cells().contains(&FOUR_POS) {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 4;
				break;
			}
		}
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].cells().contains(&ONE_POS) {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 1;
				break;
			}
		}
		// 2 must be assigned to the 2-hook because while 3 will 
		// technically fit, it would violate the 2x2 rule
		for i in 0..hook_arrangement.len() {
			if hook_arrangement[i].dim == 2 {
				if number_assignments[i] != 0 {return;}
				number_assignments[i] = 2;
				break;
			}
		}
	}
	// A complete assignment has been reached - check that specified rows and 
	// columns contain at least once hook cell with the required digit
	else if index == hook_arrangement.len() {
		let six_cells = hook_arrangement[number_assignments.iter().position(|&r| r == 6).unwrap()].cells();
		let mut six_found = false;
		for x in 0..GRID_DIM {
			if six_cells.contains(&Coord::new(x as isize, SIX_ROW_LEFT as isize)) {
				six_found = true;
				break;
			}
		}
		if !six_found {
			if verbose { println!("SIX ROW {} not in 6-assigned hook cells {:?}", SIX_ROW_LEFT, six_cells); }
			return;
		}
		let two_cells = hook_arrangement[number_assignments.iter().position(|&r| r == 2).unwrap()].cells();
		let mut two_found = false;
		for x in 0..GRID_DIM {
			if two_cells.contains(&Coord::new(x as isize, TWO_ROW_RIGHT as isize)) {
				two_found = true;
				break;
			}
		}
		if !two_found {
			if verbose { println!("TWO ROW {} not in 2-assigned hook cells {:?}", TWO_ROW_RIGHT, two_cells); }
			return;
		}
		let three_cells = hook_arrangement[number_assignments.iter().position(|&r| r == 3).unwrap()].cells();
		let mut three_found = false;
		for y in 0..GRID_DIM {
			if three_cells.contains(&Coord::new(THREE_COL_LO as isize, y as isize)) {
				three_found = true;
				break;
			}
		}
		if !three_found {
			if verbose { println!("THREE COL {} not in 3-assigned hook cells {:?}", THREE_COL_LO, three_cells); }
			return;
		}
		let seven_cells = hook_arrangement[number_assignments.iter().position(|&r| r == 7).unwrap()].cells();
		let mut seven_found = false;
		for y in 0..GRID_DIM {
			if seven_cells.contains(&Coord::new(SEVEN_COL_HI as isize, y as isize)) {
				seven_found = true;
				break;
			}
		}
		if !seven_found {
			if verbose { println!("SEVEN COL {} not in 7-assigned hook cells {:?}", SEVEN_COL_HI, seven_cells); }
			return;
		}
		// Hook/number combination okay
		solutions.push(number_assignments.clone());
		return;
	}
    // Continue if this hook as already been assigned a number
	if number_assignments[index] != 0 {
		hook_number_assignment_recurse(hook_arrangement, number_assignments, index + 1, solutions);
	}
    // Try all available numbers on the current hook
	else {
		let mut remaining_numbers:Vec<usize> = Vec::new();
		for n in 1..=GRID_DIM {
			if !number_assignments.contains(&n) {
				remaining_numbers.push(n);
			}
		}
		for n in remaining_numbers {
			if n > hook_arrangement[index].num_cells() {
				continue;
			}
			let mut next_number_assignments = number_assignments.clone();
			next_number_assignments[index] = n;
			hook_number_assignment_recurse(hook_arrangement, &mut next_number_assignments, index + 1, solutions);
		}
	}
}

/// Pentomino
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pentomino {
	pub dim_x: usize,
	pub dim_y: usize,
	pub cells: HashSet<Coord>,
	pub position: Coord,
	pub class:String
}
impl Hash for Pentomino {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let mut bits:usize = 0;
		let normalized = self.clone().normalize();
		for y in 0..5 {
			for x in 0..5 {
				if normalized.cells.contains(&Coord::new(x,y)) {
					bits |= 1 << (x + 5*y);
				}
			}
		}
		bits.hash(state);
	}
}
impl Pentomino {
	pub fn new(input_string: &str, class:String) -> Pentomino {
		let mut min_x:isize = -1;
		let mut min_y:isize = -1;
		let mut max_x:isize = -1;
		let mut max_y:isize = -1;
		let mut cells:HashSet<Coord> = HashSet::new();
		let lines:Vec<&str> = input_string.split("\n").collect();
		assert!(lines.len() > 0);
		for y in 0..lines.len() {
			let chars:Vec<char> = lines[y].chars().collect();
			for x in 0..chars.len() {
				if chars[x] == '#' {
					cells.insert(Coord::new(x as isize,y as isize));
					if (x as isize) < min_x || min_x == -1 {
						min_x = x as isize ;
					}
					if (x as isize) > max_x || min_x == -1 {
						max_x = x as isize ;
					}
					if (y as isize) < min_y || min_y == -1 {
						min_y = y as isize ;
					}
					if (x as isize) > max_y || min_y == -1 {
						max_y = y as isize ;
					}
				}
			}
		}
		assert!(max_x - min_x + 1 > 0);
		assert!(max_y - min_y + 1 > 0);
		let mut pentomino = Pentomino{ dim_x: 0, dim_y: 0, cells: cells, position: Coord::new(0,0), class: class };
		pentomino.normalize();
		return pentomino;
	}
    /// Copy the pentomino with an offset position
	pub fn copy_with_position(&self, position:Coord) -> Pentomino {
		return Pentomino{dim_x:self.dim_x, dim_y:self.dim_y, cells:self.cells.clone(), position:position, class:self.class.clone()};
	}
	/// Normalize coordinates of the pentomino cells so that the bounding box
	/// has an upper-left coordinate of (0,0)
	pub fn normalize(&mut self) {
		let mut next_cells:HashSet<Coord> = HashSet::new();
		let mut min_x = 5;
		let mut min_y = 5;
		let mut max_x = -5;
		let mut max_y = -5;
		for cell in self.cells.iter() {
			if cell.x < min_x {
				min_x = cell.x;
			}
			if cell.y < min_y || min_y == -1 {
				min_y = cell.y;
			}
			if cell.x > max_x || max_x == -1 {
				max_x = cell.x;
			}
			if cell.y > max_y || max_y == -1 {
				max_y = cell.y;
			}
		}
		for cell in self.cells.drain() {
			next_cells.insert(Coord::new(cell.x - min_x, cell.y - min_y));
		}
		self.position = Coord::new(0,0);
		self.cells = next_cells;
		self.dim_x = (max_x - min_x + 1) as usize;
		self.dim_y = (max_y - min_y + 1) as usize;
	}
	/// Rotate cells of self by 90 degrees counterclockwise about the origin
	pub fn rotate(&mut self) {
		let mut next_cells:HashSet<Coord> = HashSet::new();
		for cell in self.cells.drain() {
			next_cells.insert(Coord::new(-cell.y, cell.x));
		}
		self.cells = next_cells;
		self.normalize();
	}
	/// Reflect cells of self across the y-axis
	pub fn reflect(&mut self) {
		let mut next_cells:HashSet<Coord> = HashSet::new();
		for cell in self.cells.drain() {
			next_cells.insert(Coord::new(-cell.x, cell.y));
		}
		self.cells = next_cells;
		self.normalize();
	}
	/// Get cells offset by position
	pub fn get_cells_with_offset(&self) -> HashSet<Coord> {
		let mut offset_cells:HashSet<Coord> = HashSet::new();
		for coord in self.cells.iter() {
			offset_cells.insert(Coord{x: coord.x + self.position.x, y: coord.y + self.position.y});
		}
		return offset_cells;
	}
}

/// Initialize a map of string identifiers to sets of all distinct members of 
/// the pentomino types isomorphic up to rotation and reflection.
pub fn init_pentominos() -> HashMap<&'static str, HashSet<Pentomino>> {
	let mut pentomino_map:HashMap<&str, HashSet<Pentomino>> = HashMap::new();
    // Note that the P shape is omitted due to including a 2x2 block.
	let pentomino_shapes:HashMap<&str, &str> = HashMap::from([
		("F", F_BASE), 
		("I", I_BASE), 
		("L", L_BASE), 
		("N", N_BASE), 
		("T", T_BASE), 
		("U", U_BASE), 
		("V", V_BASE),
		("W", W_BASE), 
		("X", X_BASE),
		("Y", Y_BASE),
		("Z", Z_BASE)]);
	for key in pentomino_shapes.keys() {
		let mut set:HashSet<Pentomino> = HashSet::new();
		let base_pentomino = Pentomino::new(pentomino_shapes.get(key).unwrap(), key.to_string());
		for rotations in 0..4 {
			for reflections in 0..2 {
				let mut pentomino = base_pentomino.clone();
				for _ in 0..rotations {
					pentomino.rotate();
				}
				for _ in 0..reflections {
					pentomino.reflect();
				}
				if !set.contains(&pentomino) {
					set.insert(pentomino);
				}
			}
		}
		pentomino_map.insert(key, set);
	}
	return pentomino_map;
}

pub fn pentominos_print(pentominos:&Vec<Pentomino>) {
    for y in 0..GRID_DIM {
        for x in 0..GRID_DIM {
            let coord = Coord::new(x as isize, y as isize);
            let mut found = false;
            for i in 0..pentominos.len() {
                if pentominos[i].get_cells_with_offset().contains(&coord) {
                    print!("{} ", pentominos[i].class);        
                    found = true;
                    break;
                }
            }
            if !found {
                print!(". ");
            }
        }
        println!();
    }
}

/// Find all empty regions on a grid with given pentomino positions and return
/// the product of their areas
pub fn pentominos_empty_cell_product(pentominos:&Vec<Pentomino>) -> usize {
    let mut grid:HashSet<Coord> = HashSet::new();
    let mut seen:HashSet<Coord> = HashSet::new();
    let mut area_product = 1;
    let dirs = vec![Coord::new(1,0), Coord::new(0,1), Coord::new(-1, 0), Coord::new(0, -1)];	
    for y in 0..GRID_DIM {
        for x in 0..GRID_DIM {
            let coord = Coord::new(x as isize, y as isize);
            for i in 0..pentominos.len() {
                if pentominos[i].get_cells_with_offset().contains(&coord) {
                    grid.insert(coord);
                    break;
                }
            }
        }
    }
    for y in 0..GRID_DIM {
        for x in 0..GRID_DIM {
            let coord = Coord::new(x as isize, y as isize);
            if grid.contains(&coord) || seen.contains(&coord) {
                continue;
            }
            // Flood fill
            let mut frontier:Vec<Coord>;
            let mut frontier_next:Vec<Coord> = vec![coord];
            let mut members:HashSet<Coord> = HashSet::new();
            while frontier_next.len() > 0 {
                frontier = frontier_next;
                frontier_next = Vec::new();
                while frontier.len() > 0 {
                    let member = frontier.pop().unwrap();
                    if members.contains(&member) {
                        continue;
                    }
                    members.insert(member);
                    for delta in dirs.iter() {
                        let neighbor = &member + delta;
                        if neighbor.x < 0 || neighbor.y < 0 || neighbor.x >= GRID_DIM as isize || neighbor.y >= GRID_DIM as isize || grid.contains(&neighbor) || members.contains(&neighbor) {
                            continue;
                        }
                        frontier_next.push(neighbor);
                    }
                }
            }
            if members.len() > 0 {
                area_product *= members.len();
                for member in members.iter() {
                    seen.insert(*member);
                }
            }
        }
    }
    return area_product;
}

/// DFS of pentomino arrangements.
pub fn pentomino_recurse(pentominos_map:&HashMap<&'static str, HashSet<Pentomino>>, assigned_pentominos:&mut Vec<Pentomino>, hook_number_assignments:&Vec<(Vec<Hook>, Vec<usize>)>, hook_number_index:usize, solutions:&mut HashSet<Vec<Pentomino>>) {
	// Remaining pentomino classes after I,U,Z,V,N,X have been assigned
    let remaining_pentomino_key_order: Vec<&str> = vec!["F", "L", "T", "W", "Y"];
	// Assign the pentominos specified in row indicators first, in order: I, U, Z, V, N, X
	// I
    if assigned_pentominos.len() == 0 {
        for i_pentomino in pentominos_map.get("I").unwrap() {
            // must touch top row
            let y_offset = 0;
            for x in 0..GRID_DIM {
                assigned_pentominos.push(i_pentomino.copy_with_position(Coord::new(x as isize,y_offset)));
                if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
                    pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
				}
                assigned_pentominos.pop();
			}
		}
	}
	// U
    else if assigned_pentominos.len() == 1 {
        for u_pentomino in pentominos_map.get("U").unwrap() {
            // must touch top row
            let y_offset = 0;
            for x in 0..GRID_DIM {
                assigned_pentominos.push(u_pentomino.copy_with_position(Coord::new(x as isize,y_offset as isize)));
                if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
                    pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
				}
                assigned_pentominos.pop();
			}
		}
	}
	// Z
    else if assigned_pentominos.len() == 2 {
        for z_pentomino in pentominos_map.get("Z").unwrap() {
            // must touch bottom row
            let y_offset = GRID_DIM - z_pentomino.dim_y;
            for x in 0..GRID_DIM {
                assigned_pentominos.push(z_pentomino.copy_with_position(Coord::new(x as isize,y_offset as isize)));
                if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
                    pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
				}
                assigned_pentominos.pop();
			}
		}
	}
	// V
    else if assigned_pentominos.len() == 3 {
        for v_pentomino in pentominos_map.get("V").unwrap() {
            // must touch bottom row
            let y_offset = GRID_DIM - v_pentomino.dim_y;
            for x in 0..GRID_DIM {
                assigned_pentominos.push(v_pentomino.copy_with_position(Coord::new(x as isize,y_offset as isize)));
                if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
                    pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
				}
                assigned_pentominos.pop();
			}
		}
	}
	// N
    else if assigned_pentominos.len() == 4 {
        for n_pentomino in pentominos_map.get("N").unwrap() {
            // must touch N row
			for y_offset in N_ROW_LEFT - (n_pentomino.dim_y as isize) + 1..=N_ROW_LEFT {
            	for x in 0..GRID_DIM {
					assigned_pentominos.push(n_pentomino.copy_with_position(Coord::new(x as isize,y_offset as isize)));
					if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
						pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
					}
					assigned_pentominos.pop();
				}
			}
		}
	}
	// X
    else if assigned_pentominos.len() == 5 {
        for x_pentomino in pentominos_map.get("X").unwrap() {
            // must touch X row
			for y_offset in X_ROW_RIGHT - (x_pentomino.dim_y as isize) + 1..=X_ROW_RIGHT {
            	for x in 0..GRID_DIM {
					assigned_pentominos.push(x_pentomino.copy_with_position(Coord::new(x as isize,y_offset as isize)));
					if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
						pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
					}
					assigned_pentominos.pop();
				}
			}
		}
	}
	// Remaining pentominos
	else if assigned_pentominos.len() < GRID_DIM {
		let mut remaining_types:Vec<String> = Vec::new();
		// Find the highest index number of any assigned pentomino in the 
		// remaining_pentomino_key_order list
		let mut remaining_start_index = 0;
		for i in 0..assigned_pentominos.len() {
			for j in 0..remaining_pentomino_key_order.len() {
				if assigned_pentominos[i].class == remaining_pentomino_key_order[j].to_string() {
					if j + 1 > remaining_start_index {
						remaining_start_index = j+1;
					}
					break;
				}
			}
		}
		for i in remaining_start_index..remaining_pentomino_key_order.len() {
			let type_key = remaining_pentomino_key_order[i];
			let mut found = false;
			for i in 0..assigned_pentominos.len() {
				if assigned_pentominos[i].class == type_key.to_string() {
					found = true;
					break;
				}
			}
			if !found {
				remaining_types.push(type_key.to_string());
			}
		}
		for type_key in remaining_types {
			for pentomino in pentominos_map.get(type_key.as_str()).unwrap() {
				for y in 0..GRID_DIM {
					for x in 0..GRID_DIM {
						assigned_pentominos.push(pentomino.copy_with_position(Coord::new(x as isize,y as isize)));
						if pentomino_validate(assigned_pentominos, hook_number_assignments, hook_number_index) {
							pentomino_recurse(pentominos_map, assigned_pentominos, hook_number_assignments, hook_number_index, solutions);
						}
						assigned_pentominos.pop();
					}
				}
			}
		}
	}
	// 9 pentominos assigned
	else {
		solutions.insert(assigned_pentominos.clone());
	}
}

/// Test conditions of full and partial pentomino assigments against a given 
/// hook/number combination
pub fn pentomino_validate(assigned_pentominos:&Vec<Pentomino>, hook_number_assignments:&Vec<(Vec<Hook>, Vec<usize>)>, hook_number_index:usize) -> bool {
	let verbose = false;
	// Check if any pentominos are off the grid 
	for i in 0..assigned_pentominos.len() {
		let cells = assigned_pentominos[i].get_cells_with_offset();
		for cell in cells {
			if cell.x < 0 || cell.x >= GRID_DIM as isize || cell.y < 0 || cell.y >= GRID_DIM as isize {
				if verbose { println!("Pentomino {} has cell off the grid with coord {:?}", assigned_pentominos[i].class, cell); }
				return false;
			}
		}
	}
	// Check sums for each assigned pentomino given the specified hook/number 
	// combination
	for pentomino in assigned_pentominos.iter() {
        let cells = pentomino.get_cells_with_offset();
        let mut total = 0;
        for coord in cells {
			for j in 0..hook_number_assignments[hook_number_index].1.len() {
				let hook_cells = hook_number_assignments[hook_number_index].0[j].cells();
				if hook_cells.contains(&coord) {
					total += hook_number_assignments[hook_number_index].1[j];
					break;
				}
			}
		}
        if total % 5 != 0 {
			if verbose { println!("Numeric total for pentomino {:?} is {}", pentomino, total); }
            return false;
		}
	}
    
	// Produce the grid for pentomino positions and number positions, ensure 
	// no pentomino positions collide, and check that no digit appears more 
	// than the required number of times. 
    let mut number_totals = vec![0;10];
	let mut grid_pentomino_class:HashMap<Coord, String> = HashMap::new();
	let mut grid_number:HashMap<Coord, usize> = HashMap::new();
	for y in 0..GRID_DIM {
		for x in 0..GRID_DIM {
			let coord = Coord::new(x as isize,y as isize);
			for i in 0..assigned_pentominos.len() {
				if assigned_pentominos[i].get_cells_with_offset().contains(&coord) {
					if grid_pentomino_class.contains_key(&coord) {
						if verbose {println!("Pentomino collision at position {:?}: Trying to assign {}, {} already assigned", coord, assigned_pentominos[i].class, grid_pentomino_class.get(&coord).unwrap());}
						return false;
					}
					grid_pentomino_class.insert(coord.clone(), assigned_pentominos[i].class.clone());
					// find the corresponding hook, then insert the assigned 
					// number for that hook
					for j in 0..hook_number_assignments[hook_number_index].0.len() {
						if hook_number_assignments[hook_number_index].0[j].cells().contains(&coord) {
							grid_number.insert(coord.clone(), hook_number_assignments[hook_number_index].1[j]);
                            number_totals[hook_number_assignments[hook_number_index].1[j]] += 1;
                        }
					}
				}
			}
		}
	}
    for i in 0..number_totals.len() {
        if number_totals[i] > i {
            if verbose {println!("Number {} appears more than {} times ({}).", i, i, number_totals[i]);}
            return false;
        }
    }
	let mut current_pentomino_classes:Vec<String> = Vec::new();
	for i in 0..assigned_pentominos.len() {
		if current_pentomino_classes.contains(&assigned_pentominos[i].class) {
			if verbose { println!("Multiple pentominos of class {} found in assignment", assigned_pentominos[i].class); }
			return false;
		}
		current_pentomino_classes.push(assigned_pentominos[i].class.clone());
	}
	if current_pentomino_classes.contains(&"I".to_string()) {
		// From the left
		let mut found = false;
		for x in 0..GRID_DIM {
			let coord = Coord::new(x as isize, I_ROW_LEFT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "I".to_string() {
					if verbose {println!("Found wrong pentomino first in I row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("I coordinate not found in expected row");}
			return false;
		}
	}
	if current_pentomino_classes.contains(&"U".to_string()) {
		// From the right
		let mut found = false;
		for x in (0..GRID_DIM).rev() {
			let coord = Coord::new(x as isize, U_ROW_RIGHT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "U".to_string() {
					if verbose {println!("Found wrong pentomino first in U row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("U coordinate not found in expected row");}
			return false;
		}
	}
	if current_pentomino_classes.contains(&"X".to_string()) {
		// From the right
		let mut found = false;
		for x in (0..GRID_DIM).rev() {
			let coord = Coord::new(x as isize, X_ROW_RIGHT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "X".to_string() {
					if verbose {println!("Found wrong pentomino first in X row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("X coordinate not found in expected row");}
			return false;
		}
	}
	if current_pentomino_classes.contains(&"N".to_string()) {
		// From the left
		let mut found = false;
		for x in 0..GRID_DIM {
			let coord = Coord::new(x as isize, N_ROW_LEFT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "N".to_string() {
					if verbose {println!("Found wrong pentomino first in N row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("N coordinate not found in expected row");}
			return false;
		}
	}
	if current_pentomino_classes.contains(&"Z".to_string()) {
		// From the left
		let mut found = false;
		for x in 0..GRID_DIM {
			let coord = Coord::new(x as isize, Z_ROW_LEFT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "Z".to_string() {
					if verbose {println!("Found wrong pentomino first in Z row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("Z coordinate not found in expected row");}
			return false;
		}
	}
	if current_pentomino_classes.contains(&"V".to_string()) {
		// From the right
		let mut found = false;
		for x in (0..GRID_DIM).rev() {
			let coord = Coord::new(x as isize, V_ROW_RIGHT as isize);
			if grid_pentomino_class.contains_key(&coord) {
				if *grid_pentomino_class.get(&coord).unwrap() != "V".to_string() {
					if verbose {println!("Found wrong pentomino first in V row at coord {:?}: {}", coord, grid_pentomino_class.get(&coord).unwrap());}
					return false;
				}
				else {
					found = true;
					break;
				}
			}
		}
		if !found {
			if verbose {println!("V coordinate not found in expected row");}
			return false;
		}
	}
	// Check the 2x2 rule
	for x in 0..GRID_DIM - 1 {
		for y in 0..GRID_DIM - 1 {
			let mut cell_count = 0;
			for x1 in 0..2 {
				for y1 in 0..2 {
					let offset_coord = Coord::new((x + x1) as isize, (y + y1) as isize);
					if grid_pentomino_class.contains_key(&offset_coord) {
						cell_count += 1;
					}
				}
			}
			if cell_count > 3 {
				if verbose {println!("2x2 rule violated at {},{}", x, y);}
				return false;
			}
		}
	}
	// If all pentominos have been assigned, check all specified number 
	// positions, verify that the cells are all connected, and check that the 
	// numbers within hooks appear at the required number of times
	if assigned_pentominos.len() == GRID_DIM {
		// Check that the number of cells occupied by each digit matches the 
		// expected counts
		for i in 0..number_totals.len() {
            if number_totals[i] != i {
                if verbose {println!("Number {} appears {} times.", i, number_totals[i]);}
                return false;
            }
        }
        // Check specified postions, rows and columns
		if !grid_number.contains_key(&ONE_POS) || *grid_number.get(&ONE_POS).unwrap() != 1 {
			if verbose {println!("ONE_POS incorrect");}
			return false;
		}
		if !grid_number.contains_key(&FOUR_POS) || *grid_number.get(&FOUR_POS).unwrap() != 4 {
			if verbose {println!("FOUR_POS incorrect");}
			return false;
		}
		if !grid_number.contains_key(&FIVE_POS) || *grid_number.get(&FIVE_POS).unwrap() != 5 {
			if verbose {println!("FIVE_POS incorrect");}
			return false;
		}
		if !grid_number.contains_key(&EIGHT_POS) || *grid_number.get(&EIGHT_POS).unwrap() != 8 {
			if verbose {println!("EIGHT_POS incorrect");}
			return false;
		}
		if !grid_number.contains_key(&NINE_POS) || *grid_number.get(&NINE_POS).unwrap() != 9 {
			if verbose {println!("NINE_POS incorrect");}
			return false;
		}
		// 6 from the left
		let mut six_okay = false;
		for x in 0..GRID_DIM {
			let coord = Coord::new(x as isize, SIX_ROW_LEFT);
			if grid_number.contains_key(&coord) {
				if *grid_number.get(&coord).unwrap() != 6 {
					if verbose {println!("SIX_ROW_LEFT found wrong value");}
					return false;
				}
				else {
					six_okay = true;
					break;
				}
			}
		}
		if !six_okay {
			if verbose {println!("SIX_ROW_LEFT found nothing");}
			return false;
		}
		// 2 from the right
		let mut two_okay = false;
		for x in (0..GRID_DIM).rev() {
			let coord = Coord::new(x as isize, TWO_ROW_RIGHT);
			if grid_number.contains_key(&coord) {
				if *grid_number.get(&coord).unwrap() != 2 {
					if verbose {println!("TWO_ROW_RIGHT found wrong value");}
					return false;
				}
				else {
					two_okay = true;
					break;
				}
			}
		}
		if !two_okay {
			if verbose {println!("TWO_ROW_RIGHT found nothing");}
			return false;
		}
		// 7 from the top
		let mut seven_okay = false;
		for y in 0..GRID_DIM {
			let coord = Coord::new(SEVEN_COL_HI, y as isize);
			if grid_number.contains_key(&coord) {
				if *grid_number.get(&coord).unwrap() != 7 {
					if verbose {println!("SEVEN_COL_HI found wrong value");}
					return false;
				}
				else {
					seven_okay = true;
					break;
				}
			}
		}
		if !seven_okay {
			if verbose {println!("SEVEN_COL_HI found nothing");}
			return false;
		}
		// 3 from the bottom
		let mut three_okay = false;
		for y in (0..GRID_DIM).rev() {
			let coord = Coord::new(THREE_COL_LO, y as isize);
			if grid_number.contains_key(&coord) {
				if *grid_number.get(&coord).unwrap() != 3 {
					if verbose {println!("THREE_COL_LO found wrong value");}
					return false;
				}
				else {
					three_okay = true;
					break;
				}
			}
		}
		if !three_okay {
			if verbose {println!("THREE_COL_LO found nothing");}
			return false;
		}
		// Check that all occupied cells are connected
		let mut connected:HashSet<Coord> = HashSet::new();
		let dirs = vec![Coord::new(1,0), Coord::new(0,1), Coord::new(-1, 0), Coord::new(0, -1)];
		let mut frontier:Vec<Coord>;
		let mut frontier_next:Vec<Coord> = Vec::new();
		frontier_next.push(ONE_POS.clone());
		while frontier_next.len() > 0 {
			frontier = frontier_next;
			frontier_next = Vec::new();
			while frontier.len() > 0 {
				let coord = frontier.pop().unwrap();
				if connected.contains(&coord) {
					continue;
				}
				connected.insert(coord);
				for dir in dirs.iter() {
					let neighbor = &coord + dir;
					if !connected.contains(&neighbor) && grid_number.contains_key(&neighbor) {
						frontier.push(neighbor);
					}
				}
			}
		}
		if connected.len() != OCCUPIED_CELLS {
			if verbose {println!("Only {} cells connected to ONE_POS", connected.len());}
			return false;
		}
	}
	return true;
}

/// Perform DFS to find valid pentomino arrangements given a hook/number 
/// combination. Lower branches of the tree are passed to child threads for 
/// evaluation in parallel.
pub fn pentomino_permutations(pentominos_map:&HashMap<&'static str, HashSet<Pentomino>>,  hook_number_assignments:&Vec<(Vec<Hook>, Vec<usize>)>, hook_number_index:usize, threads:usize) -> HashSet<Vec<Pentomino>> {
    let mut pentomino_results:HashSet<Vec<Pentomino>> = HashSet::new();    
    let mut child_results:Vec<HashSet<Vec<Pentomino>>> = Vec::new();
    for _i in 0..threads {
        child_results.push(HashSet::new());
    }
    let mut handles = Vec::new();
    let mut added = 0;
    let results_arc = Arc::new(Mutex::new(child_results));
	let pentominos_map_arc = Arc::new(pentominos_map.clone());
	let hook_number_assignments_arc = Arc::new(hook_number_assignments.clone());

    // Iterate over members/positions of I, U, Z, V, N
	for i_pentomino_base in pentominos_map.get("I").unwrap().iter() {
		for i_x in 0..GRID_DIM {
			let i_pentomino = i_pentomino_base.copy_with_position(Coord::new(i_x as isize, 0));
			for u_pentomino_base in pentominos_map.get("U").unwrap().iter() {
				for u_x in 0..GRID_DIM {
					let u_pentomino = u_pentomino_base.copy_with_position(Coord::new(u_x as isize, 0));
                    if !pentomino_validate(&vec![i_pentomino.clone(), u_pentomino.clone()], hook_number_assignments, hook_number_index) {
                        continue;
                    }
					for z_pentomino_base in pentominos_map.get("Z").unwrap().iter() {
						for z_x in 0..GRID_DIM {
							let z_pentomino = z_pentomino_base.copy_with_position(Coord::new(z_x as isize, (GRID_DIM - z_pentomino_base.dim_y) as isize));
							if !pentomino_validate(&vec![i_pentomino.clone(), u_pentomino.clone(), z_pentomino.clone()], hook_number_assignments, hook_number_index) {
                                continue;
                            }
                            for v_pentomino_base in pentominos_map.get("V").unwrap().iter() {
								for v_x in 0..GRID_DIM {
									let v_pentomino = v_pentomino_base.copy_with_position(Coord::new(v_x as isize, (GRID_DIM - v_pentomino_base.dim_y) as isize));
									if !pentomino_validate(&vec![i_pentomino.clone(), u_pentomino.clone(), z_pentomino.clone(), v_pentomino.clone()], hook_number_assignments, hook_number_index) {
                                        continue;
                                    }
                                    for n_pentomino_base in pentominos_map.get("N").unwrap().iter() {
                                        for n_y in N_ROW_LEFT - (n_pentomino_base.dim_y as isize) + 1..=N_ROW_LEFT {
                                            for n_x in 0..GRID_DIM {
                                                let n_pentomino = n_pentomino_base.copy_with_position(Coord::new(n_x as isize, n_y));
                                                if !pentomino_validate(&vec![i_pentomino.clone(), u_pentomino.clone(), z_pentomino.clone(), v_pentomino.clone(), n_pentomino.clone()], hook_number_assignments, hook_number_index) {
                                                    continue;
                                                }
                                                let pentominos_map_arc_clone = Arc::clone(&pentominos_map_arc);
                                                let hook_number_assignments_arc_clone = Arc::clone(&hook_number_assignments_arc);
                                                let results_arc_clone = Arc::clone(&results_arc);
                                                let i_pentomino_clone = i_pentomino.clone();
                                                let u_pentomino_clone = u_pentomino.clone();
                                                let z_pentomino_clone = z_pentomino.clone();
                                                let v_pentomino_clone = v_pentomino.clone();
                                                let n_pentomino_clone = n_pentomino.clone();
                                                let handle = thread::spawn(move || {
                                                    let mut initial_assignments = vec![i_pentomino_clone, u_pentomino_clone, z_pentomino_clone, v_pentomino_clone, n_pentomino_clone];
                                                    let mut solutions: HashSet<Vec<Pentomino>> = HashSet::new();
                                                    pentomino_recurse(&pentominos_map_arc_clone, &mut initial_assignments, &hook_number_assignments_arc_clone, hook_number_index, &mut solutions);
                                                    let mut c_r = results_arc_clone.lock().unwrap();
                                                    (*c_r)[added] = solutions;
                                                });
                                                handles.push(handle);
                                                added += 1;
                                                if added == threads {
                                                    // Execute the batch
                                                    while handles.len() > 0 {
                                                        let handle2 = handles.remove(0);
                                                        handle2.join().unwrap();
                                                    }
                                                    let batchresult = results_arc.lock().unwrap();
                                                    for i in 0..added {
                                                        for result in batchresult[i].iter() {
                                                            pentomino_results.insert(result.clone());
                                                        }
                                                    }
                                                    added = 0;
                                                }
                                            }
                                        }
                                    }
								}
							}
						}
					}
				}
			}
		}
    }
    // Run any remaining jobs
    if added > 0 {
        while handles.len() > 0 {
            let handle = handles.remove(0);
            handle.join().unwrap();
        }
        let batchresult = results_arc.lock().unwrap();
		for i in 0..added {
			for result in batchresult[i].iter() {
				pentomino_results.insert(result.clone());
			}
		}
    }
	return pentomino_results;
}

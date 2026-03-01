use crate::coord::Coord;
use std::collections::{HashMap, HashSet};

/// Grid dimensions
pub const DIM:isize = 13;
/// Grid dimensions squared
pub const DIM2:usize = 169;
/// Array of directions to orthogonally adjacent cells
pub const NEIGHBOR_DELTAS:[Coord;4] = [Coord{x:0,y:-1},Coord{x:1,y:0},Coord{x:0,y:1},Coord{x:-1,y:0}];
/// Constant size array type used for hashable keys of `Komino` shapes and grid
/// assignments
/// 
pub type KeyArray = [u8;DIM2]; 
/// Struct for storing a K-omino shape along with all distinct rotations and 
/// reflections
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Komino {
    /// The initial arrangement of cells of this `Komino` before any 
    /// reflections or rotations.
    pub base_cells:HashSet<Coord>,
    /// A `KeyArray` hashable key of the base cell arrangement for this 
    /// `Komino`. Used in `KominoStore` lookups.
    pub base_key:KeyArray,
    /// The size of this `Komino`
    pub val:usize,
    /// The list of all distinct cell arrangements of this `Komino` under any 
    /// combination of rotations and reflections.
    pub transforms:Vec<HashSet<Coord>>,
    /// `KeyArray` hashable keys for each distinct transform
    pub transform_keys:HashMap<KeyArray, usize>
}
impl Komino {
    /// Return a new `Komino` with the specified `cells`
    pub fn new(cells:&HashSet<Coord>) -> Komino {
        let val = cells.len();
        let base_cells = Komino::normalize_cells(cells);
        let base_key:KeyArray = shape_key(&base_cells);
        let mut transforms:Vec<HashSet<Coord>> = Vec::new();
        let mut seen_transforms:HashSet<KeyArray> = HashSet::new();
        for rotation in 0..4 {
            for reflection in 0..2 {
                let mut transformed_cells = base_cells.clone();
                if reflection == 0 {
                    transformed_cells = Komino::reflect(&transformed_cells);
                }
                for _ in 0..=rotation {
                    transformed_cells = Komino::rotate(&transformed_cells);
                }
                let transform_key = shape_key(&transformed_cells);
                if seen_transforms.contains(&transform_key) {
                    continue;
                }
                transforms.push(transformed_cells);
                seen_transforms.insert(transform_key);
            }
        }
        let mut transform_keys:HashMap<KeyArray, usize>= HashMap::new();
        for i in 0..transforms.len() {
            transform_keys.insert(shape_key(&transforms[i]), i);
        }
        return Komino { base_cells, base_key, val, transforms, transform_keys };
    }
    /// Translate the given `cells` to have the upper-left corner of the 
    /// bounding rectangle at (0,0)
    fn normalize_cells(cells:&HashSet<Coord>) -> HashSet<Coord> {
        let mut min_x:isize = 50;
        let mut min_y:isize = 50;
        let mut result:HashSet<Coord> = HashSet::new();
        for cell in cells {
            if cell.x < min_x { min_x = cell.x; }
            if cell.y < min_y { min_y = cell.y; }
        }
        for cell in cells {
            let next_cell = Coord::new(cell.x - min_x, cell.y - min_y);
            result.insert(next_cell);
        }
        return result;
    }
    /// Rotate `cells` clockwise 90 degrees about the origin and normalize 
    /// positions
    fn rotate(cells:&HashSet<Coord>) -> HashSet<Coord> {
        let mut result:HashSet<Coord> = HashSet::new();
        for cell in cells {
            let next_cell = Coord::new(cell.x * 0 + cell.y * -1, cell.x * 1 + cell.y * 0);
            result.insert(next_cell);
        }
        return Komino::normalize_cells(&result);
    }
    /// Reflect `cells` about the vertical axis and normalize positions
    fn reflect(cells:&HashSet<Coord>) -> HashSet<Coord> {
        let mut result:HashSet<Coord> = HashSet::new();
        for cell in cells {
            let next_cell = Coord::new(-cell.x, cell.y);
            result.insert(next_cell);
        }
        return Komino::normalize_cells(&result);
    }
    /// Translate `cells` by `offset` spaces horizontally and vertically
    fn translate(cells:&HashSet<Coord>, offset:Coord) -> HashSet<Coord> {
        let mut result:HashSet<Coord> = HashSet::new();
        for cell in cells {
            let next_cell = *cell + offset;
            result.insert(next_cell);
        }
        return result;
    }
    /// Return all distinct `Komino` shapes that can be created by adding one
    /// cell to this `Komino`
    fn next_kominos(&self) -> Vec<Komino> {
        let mut result = Vec::new();
        let mut tried:HashSet<Coord> = HashSet::new();
        for cell in self.base_cells.iter() {
            for d in NEIGHBOR_DELTAS {
                let coord = d + *cell;
                if self.base_cells.contains(&coord) {
                    continue;
                }
                if tried.contains(&coord) {
                    continue;
                }
                tried.insert(coord);
                let mut next_cells = self.base_cells.clone();
                next_cells.insert(coord);
                result.push(Komino::new(&next_cells));
            }
        }
        return result;
    }
    /// Determine all possible positions/rotations/reflections of this `Komino`
    /// on the grid such that it covers all `formula_grid` cells of the 
    /// corresponding size and covers none of the `formula_grid` cells of 
    /// different sizes.
    /// # Arguments
    /// * `formula_grid` - The initial positions and values of the solved formula grid cells.
    /// * `assignments` - Any previously assigned `Komino` shape cells on the grid
    /// * `seen` - A set of distinct previously tested assignments of `Komino` cells to the grid, to avoid duplication by position/rotation/reflection
    fn possible_assignments(&self, formula_grid:&HashMap<Coord, usize>, assignments:&HashMap<Coord, usize>, seen:&mut HashSet<KeyArray>) -> Vec<HashSet<Coord>> {
        let mut candidates:Vec<HashSet<Coord>> = Vec::new();
        let mut value_cells:HashSet<Coord> = HashSet::new();
        for (k,v) in formula_grid {
            if *v == self.val {
                value_cells.insert(*k);
            }
        }
        for transform in self.transforms.iter() {
            for y in 0..DIM {
                for x in 0..DIM {
                    let offset = Coord::new(x,y);
                    let transform_cells = Komino::translate(&transform, offset);
                    let mut candidate_okay = true;
                    let mut candidate_value_cell_count = 0;
                    for cell in transform_cells.iter() {
                        // All cells of the candidate position/transform must 
                        // be inside the grid, not cover any formulas of a 
                        // different value and not be previously assigned
                        if !in_bounds(cell) {
                            candidate_okay = false;
                            break;
                        }
                        if formula_grid.contains_key(cell) && *formula_grid.get(cell).unwrap() != self.val {
                            candidate_okay = false;
                            break;
                        }
                        if assignments.contains_key(cell) {
                            candidate_okay = false;
                            break;
                        }
                        if value_cells.contains(cell) {
                            candidate_value_cell_count += 1;
                        }
                    }
                    // Candidate position/transform must cover all formula 
                    // cells matching the intended value
                    if candidate_value_cell_count != value_cells.len() {
                        candidate_okay = false;
                    }
                    if !candidate_okay {
                        continue;
                    }
                    // If an identical assignment of cells has been previously 
                    // considered under a different transform, ignore this one
                    let mut next_assignment = assignments.clone();
                    for cell in transform_cells.iter() {
                        next_assignment.insert(*cell, self.val);
                    }
                    let next_assignment_key = assignment_key(&next_assignment);
                    if seen.contains(&next_assignment_key) {
                        continue;
                    }
                    seen.insert(next_assignment_key);
                    if candidate_okay {
                        candidates.push(transform_cells);
                    }
                }
            }
        }
        return candidates;        
    }
}
/// Struct that builds a series of lookups for viable `Komino` shapes and 
/// inheritances given the formula grid in order to solve the puzzle.
pub struct KominoStore {
    /// Collection of all indexed `Komino` elements, organized by size
    pub kominos:Vec<Vec<Komino>>,
    /// Lookup of all indexed `Komino`s that can construct the `Komino` with 
    /// the given `KeyArray` by adding one additional cell.
    pub predecessors:HashMap<KeyArray, HashSet<(usize, usize)>>,
    /// Lookup of all indexed `Komino`s that can be constructed by adding one
    /// cell to the `Komino` given by the `KeyArray`.
    pub descendants:HashMap<KeyArray, HashSet<(usize, usize)>>,
    /// Maximum size of `Komino`s to index
    pub max_cells:usize,
    /// The puzzle formula grid
    pub formula_grid:HashMap<Coord, usize>
}
impl KominoStore {
    /// Create a new `KominoStore` with a listing of all viable `Komino` shapes
    /// given the `formula_grid` as well as inheritance information.
    /// 
    /// # Arguments
    /// 
    /// * `max_cells` - Greatest `Komino` size to index
    /// * `formula_grid` - The parsed formula grid cell positions and values
    /// * `verbose` - Print progress of constructing `Komino` listings
    pub fn new(max_cells:usize, formula_grid:&HashMap<Coord, usize>, verbose:bool) -> KominoStore {
        assert!(max_cells >= 2, "Max cells must be 2 or greater ({} provided)", max_cells);
        if verbose { println!("Building inheritances for K-ominos assignable to the grid..."); }
        let mut kominos:Vec<Vec<Komino>> = Vec::new();
        let mut komino_lookup:HashMap<KeyArray, (usize, usize)> = HashMap::new();
        let mut predecessors:HashMap<KeyArray, HashSet<(usize, usize)>> = HashMap::new();
        let mut descendants:HashMap<KeyArray, HashSet<(usize, usize)>> = HashMap::new();
        // zero-ominos - none
        kominos.push(Vec::new());
        // one-ominos - one
        let mut one_cells = HashSet::new();
        one_cells.insert(Coord::new(0,0));
        let one_komino = Komino::new(&one_cells);
        komino_lookup.insert(one_komino.base_key, (1, 0));
        predecessors.insert(one_komino.base_key, HashSet::new());
        kominos.push(vec![one_komino]);
        // Build the list and lookup of possible kominos for each size, plus 
        // the lookup of predecessors
        for size in 2..=max_cells {
            let mut size_formula_cells:HashSet<Coord>  = HashSet::new();
            for formula_grid_cell in formula_grid.keys() {
                if *formula_grid.get(formula_grid_cell).unwrap() == size {
                    size_formula_cells.insert(*formula_grid_cell);
                }
            }
            let mut next_kominos_size:Vec<Komino> = Vec::new();
            for i in 0..kominos[size - 1].len() {
                let predecessor_index = komino_lookup.get(&kominos[size - 1][i].base_key).unwrap().clone();
                let next_kominos = kominos[size - 1][i].next_kominos();
                for j in 0..next_kominos.len() {
                    // Test if this shape has viable 
                    // positions/rotations/reflections to be placed on the grid
                    let test_assignments = HashMap::new();
                    let mut test_seen:HashSet<KeyArray> = HashSet::new();
                    if next_kominos[j].possible_assignments(formula_grid, &test_assignments, &mut test_seen).len() == 0 {
                        continue;
                    }
                    let mut found = false;
                    for (k,_) in next_kominos[j].transform_keys.iter() {
                        if komino_lookup.contains_key(k) {
                            found = true;
                            predecessors.get_mut(k).unwrap().insert(predecessor_index);
                            break;
                        }
                    }
                    if !found {
                        // Insert this komino into the store
                        next_kominos_size.push(next_kominos[j].clone());
                        komino_lookup.insert(next_kominos[j].base_key, (size, next_kominos_size.len() - 1));
                        let mut new_komino_predecessor_set = HashSet::new();
                        new_komino_predecessor_set.insert(predecessor_index);
                        predecessors.insert(next_kominos[j].base_key, new_komino_predecessor_set);
                    }
                }
            }
            if verbose { println!("Viable distinct {}-ominos:\t{}", size, next_kominos_size.len()); }
            kominos.push(next_kominos_size);
            
        }
        // Build the descendants lookup
        for key in predecessors.keys() {
            let (predecessor_size, predecessor_index) = komino_lookup.get(key).unwrap();
            let key_predecessors = predecessors.get(key).unwrap();
            for (size, index) in key_predecessors {
                if descendants.contains_key(&kominos[*size][*index].base_key) {
                    descendants.get_mut(&kominos[*size][*index].base_key).unwrap().insert((*predecessor_size, *predecessor_index));
                }
                else {
                    let mut descendant_set = HashSet::new();
                    descendant_set.insert((*predecessor_size, *predecessor_index));
                    descendants.insert(kominos[*size][*index].base_key, descendant_set);
                }
            }
        }
        return KominoStore { kominos:kominos, predecessors:predecessors, descendants:descendants, max_cells:max_cells, formula_grid:formula_grid.clone()};
    }
    /// Find arrangements of `Komino`s that satisfy the formula grid.
    /// 
    /// # Arguments
    /// 
    /// * `verbose` - Print progress
    pub fn solve(&self, verbose:bool) -> Vec<HashMap<Coord, usize>> {
        if verbose { println!("Starting search for K-omino placements..."); }
        // Start from the 12-omino bottleneck and find possible assignments 
        // forward and backwards. This order is a little haphazard, but the 
        // idea is to keep the DFS queue from growing too large by selecting 
        // next K-omino sizes appropriately. 
        let komino_add_order = vec![(12, 0), (11, 12), (10, 11), (9, 10), (8, 9), (13,12), (7,8), (6,7), (5,6), (14,13), (4,5), (15,14), (16,15), (3,4), (2,3), (1,2)];
        let mut solutions_kominos = Vec::new();
        let mut solutions= Vec::new();
        for twelve_komino_index in 0..self.kominos[12].len() {
            // DFS - track state by kominos added. Format is (komino_size, 
            // komino_index, possible assignment of cells on the grid with 
            // rotation/reflection/translation)
            let mut frontier:Vec<Vec<(usize, usize, HashSet<Coord>)>>;
            let mut frontier_next:Vec<Vec<(usize, usize, HashSet<Coord>)>> = Vec::new();
            let temp_assignments = HashMap::new();
            let mut temp_seen = HashSet::new();
            for transform_cells in self.kominos[12][twelve_komino_index].possible_assignments(&self.formula_grid, &temp_assignments, &mut temp_seen) {
                frontier_next.push(vec![(12, twelve_komino_index, transform_cells)]);
            }
            let mut komino_order_index = 0;
            while frontier_next.len() > 0 {
                komino_order_index += 1;
                if komino_order_index >= komino_add_order.len() {
                    for i in 0..frontier_next.len() {
                        solutions_kominos.push(frontier_next[i].clone());
                    }
                    break;
                }
                frontier = frontier_next;
                frontier_next = Vec::new();
                while frontier.len() > 0 {
                    let assignment_list = frontier.pop().unwrap();
                    let mut cell_assignments:HashMap<Coord, usize> = HashMap::new();
                    for (komino_size,_,komino_assignment_cells) in assignment_list.iter() {
                        for cell in komino_assignment_cells.iter() {
                            assert!(!cell_assignments.contains_key(cell), "Double assignment at position {}", cell);
                            cell_assignments.insert(*cell, *komino_size);
                        }
                    }
                    // Because of the non-sequential order, determine if we 
                    // should consider K-ominos that are descended from or 
                    // precede a particular K-omino size.
                    let use_descendants = komino_add_order[komino_order_index].0 > komino_add_order[komino_order_index].1;
                    let predecessor_or_descendant_size = komino_add_order[komino_order_index].1;
                    let mut found_predecessor_descendant = false;
                    let mut predecessor_or_descendant_index = 0;
                    for i in 0..assignment_list.len() {
                        if assignment_list[i].0 == predecessor_or_descendant_size {
                            found_predecessor_descendant = true;
                            predecessor_or_descendant_index = assignment_list[i].1;
                        }
                    }
                    assert!(found_predecessor_descendant, "Could not find predecessor/descendant of size {} in assignment list: {:?}", predecessor_or_descendant_size, assignment_list);
                    let candidate_kominos;
                    if use_descendants {
                        if !self.descendants.contains_key(&self.kominos[predecessor_or_descendant_size][predecessor_or_descendant_index].base_key) {
                            continue;
                        }
                        candidate_kominos = self.descendants.get(&self.kominos[predecessor_or_descendant_size][predecessor_or_descendant_index].base_key).unwrap();
                    }
                    else {
                        if !self.predecessors.contains_key(&self.kominos[predecessor_or_descendant_size][predecessor_or_descendant_index].base_key) {
                            continue;
                        }
                        candidate_kominos = self.predecessors.get(&self.kominos[predecessor_or_descendant_size][predecessor_or_descendant_index].base_key).unwrap();
                    }
                    for (candidate_size,candidate_index) in candidate_kominos {
                        let mut candidate_seen:HashSet<KeyArray> = HashSet::new();
                        for candidate_position in self.kominos[*candidate_size][*candidate_index].possible_assignments(&self.formula_grid, &cell_assignments, &mut candidate_seen).iter() {
                            let mut assignment_list_next = assignment_list.clone(); 
                            assignment_list_next.push((*candidate_size, *candidate_index, candidate_position.clone()));
                            frontier_next.push(assignment_list_next);
                        }
                    }
                }
            }
        }
        for solution_kominos in solutions_kominos.iter() {
            let mut solution_assignments:HashMap<Coord, usize> = HashMap::new();
            for (komino_size,_,komino_cells) in solution_kominos.iter() {
                for cell in komino_cells.iter() {
                    solution_assignments.insert(*cell, *komino_size);
                }
            }
            solutions.push(solution_assignments);
        }
        return solutions;
    }
}
/// Return a fixed length hashable `KeyArray` for the given `shape_cells`
fn shape_key(shape_cells:&HashSet<Coord>) -> KeyArray {
    let mut key_array: KeyArray = [0; DIM2];
    for y in 0..DIM as usize {
        for x in 0..DIM as usize {
            let coord = Coord::new(x as isize, y as isize);
            if shape_cells.contains(&coord) {
                key_array[x + y * DIM as usize] = 1;
            }
        }
    }
    return key_array;
}
/// Return a fixed length hashable `KeyArray` for the given `assignments` 
/// collection of cell positions and assigned values.
fn assignment_key(assignments:&HashMap<Coord, usize>) -> KeyArray {
    let mut key_array: KeyArray = [0; DIM2];
    for y in 0..DIM as usize {
        for x in 0..DIM as usize {
            let coord = Coord::new(x as isize, y as isize);
            if assignments.contains_key(&coord) {
                key_array[x + y * DIM as usize] = *assignments.get(&coord).unwrap() as u8;
            }
        }
    }
    return key_array;
}
/// Test if the given `coord` is within the bounds of the grid
fn in_bounds(coord:&Coord) -> bool { return coord.in_bounds(DIM as usize,DIM as usize); }
/// Print the given `assignments` of cell positions and values
pub fn print_assignment(assignments:&HashMap<Coord, usize>) {
    for y in 0..DIM {
        for x in 0..DIM {
            let coord = Coord::new(x,y);
            if assignments.contains_key(&coord) {
                if *assignments.get(&coord).unwrap() > 9 {
                    print!("{} ", assignments.get(&coord).unwrap());
                }
                else {
                    print!("{}  ", assignments.get(&coord).unwrap());
                }
            }
            else {
                print!(".  ");
            }
        }
        println!();
    }
}
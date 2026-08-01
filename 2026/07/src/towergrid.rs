use crate::coord::Coord;
use std::collections::{HashMap,HashSet};

/// The available types of arithmetic operations on the knight's score.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OpType {
    Add,
    Mul,
    Div
}
impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
        let char = match self {
            OpType::Add => "+",
            OpType::Mul => "*",
            OpType::Div => "/",
        };
        write!(f,"{}", char)
    }
}
impl OpType {
    /// Returns the results of this operation type given the knight score 
    /// `curr_value` and step index `n`.
    pub fn result(&self, curr_value:usize, n:usize) -> Result<usize, String> {
        match self {
            OpType::Add => Ok(curr_value + n),
            OpType::Mul => Ok(curr_value * n),
            OpType::Div => {
                if curr_value % n == 0 {
                    Ok(curr_value/n)
                }
                else {
                    Err(format!("{} is not divisible by {}", curr_value, n))
                }
            }
        }
    }
}

/// Represents an arithmetic operation in a sequence of operations on the 
/// knight's score.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Operation {
    /// Type of arithmetic operation
    pub optype:OpType,
    /// Index N of this operation in the sequence
    pub n:usize,
    /// Resulting value of the knight's score after applying this operation in 
    /// the sequence.
    pub result:usize
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
        write!(f,"{}{} = {}", self.optype, self.n, self.result)
    }
}
impl Operation {
    /// Initialize a new `Operation` of type `optype` at position `n` in the 
    /// sequence with previous knight's score `curr_val`.
    pub fn new(optype:OpType, n:usize, curr_val:usize) -> Result<Operation, String> {
        match optype.result(curr_val, n) {
            Ok(val) => Ok(Operation{optype, n, result:val}),
            Err(e) => Err(e),
        }
    }
    /// Given a sequence of `operations` return if the sequence is valid, the 
    /// number of scored values on the grid reached in the sequence, and the 
    /// number of towers reached in the sequence.
    /// 
    /// A sequence is valid if it doesn't descend while currently on the 
    /// ground, doesn't ascend while currently on a tower, doesn't descend when 
    /// the knight's score is indivisible by N and reaches scored values on 
    /// every third operation for the first 18. 
    /// 
    /// Arguments:
    /// 
    ///   * `operations` - a list of sequential `Operation` elements.
    ///   * `grid` - the puzzle `TowerGrid`.
    ///   * `tower_at_start` - if true, assumes that the knight's starting 
    ///     position is a tower.
    pub fn evaluate_ops(operations:&Vec<Operation>, grid:&TowerGrid, tower_at_start:bool) -> (bool, usize, usize) {
        let mut matched_numbers = HashSet::new();
        let mut valid = true;
        let mut seen_towers = 0;
        let mut on_tower = tower_at_start;
        if on_tower {
            seen_towers += 1;
        }
        for i in 0..operations.len() {
            if (i + 1) <= 18 {
                if (i + 1) % 3 == 0 {
                    // The operations must include one value from the grid on 
                    // every third operation up to the 18th operation.
                    if !grid.numbers_lookup.contains_key(&operations[i].result) {
                        valid = false;
                    }
                    else {
                        matched_numbers.insert(operations[i].result);
                    }
                }
            }
            else {
                if grid.numbers_lookup.contains_key(&operations[i].result) {
                    matched_numbers.insert(operations[i].result);
                }
            }
            match operations[i].optype {
                OpType::Add => {
                    if on_tower {
                        seen_towers += 1;
                    }
                },
                OpType::Mul => {
                    // A multiplication op is impossible if already on a tower
                    if on_tower {
                        valid = false;
                    }
                    else {
                        seen_towers += 1;
                        on_tower = true;
                    }
                },
                OpType::Div => {
                    // A division op is impossible if not already on a tower
                    if !on_tower {
                        valid = false;
                    }
                    else {
                        on_tower = false;
                    }
                    // A division op is impossible if the current score isn't
                    // divisible by N.
                    if i > 0 && operations[i-1].result % (i+1) != 0 {
                        valid = false;
                    }
                }
            }
        }
        matched_numbers.remove(&0);
        return (valid, matched_numbers.len(), seen_towers);
    }

    /// Find the set of valid sequences of arithmetic operations that reach all
    /// knight scores recorded on the grid, do so for every third operation 
    /// within the first 18, and don't exceed the number of possible towers on 
    /// the grid.
    /// 
    /// Arguments:
    /// 
    ///   * `grid` - the puzzle `TowerGrid`.
    ///   * `tower_at_start` - if true, assumes that the knight's starting 
    ///     position is a tower.
    pub fn operation_search(grid:&TowerGrid, tower_at_start:bool) -> Result<HashSet<Vec<Operation>>, String> {
        let mut result = HashSet::new();
        // Upper bound on possible number of operations, since the knight can't
        // make more moves than there are squares on the grid.
        let op_limit = grid.dim*grid.dim;
        let tower_limit = grid.regions.len();
        // A successful list of operations won't include the starting 0 value
        let numbers_to_match = grid.numbers.len() - 1; 
        // For each scored number on the grid, assume it is the final score
        // reached and find any possible sequences of operations that reach it
        // from the start using a BFS. Return any successful sequences that 
        // also reach every other scored value on the grid.
        for number_coord in grid.numbers.keys() {
            let number_value = *grid.numbers.get(number_coord).unwrap();
            if number_value == 0 {
                continue;
            }
            let mut frontier:Vec<Vec<Operation>>;
            let mut frontier_next:Vec<Vec<Operation>> = Vec::new();
            frontier_next.push(Vec::new());
            while frontier_next.len() > 0 {
                frontier = frontier_next;
                frontier_next = Vec::new();
                while frontier.len() > 0 {
                    let state = frontier.pop().unwrap();
                    let (state_valid, matched_numbers, seen_towers) = Operation::evaluate_ops(&state, grid, tower_at_start);
                    if !state_valid || state.len() > op_limit || seen_towers > tower_limit {
                        continue;
                    }
                    if matched_numbers == numbers_to_match {
                        result.insert(state);
                        break;
                    }
                    let curr_val = if state.len() == 0 {0} else {state.last().unwrap().result};
                    for optype in vec![OpType::Add, OpType::Mul, OpType::Div] {
                        match Operation::new(optype, state.len() + 1, curr_val) {
                            Ok(next_op) => {
                                let mut next_state = state.clone();
                                next_state.push(next_op);
                                frontier_next.push(next_state);
                            },
                            Err(_) => {
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return Ok(result);
    }
    /// Determine the period K of recorded knight scores after the 18th 
    /// operation in a sequence of operations that covers all recorded scores 
    /// on the grid and ends on a recorded score. 
    /// 
    /// Returns `None` if there is not a regular period.
    pub fn operations_k(operations:&Vec<Operation>, grid:&TowerGrid) -> Option<usize> {    
        if operations.len() > 18 {
            let remaining_length = operations.len() - 18;
            let total_numbers = 11; // Excluding zero
            let remaining_numbers = total_numbers - 18/3;
            if remaining_length % remaining_numbers == 0 {
                let k = remaining_length / remaining_numbers;
                let mut k_match = true;
                for index in ((17 + k)..operations.len()).step_by(k) {
                    if !grid.numbers_lookup.contains_key(&operations[index].result) {
                        k_match = false;
                        break;
                    }
                }
                return match k_match {
                    true => Some(k),
                    false=> None
                }
            }
            None
        }
        else {
            None
        }
    }
}

/// Represents the puzzle grid.
pub struct TowerGrid {
    /// The dimensions of the grid
    pub dim:usize,
    /// The starting position of the knight
    pub start_coord:Coord,
    /// The list of squares on the grid organized by region
    pub regions:Vec<Vec<Coord>>,
    /// A lookup of cell positions to region index
    pub region_lookup:HashMap<Coord,usize>,
    /// The map of cell positions to noted knight scores on the grid
    pub numbers:HashMap<Coord,usize>,
    /// A lookup of noted knight score values to positions
    pub numbers_lookup:HashMap<usize, Coord>,
    /// A set of deltas for all possible single knights moves in two dimensions
    pub knight_moves:HashSet<Coord>,
    /// A set of deltas for all possible single knights moves if ascending or
    /// descending a tower
    pub knight_moves_short:HashSet<Coord>
}
impl TowerGrid {
    /// Inititialize a new `TowerGrid`
    pub fn new() -> TowerGrid {
        let dim = 8;
        let start_coord = Coord::new(0,7);
        let mut regions:Vec<Vec<Coord>> = Vec::new();
        regions.push(vec![Coord::new(0,0),Coord::new(1,0),Coord::new(2,0),Coord::new(3,0),Coord::new(4,0)]);
        regions.push(vec![Coord::new(5,0),Coord::new(6,0),Coord::new(7,0),Coord::new(7,1),Coord::new(7,2)]);
        regions.push(vec![Coord::new(0,1),Coord::new(1,1),Coord::new(2,1),Coord::new(0,2),Coord::new(2,2)]);
        regions.push(vec![Coord::new(3,1),Coord::new(4,1),Coord::new(3,2),Coord::new(4,2),Coord::new(5,2)]);
        regions.push(vec![Coord::new(5,1),Coord::new(6,1),Coord::new(6,2),Coord::new(6,3),Coord::new(7,3)]);
        regions.push(vec![Coord::new(0,3),Coord::new(0,4),Coord::new(1,4),Coord::new(0,5),Coord::new(0,6)]);
        regions.push(vec![Coord::new(1,2),Coord::new(1,3),Coord::new(2,3),Coord::new(2,4),Coord::new(2,5)]);
        regions.push(vec![Coord::new(3,3),Coord::new(4,3),Coord::new(3,4),Coord::new(4,4)]);
        regions.push(vec![Coord::new(5,3),Coord::new(5,4),Coord::new(6,4),Coord::new(4,5),Coord::new(5,5)]);
        regions.push(vec![Coord::new(1,5),Coord::new(1,6),Coord::new(0,7),Coord::new(1,7),Coord::new(2,7)]);
        regions.push(vec![Coord::new(3,5),Coord::new(2,6),Coord::new(3,6),Coord::new(4,6),Coord::new(3,7)]);
        regions.push(vec![Coord::new(6,5),Coord::new(5,6),Coord::new(6,6),Coord::new(4,7),Coord::new(5,7)]);
        regions.push(vec![Coord::new(7,4),Coord::new(7,5),Coord::new(7,6),Coord::new(6,7),Coord::new(7,7)]);
        let mut region_lookup = HashMap::new();
        for index in 0..regions.len() {
            for coord in regions[index].iter() {
                region_lookup.insert(coord.clone(), index);
            }
        }
        let mut numbers = HashMap::new();
        numbers.insert(Coord::new(5,0),37);
        numbers.insert(Coord::new(7,0),1100);
        numbers.insert(Coord::new(3,2),23);
        numbers.insert(Coord::new(5,2),138);
        numbers.insert(Coord::new(0,3),528);
        numbers.insert(Coord::new(1,4),449);
        numbers.insert(Coord::new(4,4),16);
        numbers.insert(Coord::new(1,5),750);
        numbers.insert(Coord::new(3,5),88);
        numbers.insert(Coord::new(5,5),272);
        numbers.insert(Coord::new(6,5),1);
        numbers.insert(Coord::new(0,7),0);
        let mut numbers_lookup = HashMap::new();
        for coord in numbers.keys() {
            numbers_lookup.insert(*numbers.get(coord).unwrap(), coord.clone());
        }
        let mut knight_moves = HashSet::new();
        knight_moves.insert(Coord::new(-1,-2));
        knight_moves.insert(Coord::new(-2,-1));
        knight_moves.insert(Coord::new(-2,1));
        knight_moves.insert(Coord::new(-1,2));
        knight_moves.insert(Coord::new(1,2));
        knight_moves.insert(Coord::new(2,1));
        knight_moves.insert(Coord::new(2,-1));
        knight_moves.insert(Coord::new(1,-2));
        let mut knight_moves_short = HashSet::new();
        knight_moves_short.insert(Coord::new(0,-2));
        knight_moves_short.insert(Coord::new(-2,0));
        knight_moves_short.insert(Coord::new(2,0));
        knight_moves_short.insert(Coord::new(0,2));
        return TowerGrid { dim, start_coord, regions, region_lookup, numbers, numbers_lookup, knight_moves, knight_moves_short };
    }

    /// Given a `start_coord` and an optional list of previously `visited` 
    /// positions for the knight, as well as assumptions about the 
    /// `next_value` of the knight's score and if there is a change in 
    /// altitude, return a list of possible next positions the knight can move
    /// to.
    pub fn viable_next_positions(&self, start_coord:Coord, change_in_altitude:bool, next_value:usize, visited:Option<&HashSet<Coord>>) -> Vec<Coord> {
        let mut result = Vec::new();
        let rect_dim = Coord::newu(self.dim, self.dim);
        let available_moves = if change_in_altitude { &self.knight_moves_short } else { &self.knight_moves };
        for delta in available_moves.iter() {
            let next_coord = start_coord + *delta;
            let mut okay = true;
            if !next_coord.in_bounds(&rect_dim) {
                okay = false;
            }
            if self.numbers.contains_key(&next_coord) && next_value != *self.numbers.get(&next_coord).unwrap() {
                okay = false;
            }
            if !visited.is_none() && visited.unwrap().contains(&next_coord) {
                okay = false;
            }
            if okay {
                result.push(next_coord);
            }
        }
        return result;
    }

    /// Find possible sequences of moves between recorded knight score 
    /// positions given a viable sequence of arthimetic `operations` that 
    /// covers all the scores. Once all possible path segments have been found, 
    /// find all complete paths that can be constructed from these segments, 
    /// then find a unique final path that includes a visit to the final 
    /// missing tower. Return the final path, the set of towers on the grid, 
    /// the scores of each visited cell and the final puzzle solution value.
    /// 
    /// Arguments:
    /// 
    ///   * `operations` - A viable sequence of arithmetic operations that 
    ///     could produce the recorded knight's scores on the grid.
    ///   * `tower_at_start` - if true, assumes that the knight's starting 
    ///     position is a tower.
    pub fn solve(&self, operations:&Vec<Operation>, tower_at_start:bool) -> Result<(Vec<Coord>, HashSet<Coord>, HashMap<Coord,usize>, usize), String> {
        let mut results:Vec<(Vec<Coord>, HashSet<Coord>, HashMap<Coord,usize>, usize)> = Vec::new();
        let k_option = Operation::operations_k(operations, self);
        if k_option.is_none() {
            return Err(format!("Provided sequence of operations does not have a period k"));
        }
        let k = k_option.unwrap();
        // Break the sequence of operations into segments between the marked
        // scores.
        let mut operation_groups:Vec<Vec<(Operation,bool, bool)>> = Vec::new();
        let mut score_squares:Vec<Coord> = Vec::new();
        let mut on_tower = tower_at_start;
        let mut group_size = 3;
        let mut operation_group:Vec<(Operation, bool, bool)> = Vec::new();
        for i in 0..18 {
            let mut change_in_altitude = false;
            if operations[i].optype == OpType::Div {
                on_tower = false;
                change_in_altitude = true;
            }
            else if  operations[i].optype == OpType::Mul {
                on_tower = true;
                change_in_altitude = true;
            }
            operation_group.push((operations[i], on_tower, change_in_altitude));
            if i > 0 && (i + 1) % group_size == 0 {
                let score_square = *self.numbers_lookup.get(&operations[i].result).unwrap();
                score_squares.push(score_square);
                operation_groups.push(operation_group);
                operation_group = Vec::new();
            }
        }
        group_size = k;
        for i in 18..operations.len() {
            let mut change_in_altitude = false;
            if operations[i].optype == OpType::Div {
                on_tower = false;
                change_in_altitude = true;
            }
            else if  operations[i].optype == OpType::Mul {
                on_tower = true;
                change_in_altitude = true;
            }
            operation_group.push((operations[i], on_tower, change_in_altitude));
            if (i + 1 - 18) % group_size == 0 {
                let score_square = *self.numbers_lookup.get(&operations[i].result).unwrap();
                score_squares.push(score_square);
                operation_groups.push(operation_group);
                operation_group = Vec::new();
            }
        }
        // Given the segmented list of arithmetic operations between the known
        // scored positions, find all possible subpaths that can move between
        // consecutive scored positions.
        let mut group_paths:Vec<Vec<Vec<(Coord, bool)>>> = Vec::new();
        let mut start_pos = self.start_coord;
        for group_index in 0..operation_groups.len() {
            let mut paths:Vec<Vec<(Coord, bool)>> = Vec::new();
            let end_pos = score_squares[group_index];
            let mut frontier:Vec<Vec<(Coord,bool)>>;
            let mut frontier_next:Vec<Vec<(Coord,bool)>> = Vec::new();
            let start_on_tower;
            if group_index == 0 {
                start_on_tower = tower_at_start;
            }
            else {
                (_, start_on_tower, _) = *operation_groups[group_index - 1].last().unwrap()
            }
            frontier_next.push(vec![(start_pos, start_on_tower)]);
            while frontier_next.len() > 0 {
                frontier = frontier_next;
                frontier_next = Vec::new();
                while frontier.len() > 0{
                    let path = frontier.pop().unwrap();
                    let visited:HashSet<Coord> = path.iter().map(|x| x.0).collect();
                    let curr_pos = if path.len() == 0 { start_pos } else { path.last().unwrap().0 };
                    if curr_pos == end_pos && path.len() == operation_groups[group_index].len() + 1 {
                        paths.push(path);
                        continue;
                    }
                    if path.len() >= operation_groups[group_index].len() + 1 {
                        continue;
                    }
                    let next_val = operation_groups[group_index][path.len() - 1].0.result;
                    let next_change_in_altitude = operation_groups[group_index][path.len() - 1].2;
                    let next_on_tower = operation_groups[group_index][path.len() - 1].1;
                    for next_pos in self.viable_next_positions(curr_pos, next_change_in_altitude, next_val, Some(&visited)).iter() {
                        let mut next_path = path.clone();
                        next_path.push((*next_pos, next_on_tower));
                        frontier_next.push(next_path);
                    }
                }
            }
            group_paths.push(paths);
            start_pos = end_pos;
        }
        // Given the possible subpaths that move between the various 
        // consecutive scored positions, recursively find all viable full 
        // paths. 
        let mut full_paths = Vec::new();
        let mut indexes = Vec::new();
        self.solve_recurse(operations, &operation_groups, &group_paths, &mut indexes, &mut full_paths);
        // For each possible full path, try to find subsequent moves which can
        // reach a 13th tower.
        for i in 0..full_paths.len() {
            let mut towers:HashSet<Coord> = HashSet::new();
            let mut positions:HashMap<usize,Coord> = HashMap::new();
            let mut positions_lookup:HashMap<Coord,usize> = HashMap::new();
            for j in 0..full_paths[i].len() {
                
                let (pos, on_tower) = full_paths[i][j];
                positions.insert(j, pos);
                positions_lookup.insert(pos, j);
                if on_tower {
                    towers.insert(pos);
                }
            }
            let mut tower_regions = HashSet::new();
            for tower in towers.iter() {
                let region_index = *self.region_lookup.get(tower).unwrap();
                tower_regions.insert(region_index);
            }
            let mut towerless_regions = HashSet::new();
            for region_index in 0..self.regions.len() {
                if !tower_regions.contains(&region_index) {
                    towerless_regions.insert(region_index);
                }
            }
            let mut tower_candidates = Vec::new();
            for r in towerless_regions.iter() {
                for coord in self.regions[*r].iter() {
                    if !positions_lookup.contains_key(&coord) {
                        tower_candidates.push(coord);
                    }
                }
            }
            for final_tower in tower_candidates {
                let possible_solutions = self.final_step(&full_paths[i], final_tower);
                match possible_solutions {
                    Some(solutions) => {
                        for k in 0..solutions.len() {
                            let final_result = self.validate_and_score(&full_paths[i], &solutions[k]);
                            match final_result {
                                Ok((complete_path, towers, scored_cells, puzzle_solution)) => {
                                    results.push((complete_path, towers, scored_cells, puzzle_solution));
                                },
                                Err(_) => continue
                            }
                        }
                    },
                    None => continue
                }
            }
        }
        // A single reconstructed path is expected.
        if results.len() > 1 {
            return Err(format!("Unique solution not found ({})", results.len()));
        }
        if results.len() == 0 {
            return Err(format!("No solutions found"));
        }
        return Ok(results[0].clone());
    }

    /// Given a possible `final_tower` position and a `partial_path`, find all 
    /// paths between the final position in the given `partial_path`` and the 
    /// `final_tower` if any are possible.
    pub fn final_step(&self, partial_path:&Vec<(Coord, bool)>, final_tower:&Coord) -> Option<Vec<Vec<(Coord, bool)>>> {
        let mut visited_positions = HashSet::new();
        let mut unvisited_positions = HashSet::new();
        for i in 0..partial_path.len() {
            visited_positions.insert(partial_path[i].0);
        }
        for y in 0..self.dim {
            for x in 0..self.dim {
                let coord = Coord::newu(x,y);
                if !visited_positions.contains(&coord) {
                    unvisited_positions.insert(coord);
                }
            }
        }
        let start_position = partial_path.last().unwrap().0;
        let mut solutions = Vec::new();
        let mut frontier;
        let mut frontier_next = Vec::new();
        frontier_next.push(vec![start_position]);
        while frontier_next.len() > 0 {
            frontier = frontier_next;
            frontier_next = Vec::new();
            while frontier.len() > 0 {
                let path = frontier.pop().unwrap();
                if *path.last().unwrap() == *final_tower {
                    let mut augmented_path = Vec::new();
                    for i in 0..path.len() {
                        // The final position will be on a tower
                        if i == path.len() - 1 {
                            augmented_path.push((path[i], true));
                        }
                        else {
                            augmented_path.push((path[i],false));
                        }
                    }
                    solutions.push(augmented_path);
                    continue;
                }
                let curr_pos = path.last().unwrap();
                let mut available_next_positions:HashSet<Coord> = HashSet::new();
                for delta in self.knight_moves.iter() {
                    let next_pos = *curr_pos + *delta;
                    if path.contains(&next_pos) || !unvisited_positions.contains(&next_pos) {
                        continue;
                    }
                    // The final tower can't be reached with a knights move on the plane
                    if next_pos == *final_tower {
                        continue
                    }
                    available_next_positions.insert(next_pos);
                }
                for delta in self.knight_moves_short.iter() {
                    let next_pos = *curr_pos + *delta;
                    if path.contains(&next_pos) || !unvisited_positions.contains(&next_pos) {
                        continue;
                    }
                    // The final tower must be reached with a knights move off of the plane
                    if next_pos != *final_tower {
                        continue
                    }
                    available_next_positions.insert(next_pos);
                }
                for pos in available_next_positions.iter() {
                    let mut next_path = path.clone();
                    next_path.push(*pos);
                    frontier_next.push(next_path);
                }
            }
        }
        if solutions.len() > 0 {
            return Some(solutions);
        }
        return None;
    }

    /// Recursively find all viable full paths between the knight start 
    /// position and the final scored position on the grid given a list of
    /// possible paths between scored positions.
    /// 
    /// Arguments:
    /// 
    ///   * `operations` - A viable sequence of arithmetic operations that 
    ///     could produce the recorded knight's scores on the grid.
    ///   * `operation_groups` - A list of segments of arithmentic operation 
    ///     tuples on the knight's score, with each segment ending on a scored 
    ///     position. The tuples give the operation at the given step, whether 
    ///     the step ends on a tower, and if the step changes the knight's 
    ///     altitude.
    ///   * `group_paths` - For each segment between scored positions of the 
    ///     knight's path, gives the list of all possible viable paths of 
    ///     knight's moves on the grid between the scored positions. Tuples for
    ///     each path segment indicate the position on the grid at that step 
    ///     and if the position contains a tower.
    ///   * `indexes` - the current indexes of the `group_paths` segment paths 
    ///     being evaluated.
    ///   * `solutions` - A list of any viable full paths found by the 
    ///     recursion. Paths are given as a sequence of tuples indicating the 
    ///     position at each step and if the position contains a tower.
    pub fn solve_recurse(&self, operations:&Vec<Operation>, operation_groups:&Vec<Vec<(Operation, bool, bool)>>, group_paths:&Vec<Vec<Vec<(Coord, bool)>>>, indexes:&mut Vec<usize>, solutions:&mut Vec<Vec<(Coord, bool)>>) {
        if indexes.len() == operation_groups.len() {
            let mut full_path = Vec::new();
            for group_index in 0..indexes.len() {
                let mut offset = 0;
                if group_index < indexes.len() - 1 {
                    offset = -1;
                }
                let path_index = indexes[group_index];
                for i in 0..(group_paths[group_index][path_index].len() as isize + offset) as usize {
                    full_path.push(group_paths[group_index][path_index][i].clone());
                }
            }
            solutions.push(full_path);
            return;
        }
        let mut visited:HashSet<Coord> = HashSet::new();
        let mut towers:HashSet<Coord> = HashSet::new();
        let mut tower_regions:HashSet<usize> = HashSet::new();
        for group_index in 0..indexes.len() {
            let path_index = indexes[group_index];
            for i in 0..group_paths[group_index][path_index].len() {
                let pos = group_paths[group_index][path_index][i].0;
                let is_tower = group_paths[group_index][path_index][i].1;
                let region_index = *self.region_lookup.get(&pos).unwrap();
                if visited.contains(&pos) && i > 0 {
                    return;
                }
                if is_tower {
                    if tower_regions.contains(&region_index) && i > 0 {
                        return;
                    }
                    tower_regions.insert(region_index);
                    towers.insert(pos);
                }
                visited.insert(pos);
            }
        }
        // Prospective paths are not viable if we have more than one tower per region
        // or if any positions are visited more than once
        for path_index in 0..group_paths[indexes.len()].len() {
            let mut path_okay = true;
            for i in 0..group_paths[indexes.len()][path_index].len() {
                let (pos, on_tower) = group_paths[indexes.len()][path_index][i];
                if visited.contains(&pos) && i > 0 {
                    path_okay = false;
                    break;
                }
                if on_tower {
                    let tower_region = self.region_lookup.get(&pos).unwrap();
                    if tower_regions.contains(&*tower_region) && i > 0 {
                        path_okay = false;
                        break;
                    }
                }
            }
            if path_okay {
                indexes.push(path_index);
                self.solve_recurse(operations, operation_groups, group_paths, indexes, solutions);
                indexes.pop();
            }
        }
    }

    /// If the path is a valid solution, returns the visited cells in order, 
    /// the set of towers, the visited cells with scores, and the final puzzle 
    /// answer.
    ///  
    /// Arguments:
    /// 
    ///   * `paths_to_score_cells` - A viable path over the grid from the 
    ///     starting position that reaches all scored cells.
    ///   * `final_path` - A path from the last position of 
    ///     `path_to_score_cells` to a final tower on the grid.
    pub fn validate_and_score(&self, paths_to_score_cells:&Vec<(Coord, bool)>, final_path:&Vec<(Coord, bool)>) -> Result<(Vec<Coord>, HashSet<Coord>, HashMap<Coord,usize>, usize), String> {
        let mut complete_path = Vec::new();
        let mut towers = HashSet::new();
        let mut tower_regions = HashSet::new();
        for i in 0..paths_to_score_cells.len() {
            complete_path.push(paths_to_score_cells[i].0);
            if paths_to_score_cells[i].1 {
                towers.insert(paths_to_score_cells[i].0);
                let tower_region = self.region_lookup.get(&paths_to_score_cells[i].0).unwrap();
                tower_regions.insert(tower_region);
            }
        }
        // The start of the final path and the end of the initial path share a 
        // position
        for i in 1..final_path.len() {
            complete_path.push(final_path[i].0);
            if final_path[i].1 {
                towers.insert(final_path[i].0);
                let tower_region = self.region_lookup.get(&final_path[i].0).unwrap();
                tower_regions.insert(tower_region);
            }
        }
        if towers.len() != self.regions.len() {
            return Err(format!("Incorrect number of towers {}", towers.len()));
        }
        if tower_regions.len() != self.regions.len() {
            return Err(format!("Towers do not cover all regions"));           
        }
        let mut visited = HashSet::new();
        for i in 0..complete_path.len() {
            if visited.contains(&complete_path[i]) {
                return Err(format!("Path visits position {} a second time at step {}", complete_path[i], i));
            }
            visited.insert(complete_path[i]);
        }
        let mut score = 0;
        let mut scores = HashMap::new(); 
        let mut curr_tower = paths_to_score_cells[0].1;
        let mut prev_tower;
        for i in 0..complete_path.len() {
            if i == 0 {
                scores.insert(complete_path[i], score);
            }
            else {
                prev_tower = curr_tower;
                curr_tower = towers.contains(&complete_path[i]);
                if prev_tower == curr_tower {
                    score += i;
                }
                else if !prev_tower && curr_tower {
                    score *= i;
                }
                else if prev_tower && !curr_tower {
                    if score % i != 0 {
                        return Err(format!("Tower descent occurs at step {} when score {} is not divisible", i, score));
                    }
                    score /= i;
                }
                scores.insert(complete_path[i], score);
            }
            if self.numbers.contains_key(&complete_path[i]) {
                if *self.numbers.get(&complete_path[i]).unwrap() != score {
                    return Err(format!("Score {} does not match given score {} at position {}", score, complete_path[i], self.numbers.get(&complete_path[i]).unwrap()));
                }
            }
        }
        let orthogonally_adjacent = vec![Coord::new(1,0), Coord::new(0,1), Coord::new(-1,0), Coord::new(0,-1)];
        let mut unvisited = HashSet::new();
        for y in 0..self.dim {
            for x in 0..self.dim {
                let pos = Coord::newu(x,y);
                if !visited.contains(&pos) {
                    unvisited.insert(pos);
                }
            }
        }
        let mut puzzle_answer = 0;
        for pos in unvisited {
            let mut unvisited_neighbor_score_sum = 0;
            for delta in orthogonally_adjacent.iter() {
                let neighbor_pos = pos + *delta;
                if scores.contains_key(&neighbor_pos) {
                    unvisited_neighbor_score_sum += scores.get(&neighbor_pos).unwrap();
                }
            }
            puzzle_answer += unvisited_neighbor_score_sum;
        }
        return Ok((complete_path, towers, scores, puzzle_answer));

    }
    /// Print a given `map` of coordinates to values on the grid, padded to the
    /// given `value_width`. 
    pub fn print_map(&self, values:&HashMap<Coord,usize>, value_width:usize) {
        let blank = left_pad_string(".".to_string(), value_width);
        for y in 0..self.dim {
            for x in 0..self.dim {
                let pos = Coord::newu(x, y);
                if values.contains_key(&pos) {
                    print!("{} ", left_pad_string(format!("{}", values.get(&pos).unwrap()), value_width));
                }
                else {
                    print!("{} ", blank);
                }
            }
            println!();
        }
    }
    /// Print a given `set` of coordinates on the grid, indicated by the given
    /// `indicator` character.
    pub fn print_set(&self, set:&HashSet<Coord>, indicator:char) {
        let blank = ".";
        for y in 0..self.dim {
            for x in 0..self.dim {
                let pos = Coord::newu(x, y);
                if set.contains(&pos) {
                    print!("{} ", indicator);
                }
                else {
                    print!("{} ", blank);
                }
            }
            println!();
        }
    }
    /// Print a given knight's `path` over the grid
    pub fn print_path(&self, path:&Vec<Coord>) {
        let width = 2;
        let mut positions = HashMap::new();
        for i in 0..path.len() {
            positions.insert(path[i], i);
        }
        self.print_map(&positions, width);
    }
}

/// Return the given `string` padded on the left with spaces
/// to the given `width`
fn left_pad_string(string:String, width:usize) -> String {
    let mut result = format!("{}", string);
    while result.len() < width {
        result.insert(0, ' ');
    }
    return result;
}
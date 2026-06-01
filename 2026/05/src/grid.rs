use crate::coord::Coord;
use crate::arctype::ArcType;
use crate::dir::Dir;
use std::collections::{HashMap, HashSet};
use std::cmp::min;
use indicatif::{ProgressBar,ProgressStyle};

/// The parameters of a region on the grid bordered by arcs or grid edges.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GridRegion {
    pub whole_cells:HashSet<Coord>,
    pub arc_cells:HashMap<Coord, ArcType>,
    pub interior_cells:HashSet<Coord>,
    pub area:usize,
    pub smooth_sections:usize,
    pub score:usize
}
impl GridRegion {
    /// Construct a `GridRegion` from the provided `whole_cells` and 
    /// `arc_cells` positions which compose the region and a list of arc 
    /// assignments on the grid.
    pub fn new(whole_cells:&HashSet<Coord>, arc_cells:&HashMap<Coord, bool>, grid_arc_assignments:&HashMap<Coord, ArcType>, dim:&Coord) -> Result<GridRegion, String> {
        let mut inside_count = 0;
        let mut outside_count = 0;
        let mut interior_cells = whole_cells.clone();
        let mut arcs:HashMap<Coord,ArcType> = HashMap::new();
        for arc_pos in arc_cells.keys() {
            if !grid_arc_assignments.contains_key(arc_pos) {
                return Err(format!("Coord {} appears in grid arcs but not grid arc assignments", arc_pos));
            }
            let arc_inside = arc_cells.get(arc_pos).unwrap();
            let arc_type = grid_arc_assignments.get(arc_pos).unwrap();
            arcs.insert(*arc_pos, *arc_type);
            if *arc_inside {
                inside_count += 1;
                interior_cells.insert(*arc_pos);
            }
            else {
                outside_count += 1;
            }
        }
        if inside_count != outside_count {
            return Err("Uneven inside and outside arcs".to_string());
        }
        let area = whole_cells.len() + inside_count;
        let mut segments: HashSet<(Coord,Dir,Coord,Dir)> = HashSet::new();
        let mut included_segments:HashSet<(Coord,Dir,Coord,Dir)> = HashSet::new();
        for arc_pos in arc_cells.keys() {
            let arc_inside = arc_cells.get(arc_pos).unwrap();
            let arc_type = grid_arc_assignments.get(arc_pos).unwrap();    
            if arc_pos.x == 0 {
                if (*arc_inside && (*arc_type == ArcType::A || *arc_type == ArcType::D)) ||
                    (!*arc_inside && (*arc_type == ArcType::B || *arc_type == ArcType::C)) {
                    segments.insert((*arc_pos, Dir::N, arc_pos.add(0,1), Dir::S));
                }
            }
            else if arc_pos.x == dim.x - 1 {
                if (!*arc_inside && (*arc_type == ArcType::A || *arc_type == ArcType::D)) ||
                    (*arc_inside && (*arc_type == ArcType::B || *arc_type == ArcType::C)) {
                    segments.insert((arc_pos.add(1,0), Dir::N, arc_pos.add(1,1), Dir::S));
                }
            }
            if arc_pos.y == 0 {
                if (*arc_inside && (*arc_type == ArcType::B || *arc_type == ArcType::D)) ||
                    (!*arc_inside && (*arc_type == ArcType::A || *arc_type == ArcType::C)) {
                    segments.insert((*arc_pos, Dir::W, arc_pos.add(1,0), Dir::E));
                }
            }
            else if arc_pos.y == dim.y - 1 {
                if (!*arc_inside && (*arc_type == ArcType::B || *arc_type == ArcType::D)) ||
                    (*arc_inside && (*arc_type == ArcType::A || *arc_type == ArcType::C)) {
                    segments.insert((arc_pos.add(0,1), Dir::W, arc_pos.add(1,1), Dir::E));
                }
            }
            let endpoints = ArcType::endpoints(grid_arc_assignments.get(&arc_pos).unwrap(), arc_pos, dim);
            let mut endpoint_keys = endpoints.keys().into_iter();
            let endpoint_1 = endpoint_keys.next().unwrap();
            let endpoint_2 = endpoint_keys.next().unwrap();
            let endpoint_1_dir = endpoints.get(endpoint_1).unwrap().0;
            let endpoint_2_dir = endpoints.get(endpoint_2).unwrap().0;
            segments.insert((*endpoint_1, endpoint_1_dir, *endpoint_2, endpoint_2_dir));
        }
        for whole_pos in whole_cells.iter() {
            if whole_pos.x == 0 {
                segments.insert((*whole_pos, Dir::N, whole_pos.add(0,1), Dir::S));
            }
            else if whole_pos.x == dim.x - 1 {
                segments.insert((whole_pos.add(1,0), Dir::N, whole_pos.add(1,1), Dir::S));
            }
            if whole_pos.y == 0 {
                segments.insert((*whole_pos, Dir::W, whole_pos.add(1,0), Dir::E));
            }
            else if whole_pos.y == dim.y - 1 {
                segments.insert((whole_pos.add(0,1), Dir::W, whole_pos.add(1,1), Dir::E));
            }
        }
        // Trace the border and count discontinuities
        let mut discontinuities = 0;
        let start_segment = segments.iter().next().unwrap();
        let final_dir = start_segment.3;
        included_segments.insert(*start_segment);
        let mut curr_segment = start_segment;
        let mut curr_endoint = curr_segment.0;
        let mut curr_dir = curr_segment.1;
        loop {
            // find next segment
            let next_segment_result = segments.iter().find(|&x| (x.0 == curr_endoint || x.2 == curr_endoint) && !included_segments.contains(x));
            if next_segment_result.is_none() {
                if start_segment.0 == curr_endoint || start_segment.2 == curr_endoint {
                    if !Dir::are_opposite(&curr_dir, &final_dir) {
                        discontinuities += 1;
                    }                  
                    break;
                }
                return Err(format!("Could not find next neighbor for segment {:?}", curr_segment));
            }
            let next_segment = next_segment_result.unwrap();
            included_segments.insert(*next_segment);
            if next_segment.0 == curr_endoint {
                if !Dir::are_opposite(&curr_dir, &next_segment.1) {
                    discontinuities += 1;
                }
                curr_endoint = next_segment.2;
                curr_dir = next_segment.3;
            }
            else {
                if !Dir::are_opposite(&curr_dir, &next_segment.3) {
                    discontinuities += 1;
                }
                curr_endoint = next_segment.0;
                curr_dir = next_segment.1;
            }
            curr_segment = next_segment;
        }
        // The number of smooth sections of the region border is equal to the 
        // number of discontinuities, unless no discontinuities are found (in 
        // which case there is one continuous smooth border).
        let smooth_sections = match discontinuities {
            0 => 1,
            _ => discontinuities
        };
        let score = smooth_sections * area;
        return Ok(GridRegion { whole_cells:whole_cells.clone(), arc_cells:arcs, interior_cells, area, smooth_sections, score});
    }
}

/// Representation of the puzzle grid.
pub struct Grid {
    pub dim:Coord,
    pub value_cells:HashMap<Coord,usize>,
    pub green_cells:HashSet<Coord>
}
impl Grid {
    /// Construct a new grid from an input string and expected dimensions.
    pub fn new(input:&str, dim:Coord) -> Grid {
        let mut value_cells = HashMap::new();
        let mut green_cells = HashSet::new();
        let grid_string = input.trim_start().to_string();
        let grid_lines:Vec<&str> = grid_string.split("\n").collect();
        assert!(grid_lines.len() == dim.y as usize, "Grid has {} rows, {} expected", grid_lines.len(), dim.y);
        for y in 0..grid_lines.len() {
            assert!(grid_lines[y].len() == 3 * dim.x as usize, "Grid row {} has unexpected length ({} characters expected, {} present)", y, 3 * dim.x, grid_lines[y].len());
            // split grid into 3 character groups
            for x in (0..grid_lines[y].len()).step_by(3) {
                let mut s = grid_lines[y][x..min(grid_lines[y].len(),x + 3)].to_string();
                if s.contains("X") {
                    green_cells.insert(Coord::newu(x/3,y));
                    s = s.replace("X", "");
                }
                s = s.trim().to_string();
                match s.parse::<usize>() {
                    Ok(val) => {
                        value_cells.insert(Coord::newu(x/3,y), val);
                    },
                    Err(_) => {}
                }
            }
        }
        return Grid{dim, value_cells, green_cells};
    }
    /// Given a set of arc assignments to the grid, build the list of regions if valid.
    pub fn regions(&self, arc_assignments:&HashMap<Coord,ArcType>) -> Result<Vec<GridRegion>, String> {
        let mut result:Vec<GridRegion> = Vec::new();
        let mut whole_cells = HashSet::new();
        for y in 0..self.dim.y {
            for x in 0..self.dim.x {
                let coord = Coord::new(x,y);
                if !arc_assignments.contains_key(&coord) {
                    whole_cells.insert(coord);
                }
            }
        }
        // If no arcs are on the grid, the region is the entire grid
        if arc_assignments.len() == 0 {
            result.push(GridRegion::new(&whole_cells, &HashMap::new(), arc_assignments, &self.dim).unwrap());
            return Ok(result);
        }
        // Each region includes some number of whole cells (may be zero) and 
        // some number of arcs that make up its border. Each arc borders two 
        // regions, one on its interior and one on its exterior. List all whole 
        // cells one and each arc twice, once as interior and once as exterior.
        let mut remaining_cells:HashSet<(Coord, bool)> = HashSet::new();
        for cell_pos in whole_cells.iter() {
            remaining_cells.insert((*cell_pos, true));
        }
        for arc_pos in arc_assignments.keys() {
            remaining_cells.insert((*arc_pos, true));
            remaining_cells.insert((*arc_pos, false));
        }
        // For each remaining cell, flood fill to find the bounds of the region
        // and its border, compose the region, and remove the contributing 
        // cells from the remaining set
        while remaining_cells.len() > 0 {
            let (start_coord, start_is_inside) = remaining_cells.iter().next().unwrap();
            let mut region_whole_cells:HashSet<Coord> = HashSet::new();
            let mut region_arc_cells:HashMap<Coord,bool> = HashMap::new();
            let mut region_seen:HashSet<Coord> = HashSet::new();
            let mut frontier:Vec<(Coord, bool)>;
            let mut frontier_next:Vec<(Coord, bool)> = Vec::new();
            frontier_next.push((*start_coord, *start_is_inside));
            while frontier_next.len() > 0 {
                frontier = frontier_next;
                frontier_next = Vec::new();
                while frontier.len() > 0 {
                    let (coord, is_inside) = frontier.pop().unwrap();
                    if region_seen.contains(&coord) {
                        continue;
                    }
                    region_seen.insert(coord);
                    let is_arc = arc_assignments.contains_key(&coord);
                    if is_arc {
                        if region_arc_cells.contains_key(&coord) && *region_arc_cells.get(&coord).unwrap() != is_inside {
                            return Err(format!("Arc at position {} encountered in region from both sides. Regions invalid", coord));
                        }
                        region_arc_cells.insert(coord, is_inside);
                    }
                    else {
                        region_whole_cells.insert(coord);
                    }
                    // Add neighbors within the region
                    for d in Dir::dirs() {
                        let neighbor_coord = coord.add_dir(&d);
                        let mut neighbor_inside = true;
                        if neighbor_coord.in_bounds(&self.dim) && !region_seen.contains(&neighbor_coord){
                            // if the current coord is an arc, ignore neighbors on the opposite side
                            if is_arc {
                                if is_inside {
                                    if !ArcType::inside_neighbors(arc_assignments
                                        .get(&coord)
                                        .unwrap(), &coord, &self.dim)
                                        .contains(&neighbor_coord) {
                                        continue;
                                    }
                                }
                                else {
                                    if !ArcType::outside_neighbors(arc_assignments
                                        .get(&coord)
                                        .unwrap(), &coord, &self.dim)
                                        .contains(&neighbor_coord) {
                                        continue;
                                    }
                                }
                            }
                            // if the neighbor is an arc, determine if the current region is inside or ouside it
                            if arc_assignments.contains_key(&neighbor_coord) {
                                let neighbor_arctype = arc_assignments.get(&neighbor_coord).unwrap();
                                if ArcType::outside_neighbors(&neighbor_arctype, &neighbor_coord, &self.dim).contains(&coord) {
                                    neighbor_inside = false;
                                }
                            }
                            frontier_next.push((neighbor_coord, neighbor_inside));
                        }
                    }
                }
            }
            // Compose the region
            let region_result = GridRegion::new(&region_whole_cells, &region_arc_cells, arc_assignments, &self.dim);
            match region_result {
                Ok(region) => result.push(region),
                Err(err) => return Err(err)
            }
            for cell_pos in region_whole_cells.iter() {
                remaining_cells.remove(&(*cell_pos, true));
            }
            for (cell_pos, inside) in region_arc_cells.iter() {
                remaining_cells.remove(&(*cell_pos, *inside));
            }
        }
        return Ok(result);
    }
    /// Return the sum of squares of each row and column sum for a given arc 
    /// assignment to the grid.
    pub fn assignment_score(&self, arc_assignments:&HashMap<Coord,ArcType>) -> Result<usize, String> {
        match self.regions(arc_assignments) {
            Err(err) => Err(err),
            Ok(regions) => {
                let mut val_grid:Vec<Vec<usize>> = Vec::new();
                for _ in 0..self.dim.y {
                    val_grid.push(vec![0;self.dim.x as usize]);
                }
                for i in 0..regions.len() {
                    for interior_cell in regions[i].interior_cells.iter() {
                        if val_grid[interior_cell.y as usize][interior_cell.x as usize] != 0 {
                            return Err(format!("Interior cell overlap found at position {}", interior_cell));
                        }
                        val_grid[interior_cell.y as usize][interior_cell.x as usize] = regions[i].score;
                    }
                }
                let mut row_sum_squares = 0;
                let mut col_sum_squares = 0;
                for y in 0..self.dim.y as usize {
                    let mut col_sum = 0;
                    for x in 0..self.dim.x as usize {
                        col_sum += val_grid[y][x];
                    }
                    col_sum_squares += col_sum * col_sum;
                }
                for x in 0..self.dim.x as usize {
                    let mut row_sum = 0;
                    for y in 0..self.dim.y as usize {
                        row_sum += val_grid[y][x];
                    }
                    row_sum_squares += row_sum * row_sum;
                }
                Ok(row_sum_squares + col_sum_squares)
            }
        }
    }
    /// Returns true if the given arc assignment is a solution to the puzzle.
    pub fn success(&self, arc_assignments:&HashMap<Coord,ArcType>) -> bool {
        match self.regions(arc_assignments) {
            Err(_) => false,
            Ok(regions) => {
                let mut all_values_ok = true;
                let mut values_found = 0;
                for val_cell in self.value_cells.keys() {
                    let val = self.value_cells.get(&val_cell).unwrap();
                    for i in 0..regions.len() {
                        if regions[i].interior_cells.contains(val_cell) {
                            values_found += 1;
                            if *val != regions[i].score {
                                all_values_ok = false;
                                break;
                            }
                        }
                    }
                }
                if values_found != self.value_cells.len() {
                    all_values_ok = false;
                }
                all_values_ok
            }
        }
    }
    /// Convert a hashmap of arc assignments into a hashable list of 
    /// assignments, in raster order, for filtering out duplicates.
    pub fn arc_assignments_key(&self, arcs:&HashMap<Coord, ArcType>) -> Vec<(Coord, ArcType)> {
        let mut result = Vec::new();
        for y in 0..self.dim.y {
            for x in 0..self.dim.x {
                let coord = Coord::new(x,y);
                if arcs.contains_key(&coord) {
                    result.push((coord, *arcs.get(&coord).unwrap()));
                }
            }
        }
        return result;
    }
    /// From the given list of possible `assignments` of arcs to the grid, 
    /// return only those which satisfy each of the given `groups` of value 
    /// cells.
    pub fn assignments_satisfy_groups(&self, assignments:&Vec<HashMap<Coord, ArcType>>, groups:&Vec<Vec<Coord>>, group_max_index:usize) -> Vec<HashMap<Coord, ArcType>> {
        let mut result:Vec<HashMap<Coord, ArcType>> = Vec::new();
        for i in 0..assignments.len() {
            let mut assignment_okay = true;
            let regions_result = self.regions(&assignments[i]);
            match regions_result {
                Err(err) => { 
                    println!("Error with regions in assignment {}: {}", i, err);
                    assignment_okay = false;
               },
                Ok(regions) => {
                    for j in 0..=group_max_index {
                        for k in 0..groups[j].len() {
                            let mut cell_okay = false;
                            let cell_pos = groups[j][k];
                            let cell_val = *self.value_cells.get(&cell_pos).unwrap();
                            for r in 0..regions.len() {
                                if regions[r].interior_cells.contains(&cell_pos) && regions[r].score == cell_val {
                                    cell_okay = true;
                                    break;
                                }
                            }
                            if !cell_okay {
                                assignment_okay = false;
                                break;
                            }
                        }
                        if !assignment_okay {
                            break;
                        }
                    }
                }
            }
            if assignment_okay {
                result.push(assignments[i].clone());
            }

        }
        return result;
    }
    /// Find a satisfying arc arrangement to the puzzle and return the puzzle 
    /// answer as the sum of the squares of each row and column sum using a
    /// search optimized for the solution to the puzzle grid.
    pub fn solution(&self) -> Result<usize, String> {
        // Maximum number of steps to search for strings of arcs to subdivide 
        // regions.
        let limit = 5; 
        let mut seen_solution_arcs:HashSet<Vec<(Coord, ArcType)>> = HashSet::new();
        // Search for solutions to groups of cells which should all appear 
        // within the same region. This list of groups was deterermined after
        // solving with a lengthier method and speeds up the solution process.
        let mut groups:Vec<Vec<Coord>> = Vec::new();
        groups.push(vec![Coord::new(0,4)]); // left 25
        groups.push(vec![Coord::new(2,0),Coord::new(0,1)]); // top-left 21s
        groups.push(vec![Coord::new(1,2),Coord::new(4,1), Coord::new(3,4)]); // 27s
        groups.push(vec![Coord::new(8,2),Coord::new(8,4)]); // right 9s
        groups.push(vec![Coord::new(7,1)]); // top-right 25
        groups.push(vec![Coord::new(5,2)]); // 15
        groups.push(vec![Coord::new(0,6)]); // left 9
        groups.push(vec![Coord::new(4,7)]); // bottom 9
        groups.push(vec![Coord::new(1,7),Coord::new(3,6)]); // 63s
        groups.push(vec![Coord::new(6,8)]); // 35
        groups.push(vec![Coord::new(5,4),Coord::new(7,6)]); // 45s        
        // Validate the groups
        for i in 0..groups.len() {
            let mut group_val = 0;
            for j in 0..groups[i].len() {
                if !self.value_cells.contains_key(&groups[i][j]) {
                    return Err(format!("Error on group {} cell {}. Not a value cell", i, j));
                }
                let cell_val = *self.value_cells.get(&groups[i][j]).unwrap();
                if group_val != 0 && group_val != cell_val {
                     return Err(format!("Error on group {} cell {}. Value {} does not match group val {}", i, j, cell_val, group_val));
                }
                group_val = cell_val;
            }
        }
        println!("Searching for valid next arc arrangements satisfying known regions...\n");
        let mut assignments:Vec<HashMap<Coord,ArcType>> = Vec::new();
        let mut next_assignments:Vec<HashMap<Coord,ArcType>> = Vec::new();
        for i in 0..groups.len() {
            if next_assignments.len() == 0 {
                assignments = self.speculative_arcs(&HashMap::new(), limit);
            }
            else {
                
                let bar = ProgressBar::new(next_assignments.len() as u64);
                bar.set_style(ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} {percent}%").unwrap());
                bar.inc(0);
                for j in 0..next_assignments.len() {
                    let j_result = self.speculative_arcs(&next_assignments[j], limit);
                    for k in 0..j_result.len() {
                        let k_key = self.arc_assignments_key(&j_result[k]);
                        if seen_solution_arcs.contains(&k_key) {
                            continue;
                        }
                        seen_solution_arcs.insert(k_key);
                        assignments.push(j_result[k].clone());
                    }
                    bar.inc(1);
                }
                bar.finish_and_clear();
            }
            next_assignments = self.assignments_satisfy_groups(&assignments, &groups, i);
            if next_assignments.len() == 0 {
                break;
            }
            for j in 0..next_assignments.len() {
                if self.success(&next_assignments[j]) {
                    println!("Arc arrangement found:\n");
                    self.print_grid_assignment(&next_assignments[j]);
                    return self.assignment_score(&next_assignments[j]);
                }
            }
        }
        return Err("No solution found".to_string());
    }
    /// Find a satisfying arc arrangement to the puzzle and return the puzzle 
    /// answer as the sum of the squares of each row and column sum.
    pub fn general_solution(&self, search_limit:usize) -> Result<usize, String> {
        let mut seen_solution_arcs:HashSet<Vec<(Coord, ArcType)>> = HashSet::new();
        let mut assignments = self.speculative_arcs(&HashMap::new(), search_limit);
        let mut next_assignments:Vec<HashMap<Coord,ArcType>> = Vec::new();
        loop {
            let bar = ProgressBar::new(assignments.len() as u64);
            bar.set_style(ProgressStyle::with_template("{bar:40} {pos:>7}/{len:7} {percent}%").unwrap());
            bar.inc(0);
            for i in 0..assignments.len() {
                let mut skippable = false;
                let key = self.arc_assignments_key(&assignments[i]);
                if seen_solution_arcs.contains(&key) {
                    bar.inc(1);
                    continue;
                }
                seen_solution_arcs.insert(key);
                for j in 0..next_assignments.len() {
                    
                    if arc_assignment_contained(&assignments[i], &next_assignments[j]) {
                        skippable = true;
                        break;
                    }
                }
                if skippable {
                    bar.inc(1);
                    continue;
                }
                let next_assignments_i = self.speculative_arcs(&assignments[i], search_limit);
                for j in 0..next_assignments_i.len() {
                    let mut skippable_i_j = false;
                    for k in 0..next_assignments.len() {
                        if arc_assignment_contained(&next_assignments_i[j], &next_assignments[k]) {
                            skippable_i_j = true;
                            break;
                        }
                    }
                    if !skippable_i_j {
                        if self.success(&next_assignments_i[j]) {
                            bar.finish_and_clear();
                            println!("Arc arrangement found:\n");
                            self.print_grid_assignment(&next_assignments_i[j]);
                            return self.assignment_score(&next_assignments_i[j]);
                        }
                        next_assignments.push(next_assignments_i[j].clone());
                    }
                }
            }
            bar.finish_and_clear();
            if next_assignments.len() == 0 {
                break;
            }
            assignments = next_assignments;
            next_assignments = Vec::new();
        }
        return Err("No solution found".to_string());
    }
    /// From the current `arc_assignment`, find possible next assignments of 
    /// arcs that divide the largest region that doesn't have all value cells 
    /// satisified, up to a maximum number of `limit` new arcs. 
    pub fn speculative_arcs(&self, curr_arc_assignment:&HashMap<Coord, ArcType>, limit:usize) -> Vec<HashMap<Coord, ArcType>> {
        let mut solutions:Vec<HashMap<Coord, ArcType>> = Vec::new();
        let regions_result = self.regions(curr_arc_assignment);
        if regions_result.is_err() {
            println!("Region error on speculative arcs: {}", regions_result.err().unwrap());
            return Vec::new();
        }
        let regions = regions_result.unwrap();
        // Find largest region with inconsistant value cells
        let mut region_index = 0;
        let mut region_max_interior = 0;
        for i in 0..regions.len() {
            let mut all_val_cells_okay = true;
            for interior_cell in regions[i].interior_cells.iter() {
                if self.value_cells.contains_key(interior_cell) {
                    if regions[i].score != *self.value_cells.get(interior_cell).unwrap() {
                        all_val_cells_okay = false;
                        break;
                    }
                }
            }
            if !all_val_cells_okay && regions[i].interior_cells.len() > region_max_interior {
                region_index = i;
                region_max_interior = regions[i].interior_cells.len();
            }
        }
        // Find all points on the border of the region to be divided where an 
        // arc could attach.
        let mut start_coords:HashSet<Coord> = HashSet::new();
        let mut attachment_points:HashSet<Coord> = HashSet::new();
        for whole_cell in regions[region_index].whole_cells.iter() {
            if self.green_cells.contains(&*whole_cell) {
                continue;
            }
            if whole_cell.x == 0 {
                start_coords.insert(*whole_cell);
                attachment_points.insert(*whole_cell);
                attachment_points.insert(*&whole_cell.add(0, 1));
            }
            if whole_cell.x == self.dim.x - 1 {
                start_coords.insert(*whole_cell);
                attachment_points.insert(whole_cell.add(1,0));
                attachment_points.insert(whole_cell.add(1, 1));
            }
            if whole_cell.y == 0 {
                start_coords.insert(*whole_cell);
                attachment_points.insert(*whole_cell);
                attachment_points.insert(whole_cell.add(1, 0));
            }
            if whole_cell.y == self.dim.y - 1 {
                start_coords.insert(*whole_cell);
                attachment_points.insert(whole_cell.add(0,1));
                attachment_points.insert(*&whole_cell.add(1, 1));
            }
        }
        for arc_cell in regions[region_index].arc_cells.keys() {
            let arc_type = regions[region_index].arc_cells.get(arc_cell).unwrap();
            let endpoints = ArcType::endpoints(arc_type, arc_cell, &self.dim);
            for endpoint in endpoints.keys() {
                attachment_points.insert(*endpoint);
                let (_,neighbors) = endpoints.get(endpoint).unwrap();
                for neighbor in neighbors {
                    if !self.green_cells.contains(neighbor) && regions[region_index].whole_cells.contains(neighbor) {
                        start_coords.insert(*neighbor);
                    }
                }
            }
        }
        // From each starting coord, DFS to find possible groups of arcs that 
        // divide the region into two parts. At least one of the two new 
        // subregions must have all value cells fully satisified.
        let mut seen_assignments:HashSet<Vec<(Coord,ArcType)>> = HashSet::new();
        for start_coord in start_coords.iter() {
            for t in vec![ArcType::A, ArcType::B, ArcType::C, ArcType::D] {
                let start_endpoints = ArcType::endpoints(&t, start_coord, &self.dim);
                // The arc at this position must be able to join with one of the attachment points
                for start_endpoint in start_endpoints.keys() {
                    if !attachment_points.contains(start_endpoint) {
                        continue;
                    }
                    let mut new_arcs:Vec<(Coord, ArcType)> = Vec::new();
                    new_arcs.push((*start_coord, t));
                    self.arc_dfs(curr_arc_assignment, 
                        &start_coords, 
                        &attachment_points, 
                        &limit, 
                        start_endpoint, 
                        &mut new_arcs, 
                        &mut solutions,
                        &mut seen_assignments);  
                }                 
            }
        }
        return solutions;
    }
    /// Search for new arc groups that subdivide the given region and result in
    /// at least one new region that fully satisfies all contained value cells.
    pub fn arc_dfs(&self, 
        curr_arc_assignment:&HashMap<Coord, ArcType>, 
        start_coords:&HashSet<Coord>, 
        attachment_points:&HashSet<Coord>, 
        limit:&usize, 
        last_endpoint_coord:&Coord, 
        new_arcs:&mut Vec<(Coord, ArcType)>, 
        solutions:&mut Vec<HashMap<Coord,ArcType>>, 
        seen_arcs:&mut HashSet<Vec<(Coord, ArcType)>>) {
        let mut new_arc_assignments = curr_arc_assignment.clone();
        let mut new_arc_assignments_solo = HashMap::new();
        for i in 0..new_arcs.len() {
            new_arc_assignments.insert(new_arcs[i].0, new_arcs[i].1);
            new_arc_assignments_solo.insert(new_arcs[i].0, new_arcs[i].1);
        }
        let new_arc_assignments_key = self.arc_assignments_key(&new_arc_assignments_solo);
        if seen_arcs.contains(&new_arc_assignments_key) {
            return;
        }
        seen_arcs.insert(new_arc_assignments_key);
        // Check if the current new arcs have reached an attachement point
        let (last_pos, last_type) = new_arcs.last().unwrap();
        let endpoints = ArcType::endpoints(last_type, last_pos, &self.dim);
        if start_coords.contains(last_pos) {
            let mut endpoint_at_attachment_point = false;
            for endpoint in endpoints.keys() {
                if attachment_points.contains(endpoint) && endpoint != last_endpoint_coord {
                    endpoint_at_attachment_point = true;
                    break;
                }
            }
            if endpoint_at_attachment_point {
                let new_regions_result = self.regions(&new_arc_assignments);
                if new_regions_result.is_err() {
                    return;
                }
                let new_regions = new_regions_result.unwrap();
                let mut regions_containing_value_cells = 0;
                let mut regions_containing_correct_value_cells = 0;
                for i in 0..new_regions.len() {
                    let mut vals_found = 0;
                    let mut vals_correct = 0;
                    for val_cell in self.value_cells.keys() {
                        if new_regions[i].interior_cells.contains(val_cell) {
                            vals_found += 1;
                            let val = self.value_cells.get(val_cell).unwrap();
                            if new_regions[i].score == *val {
                                vals_correct += 1;
                            }
                        }
                    }
                    if vals_found > 0 {
                        regions_containing_value_cells += 1;
                        if vals_correct == vals_found {
                            regions_containing_correct_value_cells += 1;
                        }
                    }
                }
                if regions_containing_value_cells == new_regions.len() && regions_containing_correct_value_cells >= regions_containing_value_cells - 1 {
                    solutions.push(new_arc_assignments);
                    return;
                }
            }
        }
        if new_arcs.len() > *limit {
            return;
        }
        // Try all neighboring arcs the connect to the current arc
        let mut remaining_endpoints = endpoints.keys().filter(|x| **x != *last_endpoint_coord);
        let next_endpoint_pos:Coord;
        match remaining_endpoints.next() {
            Some(coord) => next_endpoint_pos = *coord,
            None => {
                return;
            }
        }
        let (_, next_endpoint_neighbors) = endpoints.get(&next_endpoint_pos).unwrap();
        for neighbor in next_endpoint_neighbors.iter() {
            if curr_arc_assignment.contains_key(neighbor) || self.green_cells.contains(neighbor) {
                continue;
            }
            let mut previously_added = false;
            for i in 0..new_arcs.len() {
                if new_arcs[i].0 == *neighbor {
                    previously_added = true;
                }
            }
            if previously_added {
                continue;
            }
            for t in vec![ArcType::A, ArcType::B, ArcType::C, ArcType::D] {
                let next_arc_endpoints = ArcType::endpoints(&t, neighbor, &self.dim);
                if !next_arc_endpoints.contains_key(&next_endpoint_pos) {
                    continue;
                }        
                new_arcs.push((*neighbor, t));
                self.arc_dfs(curr_arc_assignment, start_coords, attachment_points, limit, &next_endpoint_pos, new_arcs, solutions, seen_arcs);
                new_arcs.pop();
            }
        }
    }
    /// Print an arc assignment on the grid along with the region scores 
    /// assigned to cells.
    pub fn print_grid_assignment(&self, arc_assignments:&HashMap<Coord,ArcType>) {
        for y in 0..self.dim.y {
            for x in 0..self.dim.x {
                let coord = Coord::new(x,y);
                if arc_assignments.contains_key(&coord) {
                    print!("{} ", arc_assignments.get(&coord).unwrap());
                }
                else if self.green_cells.contains(&coord) {
                    print!("◼ ");
                }
                else {
                    print!("◻ ");
                }
            }
            println!();
        }
        println!();
        let regions_result = self.regions(arc_assignments);
        match regions_result {
            Err(err) => {
                println!("Region error: {}", err);
                return;
            }
            Ok(regions) => {
                for y in 0..self.dim.y {
                    for x in 0..self.dim.x {
                        let coord = Coord::new(x,y);
                        let mut score = 0;
                        for i in 0..=regions.len() {
                            if regions[i].interior_cells.contains(&coord) {
                                score = regions[i].score;
                                break;
                            }
                        }
                        print!("{} ", pad_val(score, 4));
                    }
                    println!();
                }
                println!();
            }
        }       
    }
}

/// Returns a decimal string of the given `val` right-padded to `padding` 
/// characters if shorter.
pub fn pad_val(val:usize, padding:usize) -> String {
    let mut val_str = format!("{}", val);
    while val_str.len() < padding {
        val_str.push_str(" ");
    }
    return val_str;
}

/// Returns true if every arc assigned in `arc_assignments_a` is contained in `arc_assignments_b`
pub fn arc_assignment_contained(arc_assignments_a:&HashMap<Coord,ArcType>, arc_assignments_b:&HashMap<Coord,ArcType>) -> bool {
    for coord in arc_assignments_a.keys() {
        if !arc_assignments_b.contains_key(coord) {
            return false;
        }
        let t = arc_assignments_a.get(coord).unwrap();
        if *arc_assignments_b.get(coord).unwrap() != *t {
            return false;
        }
    }
    return true;
}
pub mod komino;
pub mod coord;
use komino::{KominoStore, DIM, print_assignment};
use coord::Coord;
use std::collections::{HashMap};
use std::time::{Instant};
use clap::{Arg, ArgAction, Command};

/// Solved formula grid as a string
const FORMULA_GRID:&str = r#"
.  .  .  .  15 .  .  .  .  .  .  .  .  
.  .  .  .  .  .  .  11 .  .  .  .  .  
.  15 .  5  .  .  15 .  11 .  11 .  .  
.  .  .  .  15 .  .  8  .  12 .  12 .  
.  16 .  .  .  8  .  .  .  .  6  .  .  
.  .  .  16 .  .  .  .  .  .  .  .  6  
.  .  16 .  3  .  16 .  1  .  12 .  .  
13 .  .  .  .  .  .  .  .  4  .  .  .  
.  .  7  .  .  .  .  12 .  .  .  10 .  
.  2  .  13 .  16 .  .  14 .  .  .  .  
.  .  13 .  14 .  14 .  .  14 .  10 .  
.  .  .  .  .  9  .  .  .  .  .  .  .  
.  .  .  .  .  .  .  .  9  .  .  .  ."#;

/// Parse the solved FORMULA_GRID string and return the position and value of
/// each formula cell
fn parse_grid() -> HashMap<Coord, usize>{
    let mut result = HashMap::new();
    let grid_string = FORMULA_GRID.trim().to_string();
    let grid_lines:Vec<&str> = grid_string.split("\n").collect();
    assert!(grid_lines.len() == DIM as usize, "Grid has {} rows, {} expected", grid_lines.len(), DIM);
    for y in 0..grid_lines.len() {
        let row_split:Vec<&str> = grid_lines[y].split_ascii_whitespace().collect();
        assert!(row_split.len() == DIM as usize, "Grid has {} columns on row {}, {} expected", row_split.len(), y, DIM);
        for x in 0..row_split.len() {
            if row_split[x] == "." {
                continue;
            }
            let val = usize::from_str_radix(row_split[x], 10).expect("Unable to parse number");
            result.insert(Coord::new(x as isize,y as isize), val);
        }
    }
    return result;
}
/// Return the product of the maximum and minimum row sums of a solution
fn solution_score(solution:&HashMap<Coord, usize>) -> usize {
    let mut row_sums = Vec::new();
    for y in 0..DIM {
        let mut row_sum = 0;
        for x in 0..DIM {
            let coord = Coord::new(x,y);
            if solution.contains_key(&coord) {
                row_sum += solution.get(&coord).unwrap();
            }
        }
        row_sums.push(row_sum);
    }
    let min_row = row_sums.iter().min().unwrap();
    let max_row = row_sums.iter().max().unwrap();
    return min_row * max_row;
}
fn main() {
    let command = Command::new("feb2026").max_term_width(80)
        .about("Solver for the Jane Street February 2026 puzzle.")
        .arg(Arg::new("verbose").help("Display solution progress and details").short('v').long("verbose").action(ArgAction::SetTrue));
    let args = command.get_matches();
    let verbose = args.get_flag("verbose");
    println!("####### Jane Street Puzzle - February 2026 #######\n");
    let start_instant = Instant::now();
    let formula_grid = parse_grid();
    if verbose { 
        println!("Starting formula grid with a = 1/4, b = -3, c = 1/2:"); 
        print_assignment(&formula_grid);
    }
    let komino_store = KominoStore::new(16, &formula_grid, verbose);
    let solutions = komino_store.solve(verbose);
    assert!(solutions.len() > 0, "No solutions found");
    assert!(solutions.len() == 1, "Multiple solutions found unexpectedly");
    if verbose {
        println!("Unique solution found");
        print_assignment(&solutions[0]);
    }
    let puzzle_solution = solution_score(&solutions[0]);
    println!("Puzzle solution:\t{}", puzzle_solution);
    println!("\nTotal execution time: {:?}", start_instant.elapsed());        
}

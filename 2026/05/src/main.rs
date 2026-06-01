pub mod coord;
pub mod grid;
pub mod arctype;
pub mod dir;
use grid::Grid;
use coord::Coord;
use clap::{Command,Arg,ArgAction};

/// The puzzle grid represented as a string. X positions denote green squares.
const ARC_GRID:&str = r#"
X  .  21 X  .  X  .  X  .  
21 .  .  X  27 .  X  25 X  
X  27 .  .  .  15 .  X  9  
.  .  .  .  .  .  .  .  X  
25X.  .  27 X  45 .  .  9  
.  .  .  .  .  .  .  X  X  
9  .  .  63 .  .  .  45 .  
X  63 .  .  9  .  .  .  288
.  X  .  .  X  .  35X.  X ."#;

/// The sample grid represented as a string
const SAMPLE_GRID:&str = r#"
3  X  9  X  
.  .  .  6  
8  X  .  X  
.  6  .  24 "#;
/// Puzzle grid horizontal and vertical dimensions.
const GRID_DIM:isize = 9;
/// Sample grid horizontal and vertical dimensions.
const SAMPLE_GRID_DIM:isize = 4;
/// Limit to length of arc "strings" that subdivide regions to search for in 
/// the sample grid solution.
const SAMPLE_GRID_SEARCH_LIMIT:usize = 4;

fn main() {
    let command = Command::new("may2026").max_term_width(80)
        .about("Solver for the Jane Street May 2026 puzzle.")
        .arg(Arg::new("sample").help("Solve the sample grid").short('s').long("sample").action(ArgAction::SetTrue));
    let args = command.get_matches();
    let sample = args.get_flag("sample");
    println!("####### Jane Street Puzzle - May 2026 #######\n");
    if sample {
        // Solve the sample grid using the general solution
        let sample_grid = Grid::new(SAMPLE_GRID, Coord::new(SAMPLE_GRID_DIM,SAMPLE_GRID_DIM));
        match sample_grid.general_solution(SAMPLE_GRID_SEARCH_LIMIT) {
            Ok(val) => println!("Sample grid solution:\t{val}"),
            Err(err) => println!("Error: {}", err)
        }
    }
    else {
        // Solve the main grid using an optimized solution
        let grid = Grid::new(ARC_GRID, Coord::new(GRID_DIM,GRID_DIM));
        match grid.solution() {
            Ok(val) => println!("Solution:\t{val}"),
            Err(err) => println!("Error: {}", err)
        }
    }
}

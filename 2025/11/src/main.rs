pub mod grid;
use grid::{BoxGrid};
use std::collections::{HashSet};
use std::time::{Instant};
use clap::{Arg, ArgAction, Command};

fn main() {
    let command = Command::new("nov2025").max_term_width(80)
        .about("Solver for the Jane Street November 2025 puzzle.")
        .arg(Arg::new("sample").help("Solve the sample puzzle").short('s').long("sample").action(ArgAction::SetTrue))
        .arg(Arg::new("verbose").help("Display solution details").short('v').long("verbose").action(ArgAction::SetTrue));
    let args = command.get_matches();
    let verbose = args.get_flag("verbose");
    let is_sample = args.get_flag("sample");

    println!("####### Jane Street Puzzle - November 2025 #######\n");
    let start_instant = Instant::now();
    let (puzzle_desc, puzzle_desc_cap) = if is_sample { ("sample", "Sample") } else {("main", "Main")};
    let mut grid = BoxGrid::new(is_sample);
    let mut grid_known_good = HashSet::new();
    let mut grid_ruled_out = HashSet::new();
    if verbose {
        println!("Grid:");
        grid.print_set(true, true, &grid_known_good);
    }
    // Perform initial inference pass, assigning as many grid cells to box cell
    // and non-box cell groups as possible.
    grid.inference(&mut grid_known_good, &mut grid_ruled_out);
    if verbose {
        println!("{} of {} cells assigned in initial inference",  grid_known_good.len() + grid_ruled_out.len(), grid.dim * grid.dim);
        println!("Searching for valid box cell arrangements...");
    }
    // Search for all viable assignments of remaining cells to box cell and 
    // non-box cell groups
    let possible_box_cell_arrangements = grid.speculation(&grid_known_good, &grid_ruled_out);
    if verbose {
        println!("Searching for solutions from {} box cell arrangement{}...", possible_box_cell_arrangements.len(), if possible_box_cell_arrangements.len() == 1{ "" } else {"s"});
    }
    let mut solution_found = false;
    for i in 0..possible_box_cell_arrangements.len() {
        let results = grid.solidify(&possible_box_cell_arrangements[i].0, &possible_box_cell_arrangements[i].1, verbose);
        match results {
            Ok(opt) => {
                match opt {
                    Some(x) => {
                        println!("{} puzzle solution: {}", puzzle_desc_cap, x);
                        solution_found = true;
                        break
                    },
                    None => {}
                }
            },
            Err(e) => println!("Error during solving for speculation result {}: {}", i, e)
        }
    }
    if !solution_found {
        println!("No {} puzzle solution found", puzzle_desc);
    }
    println!("\nTotal execution time: {:?}", start_instant.elapsed());
}

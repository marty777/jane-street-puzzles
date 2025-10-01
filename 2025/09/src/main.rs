mod grid;
use grid::{Hook, Coord};
use std::time::{Instant};
use clap::{Arg, Command};
use indicatif::{ProgressBar,ProgressStyle};

fn main() {
	let command = Command::new("sept2025").max_term_width(80)
        .about("Solver for the Jane Street September 2025 puzzle.")
        .arg(Arg::new("threads").help("Set maximum number of worker threads").short('t').long("threads").value_name("THREADS").default_value("4"));
    let args = command.get_matches();    
    let mut threads = 4;
    if let Some(threads_arg) = args.get_one::<String>("threads") {
        match threads_arg.parse::<usize>() {
            Ok(n) => {
                threads = n;
                if threads < 1 {
                    println!("THREADS must be at least 1 ({} provided)", threads_arg);
                    std::process::exit(2);
                }
            },
            Err(_) => {
                println!("Could not parse THREADS argument '{}' as an integer.", threads_arg);
                std::process::exit(2);
            }
        }
    }
    println!("####### Jane Street Puzzle - September 2025 #######\n");
	let start_instant = Instant::now();
	// Initialize pentomino classes
    let pentomino_map = grid::init_pentominos();
	// Find all potentially valid hook arrangements
    let mut hook_arrangements:Vec<Vec<Hook>> = Vec::new();
    grid::hook_recurse(9, &mut Vec::new(), Coord::new(0,0), &mut hook_arrangements);
	// Find all potentially valid assignments of numbers to valid hooks
    let mut hook_number_assignments:Vec<(Vec<Hook>, Vec<usize>)> = Vec::new();
	for i in 0..hook_arrangements.len() {
		let mut solutions:Vec<Vec<usize>> = Vec::new();
		let mut initial_assignment = vec![0; grid::GRID_DIM];
		grid::hook_number_assignment_recurse(&hook_arrangements[i], &mut initial_assignment, 0, &mut solutions);
		for j in 0..solutions.len() {
			hook_number_assignments.push((hook_arrangements[i].clone(), solutions[j].clone()));
		}
	}
	println!("Searching for pentomino positions within {} potential number/hook combinations with {} worker threads...", hook_number_assignments.len(), threads);	
	let bar = ProgressBar::new(hook_number_assignments.len() as u64);
	bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {percent}%").unwrap());
    bar.inc(0);
    for i in 0..hook_number_assignments.len() {
        let pentomino_solutions = grid::pentomino_permutations(&pentomino_map, &hook_number_assignments, i, threads);
		bar.inc(1);
        if pentomino_solutions.len() > 0 {
            bar.finish_and_clear();
            if pentomino_solutions.len() != 1 {
                println!("Multiple solutions found, something went wrong: {:?}", pentomino_solutions);
                break;
            }
            let pentominos = pentomino_solutions.iter().next().unwrap().clone();
            println!("\nHooks:");
            grid::hook_print(&hook_number_assignments[i].0);
            println!("\nNumber assignments to hooks:");
            for j in 0..hook_number_assignments[i].0.len() {
                println!("{}x{} <- {}", hook_number_assignments[i].0[j].dim, hook_number_assignments[i].0[j].dim, hook_number_assignments[i].1[j]);
            }
            println!("\nPentominos:");
            grid::pentominos_print(&pentominos);
			
            println!("\nDigits:");
            for y in 0..grid::GRID_DIM {
                for x in 0..grid::GRID_DIM {
                    let coord = Coord::new(x as isize, y as isize);
                    let mut found = false;
                    for pentomino in pentominos.iter() {
                        if pentomino.get_cells_with_offset().contains(&coord) {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        for j in 0..hook_number_assignments[i].0.len() {
                            if hook_number_assignments[i].0[j].cells().contains(&coord) {
                                print!("{} ", hook_number_assignments[i].1[j]);
                                break;
                            }
                        }
                    }
                    else {
                        print!(". ");
                    }
                }
                println!();
            }
			
			println!("\nPentomino sums:");
			for pentomino in pentominos.iter() {
				let mut pentomino_sum = 0;
				for coord in pentomino.get_cells_with_offset() {
					for j in 0..hook_number_assignments[i].0.len() {
						if hook_number_assignments[i].0[j].cells().contains(&coord) {
							pentomino_sum += hook_number_assignments[i].1[j];
							break;
						}
					}
				}
				println!("{}: {}", pentomino.class, pentomino_sum);
			}
			
            println!("\nProduct of empty region areas: {}", grid::pentominos_empty_cell_product(&pentominos));
            break;
        }
	}
	println!("\nTotal execution time: {:?}", start_instant.elapsed());
}

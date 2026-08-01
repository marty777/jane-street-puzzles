pub mod coord;
pub mod towergrid;
use towergrid::{TowerGrid, Operation};

fn main() {
    println!("####### Jane Street Puzzle - July 2026 #######");
    let grid = TowerGrid::new();
    // Based on experimentation, the knight's starting position must be a tower
    let tower_at_start = true;
    // Find sequences of arithmetic operations that can match the knight's 
    // recorded scores.
    let operations_results = Operation::operation_search(&grid, tower_at_start).unwrap();
    if operations_results.len() != 1 {
        println!("A unique sequence of arithmetic operations to produce the knight's scores could not be found");
        return;
    }
    let operations = operations_results.iter().next().unwrap();
    // Using the sequence of operations, reconstruct the unique knight's path 
    // over the grid between scored positions along with the remaining moves to 
    // the inferred final tower.
    match grid.solve(operations, tower_at_start) {
        Ok((path, towers, scored_cells, solution)) => {
            println!("\nKnight visited square sequence:");
            grid.print_path(&path);
            println!("\nTower positions:");
            grid.print_set(&towers, 'T');
            println!("\nVisited square scores:");
            grid.print_map(&scored_cells, 5);
            println!("\nSolution: {}", solution);
        },
        Err(e) => {
            println!("Solution not found: {}", e);
        }
    }
}

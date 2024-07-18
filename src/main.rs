use crate::rayon_parallel_solver::{format_board, print_board, solve_par_rayon};
use crate::sequential_solver::SudokuSolver;
mod rayon_parallel_solver;
mod sequential_solver;
mod crossbeam_par_solver;
mod nonpar_solver;

use std::time::Instant;
use crate::crossbeam_par_solver::solve_par_crossbeam;

fn main() {
    println!(r#"
███████╗██╗   ██╗██████╗  ██████╗ ██╗  ██╗██╗   ██╗    ███████╗ ██████╗ ██╗    ██╗   ██╗███████╗██████╗
██╔════╝██║   ██║██╔══██╗██╔═══██╗██║ ██╔╝██║   ██║    ██╔════╝██╔═══██╗██║    ██║   ██║██╔════╝██╔══██╗
███████╗██║   ██║██║  ██║██║   ██║█████╔╝ ██║   ██║    ███████╗██║   ██║██║    ██║   ██║█████╗  ██████╔╝
╚════██║██║   ██║██║  ██║██║   ██║██╔═██╗ ██║   ██║    ╚════██║██║   ██║██║    ╚██╗ ██╔╝██╔══╝  ██╔══██╗
███████║╚██████╔╝██████╔╝╚██████╔╝██║  ██╗╚██████╔╝    ███████║╚██████╔╝███████╗╚████╔╝ ███████╗██║  ██║
╚══════╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝     ╚══════╝╚═════╝ ╚══════╝ ╚═══╝  ╚══════╝╚═╝  ╚═╝
    "#);
    println!("**************************************************************************************");

    let board = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |I|N|P|U|T|   |B|O|A|R|D|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");
    print_board(board);
    println!("**************************************************************************************");
    println!("+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
 |O|U|T|P|U|T|   |B|O|A|R|D|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+");

    println!(" ");

    println!("************************ Solution - Parallel Version - Rayon ************************");


    let now = Instant::now();
    let solutions = solve_par_rayon(board);
    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution {} - Parallel Rayon:\n{}", i + 1, format_board(solution));
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed time (parallel using rayon): {:.2?}", elapsed);

    println!("**************************************************************************************");

    println!(" ");

    println!("************************ Solution - Parallel Version - Crossbeam ************************");

    let solutions = solve_par_crossbeam(board);

    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution - Parallel Crossbeam {}:\n{}", i + 1, crate::crossbeam_par_solver::format_board(solution));
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed time (parallel using crossbeam) : {:.2?}", elapsed);

    println!("**************************************************************************************");


    // Measure the time for the first non-parallel solver ( by Mungkorn )

    println!("*************************** Solution - Sequential Version 1 ***************************");
    let mut solver = SudokuSolver::new(board);
    let now = Instant::now();
    if solver.solve() {
        println!("Solution - Sequential :\n{}", solver);
    } else {
        println!("No solution exists.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed time (sequential version 1): {:.2?}", elapsed);

    println!("**************************************************************************************");


    // Measure the time for the second non-parallel solver
    println!("*************************** Solution - Sequential Version 2 ***************************");
    let now = Instant::now();
    let solutions = crate::nonpar_solver::solve_non_parallel(board);
    if solutions.is_empty() {
        println!("No solution found.");
    } else {
        for (i, solution) in solutions.iter().enumerate() {
            println!("Solution - Sequential  {}:\n{}", i + 1, format_board(solution));
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed time (sequential version 2):: {:.2?}", elapsed);

    println!("**************************************************************************************");

}

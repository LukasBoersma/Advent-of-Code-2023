// Advent of Code 2023 in Rust, by Lukas Boersma <mail@lukas-boersma.com>
// 
// These are my solutions to all 25 puzzles of Advent of Code 2023.
// See the README.md for more information, and the dayXX modules for
// my solutions.

#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(iter_map_windows)]
#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(slice_flatten)]

use std::env;

mod utils; 
pub use utils::*;
use utils::solution_import::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    // Load the solutions
    let solutions = solutions();
    let &latest_day = solutions.iter().map(|(day, _, _)| day).max().unwrap();

    // Parse the command line argument to get the selected day, or use the latest day
    let args: Vec<String> = env::args().collect();
    let selected_day = args.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(latest_day);

    // Get the solution for the selected day
    let solution = solutions.into_iter().find(|(d, _, _)| *d == selected_day).unwrap();
    
    // Run the solution
    run_solution_day(solution);
}
use std::fs;
use std::time::Instant;
use colored::Colorize;
use list_files_macro::list_files;
use regex::Regex;

pub type SolutionFn = dyn Fn(&str) -> i64;
pub type Solution = (u32, Box<SolutionFn>, Box<SolutionFn>);

// Loads a list of all solution functions, by searching for "day*.rs" files,
// loading them as modules, and wrapping the part1 and part2 functions in closures
pub fn solutions() -> Vec::<Solution> {
    let mut solutions: Vec::<Solution> = vec![];
    let solution_file_regex = Regex::new(r"day(\d+).rs$").unwrap();
    macro_rules! build_solution {
        ($file:expr) => {
            {
                let day_number = solution_file_regex.captures($file).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
                #[path = $file]
                mod day_solution;
                solutions.push((
                    day_number,
                    Box::new((|input: &str| day_solution::part1(input))),
                    Box::new((|input: &str| day_solution::part2(input))),
                ));
            }
        };
    }

    let _ = list_files!(build_solution, "../day*.rs");

    solutions
}

pub fn run_solution_part(part: u32, solution: &Box<SolutionFn>, input: &str) {
    // Runs the solution, measuring the time it takes
    let now = Instant::now();
    let result = solution(&input);
    let elapsed = now.elapsed();
    let elapsed_str = format!("({:.2?}ms)", elapsed.as_secs_f32() * 1000.0).dimmed();
    println!("Part {}: {} {}", part, result.to_string().yellow().bold(), elapsed_str);
}

pub fn run_solution_day(solution: Solution) {
    // Load the puzzle input
    let input = fs::read_to_string(format!("inputs/day{:02}.txt", solution.0)).expect("Unable to read puzzle input file");

    // Run the solution for both parts
    println!("Day {}", solution.0);
    run_solution_part(1, &solution.1, &input);
    run_solution_part(2, &solution.2, &input);
}
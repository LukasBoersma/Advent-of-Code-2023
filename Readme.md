# Advent of Code 2023 - Rust Solutions

This repository contains my solutions for [Advent of Code 2023](https://adventofcode.com/2023).

Advent of Code is a yearly programming challenge with one programming puzzle for each day from December 1 to 25.

The challenges start relatively easy and become increasingly difficult. 
The solutions often require implementations of a variety of computer science concepts, like dynamic programming or
search algorithms. That's why they are a great way to practice a new programming language, which I did here with Rust.

Each puzzle has two parts. Each part parses a string input and outputs an integer. The puzzle input is different for each
user, which is why the inputs are not included in this repository. There are example inputs, however, which are embedded
directly in the tests in each solution file.

I should stress that I was not incredibly experienced with Rust before this project, so this should definitely
not serve as a reference for how to write good Rust code.

## Interesting solutions

Since not all puzzles are equally difficult, the later days are generally more interesting than the first ones.

Some puzzles that I liked a lot this year:

 * [Day 8](src/day08.rs): The first puzzle this that required more than just implementing the rules. The puzzle was about
   counting the number of steps through a graph needed to get from a starting node to a goal node, while following a list
   of stepping directions. Part 2 uses multiple simultaneous positions and asks for the number of steps needed until all
   simultaneous positions are in a goal node. The solution required cycle detection and computing the 
   least common multiple of all cycles to get the number of steps.
 * [Day 9](src/day09.rs): Not a challenging puzzle, but my solution code is super short which makes me happy.
 * [Day 12](src/day12.rs): The puzzle provides a 2D map with incomplete information and asks for the number of different
   possibilities to fulfill the given constraints. My solution uses dynamic programming, and it took me weeks to come up
   with a solution.
 * [Day 24](src/day24.rs): The most difficult puzzle to solve for me, especially with my "no external solvers" house rule.
   Not only was this difficult to solve theoretically, but I also ran into numerical problems multiple times, with
   64-bit integers overflowing for many of the calculations needed. I ended up with brute forcing two of the unknown
   variables and solving the remaining unknowns from there.

## House Rules

To get scores on the leaderboards, it is enough to enter the correct number on adventofcode.com, but to maximize my
learning effect, I tried to follow these self-imposed rules:

1. **Solution must compute in less than 5 seconds.** Sometimes it is possible  to write an easy brute-force algorithm
   and let the program run for some hours. I always want to find an efficient solution. If the naive brute-force approach
   works fast enough, then of course it's still fine to use.
2. **No external solvers.** often the solution can be computed by using a SAT-solver or something similar, but I want to
   find a specific algorithm that solves each puzzle.
3. **Write a general solution whenever possible.** Sometimes the input that Advent of Code generated
   for me has some properties that I could exploit to solve a sub-class of the puzzle problem. But whenever possible, I
   want to solve the general problem as described in the puzzle. However, Advent of Code often has puzzles that are
   NP-hard, and the input is designed in a way that it can be solved fast. Part of the challenge is then to analyze the
   input to discover a way to solve it. If that happens, I documented it in the solution code.

## How to read the solutions

The `src/` folder contains a solution file for each day, named `dayXX.rs`.

The comments do not always explain the puzzle problem in detail, but I always have a link to the puzzle description at
the top of the file. Most of my solutions also have a short summary of the puzzle and my solution approach.

Each solution file contains the functions `part1` and `part2`, which take a string input and return an integer.
The solutions also contain tests, which run the example input from the puzzle description, and sometimes test other
things or special cases when I needed more test during development.

These are solutions for programming puzzles, so I don't aim to write maintainable code. I do, however, want to write
understandable code. That also means that there are lots of warnings in my code.
My solutions are often very compact, but (mostly) well documented. Understanding them requires basic knowledge of Rust
and computer science concepts.

## How to run the solutions

You will need [Rust](https://www.rust-lang.org/tools/install) installed, with the nightly toolchain enabled, version 1.77 or higher.

The easiest way is with `cargo test`, which will run all the tests with the example inputs (and the other tests, too).

To run the solution for your own input, you first need to get your input from https://adventofcode.com/2023
and create a file `inputs/day01.txt` (replace 01 with the actual day number). Caution: trailing line endings are not
supported.

With the input text file created, you can then run `cargo run -- 01` (again, replace 01 with the actual day number).

## Contact

If you have any questions, found a mistake, or want to talk to me for any other reason, please write to
[mail@lukas-boersma.com](mailto:mail@lukas-boersma.com).

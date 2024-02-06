/// Advent of Code 2023 - Day 14
/// https://adventofcode.com/2023/day/14
/// 
/// Here we get a 2D grid of empty space, obstacles, or moving rocks.
/// We are asked to move the rocks around and count the number of rocks in 
/// each row.
/// For each movement, we can move all rocks up, down, left, or right until they
/// hit an obstacle (or the edge of the map, or another rock).
/// 
/// For part 1, we just move the rocks up once.
/// 
/// For part 2, we are asked to find the number of rocks in each row after
/// 1 billion iterations of moving up, left, down, and right.
/// We do this by detecting the period of the rock movements and extrapolating
/// the result.
/// 
/// And yeah, there is duplicated code in the move_* functions, but generalizing
/// won't really make it more readable, I think, because the order of iterations
/// is different for each of them.

use std::collections::HashSet;
use crate::utils::*;

type Map = Vec<Vec<char>>;

const FLOOR: char = '.';
const OBSTACLE: char = '#';
const ROCK: char = 'O';

fn parse(input: &str) -> Vec<Vec<char>> {
    input.split("\n").map(|line| line.trim().chars().collect()).collect()
}

fn move_up(map: &mut Map) -> bool {
    let mut moved = false;
    for y in (0..map.len()-1) {
        for x in 0..map[y].len() {
            if map[y][x] == FLOOR && map[y+1][x] == ROCK {
                map[y][x] = ROCK;
                map[y+1][x] = FLOOR;
                moved = true;
            }
        }
    }
    moved
}

fn move_down(map: &mut Map) -> bool {
    let mut moved = false;
    for y in (1..map.len()).rev() {
        for x in 0..map[y].len() {
            if map[y][x] == FLOOR && map[y-1][x] == ROCK {
                map[y][x] = ROCK;
                map[y-1][x] = FLOOR;
                moved = true;
            }
        }
    }
    moved
}


fn move_left(map: &mut Map) -> bool {
    let mut moved = false;
    for x in (0..map[0].len()-1) {
        for y in 0..map.len() {
            if map[y][x] == FLOOR && map[y][x+1] == ROCK {
                map[y][x] = ROCK;
                map[y][x+1] = FLOOR;
                moved = true;
            }
        }
    }
    moved
}


fn move_right(map: &mut Map) -> bool {
    let mut moved = false;
    for x in (1..map[0].len()).rev() {
        for y in 0..map.len() {
            if map[y][x] == FLOOR && map[y][x-1] == ROCK {
                map[y][x] = ROCK;
                map[y][x-1] = FLOOR;
                moved = true;
            }
        }
    }
    moved
}


fn weight(map: &Map) -> I {
    let height = map.len();
    map.iter()
        .enumerate()
        .map(|(i, row)| (height - i) * row.iter().filter(|&&cell| cell == ROCK).count())
        .sum::<usize>() as I
}


pub fn part1(input: &str) -> I {
    let mut map = parse(input);
    while move_up(&mut map) {}
    weight(&map)
}

fn printmap(map: &Map) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

pub fn part2(input: &str) -> I {
    let mut map = parse(input);

    let mut seen_maps = HashMap::<Map, I>::new();

    let mut i = 0;
    let mut has_skipped  = false;
    while i < 1_000_000_000 {
        while move_up(&mut map) {}
        while move_left(&mut map) {}
        while move_down(&mut map) {}
        while move_right(&mut map) {}

        if !has_skipped && seen_maps.contains_key(&map) {
            let period = i - seen_maps[&map];
            let skip = (1_000_000_000 - i) / period;
            println!("Period {}, skipping {} cycles, from {} to {}", period, skip, i, i + skip * period);
            i += skip * period;
            has_skipped = true;
        }

        seen_maps.insert(map.clone(), i);

        if i % 10_000 == 0 {
            println!("{}", i);
        }
        i += 1;
    }

    weight(&map)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";

        assert_eq!(part1(example_input), 136);
        assert_eq!(part2(example_input), 64);
    }

    #[test]
    fn test_example_input3() {
        let mut map = parse(".OO");
        move_left(&mut map);
        assert_eq!(map, vec![vec!['O', 'O', '.']]);
    }

    #[test]
    fn test_example_input4() {
        let mut map = parse("OO.");
        move_right(&mut map);
        assert_eq!(map, vec![vec!['.', 'O', 'O']]);
    }

    #[test]
    fn test_example_input5() {
        let mut map = parse("\
        O
        O
        .");
        move_down(&mut map);
        assert_eq!(map, vec![vec!['.'],vec!['O'],vec!['O'],]);
    }

    #[test]
    fn test_example_input6() {
        let mut map = parse("\
        .
        O
        O");
        move_up(&mut map);
        assert_eq!(map, vec![vec!['O'],vec!['O'],vec!['.'],]);
    }


}
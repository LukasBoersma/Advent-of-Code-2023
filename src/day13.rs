/// Advent of Code 2023 - Day 13
/// https://adventofcode.com/2023/day/13
/// 
/// Here we have a 2D grid (cells are either '.' or '#')
/// and are asked to find a reflection line, i.e. a vertical or horizontal
/// symmetry axis.
/// 
/// For part 2, we are asked to find the "smudge", i.e. the single cell that,
/// when flipped, creates a different symmetry axis.
/// 
/// My solution just brute-forces all possible reflection lines
/// (and all possible cell flips for part 2).

use crate::utils::*;
type Map = Vec<Vec<char>>;

fn parse(input: &str) -> Vec<Map> {
    input
        .split("\n")
        .map(|line| line.trim())
        .vec()
        .split(|&line| line.len() == 0)
        .map(|pattern| {
            pattern.iter().map(|line| line.chars().vec()).vec()
        })
        .vec()
}

// Checks if row Y is a reflection line
fn is_reflected(map: &Map, y: I) -> bool {
    let up = (0 ..= (y as usize)).rev();
    let down = ((y as usize)+1..map.len());

    up.zip(down).find(|&(y1, y2)| map[y1] != map[y2]).is_none()
}

// Flips rows and columns
fn transpose(map: &Map) -> Map {
    let (w, h) = (map[0].len(), map.len());
    (0..w).map(|x| {
        (0..h).map(|y| {
            map[y][x]
        }).vec()
    }).vec()
}

// Returns the first reflection row != ignore_y, or None
fn try_find_reflection_row(map: &Map, ignore_y: I) -> Option<I> {
    (0..(map.len() as I)-1).find(|&y| (y+1) != ignore_y && is_reflected(map, y)).and_then(|y| Some(y+1))
}

// Returns the first reflection column != ignore_x, or None
fn try_find_reflection_col(map: &Map, ignore_x: I) -> Option<I> {
    try_find_reflection_row(&transpose(map), ignore_x)
}

pub fn part1(input: &str) -> I {
    let maps = parse(input);
    // Find the reflection lines and sum the indices (times 100 for rows)
    maps.iter()
        .map(|pattern| {
            try_find_reflection_row(pattern, -1)
                .and_then(|row| Some(row*100))
                .or_else(|| try_find_reflection_col(pattern, -1))
                .unwrap()
        })
        .sum()
}

// Returns all possible smudge variations of a map
fn iter_smudges(map: &Map) -> impl Iterator<Item=Map> + '_ {
    let (w, h) = (map[0].len(), map.len());
    // For each cell, return a map copy with that cell flipped
    (0..w).map(move |x| {
        (0..h).map(move |y| {
            // Copy the map, flip the cell at x,y, and return it
            let mut smudge_copy = map.clone();
            smudge_copy[y][x] = match smudge_copy[y][x] {
                '#' => '.',
                '.' => '#',
                _ => unreachable!()
            };

            smudge_copy
        })
    }).flatten()
}

pub fn part2(input: &str) -> I {
    let maps = parse(input);
    // Find the reflection lines after flipping a smudge cell, then same as part 1.
    maps.iter()
        .map(|pattern| {
            // Find the old reflection row and column (or None)
            let old_reflection_row = try_find_reflection_row(pattern, -1).unwrap_or(-1);
            let old_reflection_col = try_find_reflection_col(pattern, -1).unwrap_or(-1);

            // Iter over all smudges, search reflection lines for each, and return the first one
            // that is not None
            iter_smudges(pattern).map(|smudged_map| {
                try_find_reflection_row(&smudged_map, old_reflection_row)
                    .and_then(|row| Some(row*100))
                    .or_else(|| try_find_reflection_col(&smudged_map, old_reflection_col))
            }).flatten().next().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

        assert_eq!(part1(example_input), 405);
        assert_eq!(part2(example_input), 400);
    }

    #[test]
    fn test_p2_1() {
        let example_input = "\
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.";

        assert_eq!(part2(example_input), 300);
    }

    #[test]
    fn test_p2_2() {
        let example_input = "\
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

        assert_eq!(part2(example_input), 100);
    }


    #[test]
    fn test_row_detection() {
        let map = parse("#..#")[0].to_owned();

        assert_eq!(try_find_reflection_row(&map, -1), None);
        assert_eq!(try_find_reflection_col(&map, -1), Some(2));
    }

    #[test]
    fn test_smudge_detection() {
        {
            let map = vec![vec!['.', '.']];
            let smudge_options = iter_smudges(&map).vec();
            assert_eq!(smudge_options, vec![
                vec![vec!['#', '.']],
                vec![vec!['.', '#']]
            ]);
        }

        {
            let map = vec![vec!['.'], vec!['.']];
            let smudge_options = iter_smudges(&map).vec();
            assert_eq!(smudge_options, vec![
                vec![vec!['#'], vec!['.']],
                vec![vec!['.'], vec!['#']]
            ]);
        }
    }
}
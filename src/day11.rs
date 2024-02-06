/// Advent of Code 2023 - Day 11
/// https://adventofcode.com/2023/day/11
/// 
/// This puzzle provides a 2D map of galaxy positions. It asks for the sum of
/// the lengths of the shortest path between every pair of galaxies.
/// Before computing the distances, the space between the galaxies is expanded.
/// - In part 1, any row and column without any galaxy is expanded by a factor
///   of 2.
/// - In part 2, the expansion factor changes to 1000000. Since we are using
///   a sparse map of galaxy positions, this doesn't really change anything for us.


use crate::{utils::*, vec2::Vec2};

// The map is a list of galaxy positions
type Map = Vec<Vec2>;

// Returns the map and the map size
fn parse(input: &str) -> (Map, Vec2) {
    // Use enumerate to attach x and y to all # symbols, ignore the others
    let map = input
        .split("\n").enumerate().map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, symbol)| {
                match symbol {
                    '#' => Some(Vec2(x as I, y as I)),
                    _ => None
                }
            })
        })
        .flatten()
        .flatten()
        .vec();

    // Get the max x and y values
    let max = map.iter().fold((0, 0), |(max_x, max_y), &Vec2(x, y)| (max_x.max(x), max_y.max(y)));

    (map, max.into())
}

// Expand the row at the given y position by factor insert_len
fn insert_row(insert_y: I, insert_len: I, map: &Map) -> Map {
    map.map(|&Vec2(x, y)| Vec2(x, if y > insert_y { y+insert_len-1 } else { y })).vec()
}

// Expand the column at the given x position by factor insert_len
fn insert_col(insert_x: I, insert_len: I, map: &Map) -> Map {
    map.map(|&Vec2(x, y)| Vec2(if x > insert_x { x+insert_len-1 } else { x }, y)).vec()
}

fn solve(input: &str, expansion_size: I) -> I {
    let (mut map, size) = parse(input);

    // Expand rows and columns.
    // Go from high to low coordinates so that we don't look at already expanded space
    for row_y in (1..size.1).rev() {
        if !map.iter().any(|&Vec2(x,y)| y == row_y) {
            map = insert_row(row_y,expansion_size, &map);
        }
    }
    for col_x in (1..size.0).rev() {
        if !map.iter().any(|&Vec2(x,y)| x == col_x) {
            map = insert_col(col_x, expansion_size, &map);
        }
    }

    // Sum the distances between all pairs
    map.iter().combinations(2).map(|pair| {
        let d = *pair[0] - *pair[1];
        d.0.abs() + d.1.abs()
    }).sum()
}

pub fn part1(input: &str) -> I {
    solve(input, 2)
}

pub fn part2(input: &str) -> I {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....";

        assert_eq!(solve(example_input, 1), 374);
        assert_eq!(solve(example_input, 10), 1030);
    }
}
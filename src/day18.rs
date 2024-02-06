/// Advent of Code 2023 - Day 18
/// https://adventofcode.com/2023/day/18
/// 
/// This puzzle gives us a list of line drawing instructions and wants us to
/// draw lines on a 2D grid according to the instructions.
/// The puzzle asks for the area of the resulting polygon.
/// 
/// Part 1 could be brute forced by filling in a 2D vector, but for part 2,
/// what was previously presented as the "line colors" is now reinterpreted
/// as a distance code, making the area very large (10^13 cells in my case).
/// 
/// I solved this by building the polygon edges as (start, end) coordinate pairs
/// and then calculating the area of the polygon row-by-row, while skipping
/// identical rows. I take advantage of the fact that all edges are either
/// horizontal or vertical:
/// 
///     - I calculate the "area" of a single row by going over all edges
///       from left to right.
///     - Whenever I cross a horizontal line, we change between the inside and
///       outside of the polygon.
///     - Handling horizontal lines and corners of the polygon is more tricky,
///       see the code for details.
///     - Most of the rows contain only vertical lines (because there is only
///       a very limited number of lines in total). Those typically form
///       large groups of identical rows, so I can calculate the area for one
///       of the rows and then multiply it by the number of identical rows.
///     - I then sum the number of all row areas to get the total polygon area.

use crate::{utils::*, vec2::Vec2};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}
type Step = (Vec2, I, Orientation);
type SparseMap = Vec<(Vec2, Vec2, Orientation)>;

/// Parses the input with the interpretation from part 1
fn parse_part1(input: &str) -> Vec<Step> {
    input.lines().map(|line| {
        let items = parse::alphanums(line);
        let (direction, orientation) = match &items[0].chars().next().unwrap() {
            'R' => (Vec2(1, 0), Orientation::Horizontal),
            'L' => (Vec2(-1, 0), Orientation::Horizontal),
            'U' => (Vec2(0, -1), Orientation::Vertical),
            'D' => (Vec2(0, 1), Orientation::Vertical),
            _ => panic!("Invalid direction"),
        };
        (
            direction,
            items[1].parse::<I>().unwrap(),
            orientation,
        )
    }).vec()
}

/// Parses the input with the interpretation from part 2
/// (the hex code is the distance)
fn parse_part2(input: &str) -> Vec<Step> {
    input.lines().map(|line| {
        let items = parse::alphanums(line);
        let code = items[2].clone();
        // hex string to int
        let distance = I::from_str_radix(&code[..5], 16).unwrap();
        let (direction, orientation) = match code[5..].chars().next().unwrap() {
            '0' => (Vec2(1, 0), Orientation::Horizontal),
            '1' => (Vec2(0, 1), Orientation::Vertical),
            '2' => (Vec2(-1, 0), Orientation::Horizontal),
            '3' => (Vec2(0, -1), Orientation::Vertical),
            _ => panic!("Invalid direction"),
        };
        (
            direction,
            distance,
            orientation,
        )
    }).vec()
}

/// Builds the polygon by applying all drawing steps.
/// Returns a list of (start, end) coordinate pairs of polygon edges.
fn build_edges(plan: Vec<Step>) -> SparseMap {
    let mut pos = Vec2(0, 0);
    let mut map = vec![];
    for (direction, distance, orientation) in plan {
        let start = pos;
        let end = pos + (direction * distance);
        let min = Vec2(start.0.min(end.0), start.1.min(end.1));
        let max = Vec2(start.0.max(end.0), start.1.max(end.1));
        map.push((min, max, orientation));
        pos = end;
    }
    map
}

/// Returns true if there is any edge in the map that connects the given points
fn map_has_edge(map: &SparseMap, from: Vec2, to: Vec2) -> bool {
    map.iter().any(|&(min, max, _)| {
        (min == from && max == to) || (min == to && max == from)
    })
}

/// Calculates the area of the polygon by going over all rows and counting
/// the number of cells that are inside the polygon for that row.
/// See the module comment above for a summary of the algorithm.
fn area(map: SparseMap) -> I {
    // Get the min and max y coordinates that the polygon covers
    let min_y = map.iter().map(|(min,_, _)| min.1).min().unwrap();
    let max_y = map.iter().map(|(_,max, _)| max.1).max().unwrap();
    
    // Iterate over the y coordinate and accumulate the area row by row
    let mut area = 0i64;
    let mut y = min_y;

    while y <= max_y {
        // Get all vertical lines that intersect this row
        let lines = map.iter()
            .filter(|&&(min, max, orientation)| {
                orientation == Orientation::Vertical
                && min.1 <= y
                && max.1 >= y
            })
            .sorted_by_key(|(min, max, _)| (min.0 + max.0))
            .vec();

        // Accumulate the area for this row
        let mut line_area = 0i64;
        let mut has_horizontal_edge = false;

        let mut line_index = 0;
        // For each row, find the slices where we are inside the polygon
        while line_index < lines.len()-1 {
            let mut inside = false;
            // Trace the inside length until we leave the polygon

            let slice_min_x = lines[line_index].0.x();
            // Start with an empty slice. The loop below will advance slice_max_x until we leave the polygon.
            let mut slice_max_x = slice_min_x;

            // Iterate over the intersecting edges until we find the end of the current slice (i.e. we leave the polygon)
            loop {
                let &(min, max, _) = lines[line_index];
                
                // Does the vertical line just pass through or is one of the ends on this row?
                if min.y() != y && max.y() != y {
                    inside = !inside;
                    slice_max_x = max.x();
                } else {
                    has_horizontal_edge = true;
                    let &(next_min, next_max, _) = lines[line_index+1];
                    slice_max_x = next_max.x();
                    line_index += 1;

                    // Two possible cases:
                    // 1. Horizontal edge that forms a u-shape with the vertical lines
                    //  => we don't pass the polygon edge (no need to do anything)
                    // 2. Horizontal edge with both vertical lines pointing in different directions
                    //  => we pass the polygon edge, so we flip the inside flag
                    if map_has_edge(&map, Vec2(min.x(), y), Vec2(next_min.x(), y))
                        && (min.y() == y && next_max.y() == y) || (max.y() == y && next_min.y() == y) 
                    { 
                        inside = !inside;
                    }
                }

                // Stop if we left the polygon
                line_index += 1;
                if !inside || line_index > lines.len() - 1 { break; }
            }

            line_area += 1 + slice_max_x - slice_min_x;
        }

        // If this line had horizontal edges, the following lines will have different area.
        // Otherwise, we can skip the rows that have the same area.
        if has_horizontal_edge {
            y += 1;
            area += line_area;
        } else {
            // How many lines can we skip? Find the next row that contains any line start or end.
            // This is the point where the line area changes.
            let can_skip_until_y = map.iter()
                .map(|(min, max, _)| [min.y(), max.y()])
                .flatten()
                .filter(|&point_y| point_y > y)
                .min()
                .unwrap();
            let same_row_count = can_skip_until_y - y;
            area += line_area * same_row_count;
            y = can_skip_until_y;
        };
    }

    area
}

pub fn part1(input: &str) -> I {
    let plan = parse_part1(input);
    let map = build_edges(plan);
    area(map)
}

pub fn part2(input: &str) -> I {
    let plan = parse_part2(input);
    let map = build_edges(plan);
    area(map)
}

// test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";
        assert_eq!(part1(input), 62);
        assert_eq!(part2(input), 952408144115);
    }

    #[test]
    fn test_box() {
        let input = "\
        R 1 (#000000)
        D 1 (#000000)
        L 1 (#000000)
        U 1 (#000000)";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_box2() {
        /*
            ###
            #.#
            ###
        */
        let input = "\
        R 2 (#000000)
        D 2 (#000000)
        L 2 (#000000)
        U 2 (#000000)";
        assert_eq!(part1(input), 9);
    }

    #[test] fn test_parse_part2() {
        assert_eq!(parse_part2("R 6 (#70c710)"), vec![(Vec2(1, 0), 461937, Orientation::Horizontal)])
    }

    // #[test]
    // fn test_box2() {
    //     /*
    //         ###.###
    //         #.###.#
    //         #######
    //     */
    //     assert_eq!(part1(input), 9);
    // }
}
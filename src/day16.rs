/// Advent of Code 2023 - Day 16
/// https://adventofcode.com/2023/day/16
/// 
/// This puzzle gives us a 2D map of elements that manipulate a light beam.
/// The elements are mirrors and splitters:
///  - Mirrors reflect light by 90°.
///  - Splitters split light into two beams, one 90° to the left and one 90° to 
///    the right.
/// 
/// Part 1 asks for the number of lit tiles tiles (tiles that have any beam on it)
/// when the beam starts in the top left corner and goes to the right.
/// Part 2 asks for the maximum number of lit tiles that can be achieved by
/// letting the beam start at any of the outer edges of the map.

use std::collections::HashSet;
use crate::{utils::*, vec2::Vec2};

type Map = Vec<Vec<char>>;

/// Returns the input map as a 2D vector of chars
fn parse(input: &str) -> Map {
    input.split("\n").map(|line| line.trim().chars().vec()).vec()
}

/// (beam position, beam direction)
type Beam = (Vec2, Vec2);

/// Advances the beam by one step, returns a list of new beams.
/// When the beam hits a splitter, the result will contain two beams.
/// When the beam leaves the map, the result will be empty.
/// Otherwise the result will contain a single beam.
fn beam_step(beam: Beam, map: &Map) -> Vec<Beam> {
    let (pos, dir) = beam;
    // Advance the beam into its direction
    let new_pos = pos + dir;
    let (x, y) = (new_pos.0 as usize, new_pos.1 as usize);

    // Check if the beam is still on the map
    if new_pos.0 < 0 || new_pos.1 < 0 || x >= map[0].len() || y >= map.len() {
        // Beam is off the map: no output beam.
        return vec![];
    } else {
        // Beam is still on the map. The output beams depend on what is on the map.
        match map[y][x] {
            // Empty space: nothing happens, beam continues
            '.' => vec![(new_pos, dir)],

            // Mirrors: rotate beam direction by 90 degrees.
            '/' => vec![(new_pos, Vec2(-dir.1, -dir.0))],
            '\\' => vec![(new_pos, Vec2(dir.1, dir.0))],

            // Splitters: Output two beams, rotated by +90 and -90 degrees.
            // (unless the beam passes in the direction of the splitter axis)
            '|' => if dir.0 == 0 {
                        vec![(new_pos, dir)]
                    } else {
                        vec![
                            (new_pos, Vec2(0, 1)),
                            (new_pos, Vec2(0, -1))
                        ]
                    },
            '-' => if dir.1 == 0 {
                        vec![(new_pos, dir)]
                    } else {
                        vec![
                            (new_pos, Vec2(1, 0)),
                            (new_pos, Vec2(-1, 0))
                        ]
                    },
            _ => unreachable!()
        }
    }
}

/// Simulates the full beam path starting at the given beam.
fn simulate_beam(map: &Map, start: Beam) -> I {
    // Initially, we have one beam, but during the simulation it will be split
    // into multiple beams.
    let mut beams = vec![(start.0 - start.1, start.1)];

    // Keep track of the cells that are lit by the beam
    let mut lit_cells = HashSet::<Vec2>::new();

    // Beams will go in cycles, so we keep track of all beam position/direction
    // pairs that we have seen before. We ignore duplicate beams, so that we
    // can terminate the simulation at some point.
    let mut seen_beams = HashSet::<(Vec2, Vec2)>::new();

    // Keep simulating until we don't see any new beams anymore
    while beams.len() > 0 {
        // Simulate all beams and filter out the ones that we have seen before
        beams = beams.iter()
            .map(|beam| beam_step(*beam, &map))
            .flatten()
            .filter(|beam| seen_beams.insert(*beam))
            .vec();

        // Add all new beam positions to the set of lit cells
        lit_cells.extend(beams.iter().map(|(pos, _)| pos));
    }

    // Return the number of lit cells
    lit_cells.len() as I
}

/// Part 1: Simulate the beam starting at (0,0) going to the right,
/// return the number of lit cells.
pub fn part1(input: &str) -> I {
    let map = parse(input);
    simulate_beam(&map, (Vec2(0,0), Vec2(1,0)))
}

/// Part 2: Find the beam starting position that lights the most cells,
/// return that maximum number of lit cells.
pub fn part2(input: &str) -> I {
    let map = parse(input);

    // All possible starting positions
    let possible_starts = [
        (0..map[0].len()).map(|x| (Vec2(x as I, 0), Vec2(0, 1))).vec(),
        (0..map[0].len()).map(|x| (Vec2(x as I, map.len() as I - 1), Vec2(0, -1))).vec(),
        (0..map.len()).map(|y| (Vec2(0, y as I), Vec2(1, 0))).vec(),
        (0..map.len()).map(|y| (Vec2(map[0].len() as I - 1, y as I), Vec2(-1, 0))).vec(),
    ];
    
    // Simulate the beam for all possible starting positions and return the
    // maximum number of lit cells.
    possible_starts.iter()
        .flatten()
        .map(|beam| simulate_beam(&map, *beam))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = r".|...\....
                      |.-.\.....
                      .....|-...
                      ........|.
                      ..........
                      .........\
                      ..../.\\..
                      .-.-/..|..
                      .|....-|.\
                      ..//.|....";
        assert_eq!(part1(input), 46);
        assert_eq!(part2(input), 51);
    }
}
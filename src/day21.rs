/// Advent of Code 2023 - Day 21
/// https://adventofcode.com/2023/day/21
/// 
/// Today we got a 2D map of walkable tiles and obstacles,
/// and are asked for the number of reachable tiles after a given number of
/// steps.
/// 
/// Part 1 asks for the number of reachable tiles after 64 steps.
/// 
/// Part 2 asks for the number of reachable tiles after 26501365 steps, and 
/// reinterprets the map to repeat infinitely in all directions.
/// 
/// The solution number is again too large for a naive implementation.
/// At least in my case, the input map did not have any obstacles along the
/// axes from the start position, so I knew that the centers of the neighboring
/// repeating map parts could be reached directly, and which tiles can be reached
/// from there will repeat every n steps, with n being the map size.
/// For my solution, I just calculated the number of reachable tiles for a few
/// step numbers and used the lagrange interpolating polynomial to extrapolate.

use std::collections::VecDeque;

use crate::{utils::*, vec2::Vec2};

/// Possible stepping directions
const DIRECTIONS: [Vec2; 4] = [
    Vec2(0, -1),
    Vec2(1, 0),
    Vec2(0, 1),
    Vec2(-1, 0),
];

/// The map, as a sparse set of obstacles
struct Map {
    pub start: Vec2,
    pub floor_tiles: HashSet<Vec2>,
    pub w: I,
    pub h: I,
}

/// Accessor methods for the map (for infinite tiling)
impl Map {
    fn wrap_pos(&self, pos: Vec2) -> Vec2 {
        let mut wrapped_pos = Vec2(pos.x() % self.w, pos.y() % self.h);
        if wrapped_pos.0 < 0 {
            wrapped_pos.0 += self.w;
        }
        if wrapped_pos.1 < 0 {
            wrapped_pos.1 += self.h;
        }
        wrapped_pos
    }

    fn is_floor(&self, pos: Vec2) -> bool {
        self.floor_tiles.contains(&self.wrap_pos(pos))
    }
}

/// Parses the map as a sparse set of obstacle positions
fn parse(input: &str) -> Map {
    let cells = input
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line
                .trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| (Vec2(x as I, y as I), c))
        })
        .flatten()
        .vec();

    Map {
        start: cells.iter().find(|(_, c)| *c == 'S').unwrap().0,
        floor_tiles: cells.iter().filter(|(_, c)| *c != '#').map(|(pos, _)| *pos).collect(),
        w: cells.iter().map(|(pos, _)| pos.x()).max().unwrap() + 1,
        h: cells.iter().map(|(pos, _)| pos.y()).max().unwrap() + 1,
    }
}

/// Naive implementation: BFS search for all reachable tiles
/// after walking the given number of steps.
/// The infinite_map flag is needed to switch between part 1 and 2.
fn solve_small(input: &str, steps: I, infinite_map: bool) -> I {
    let map = parse(input);
    let floor_tiles = map.floor_tiles.clone();

    // BFS to find all cells within the given number of steps
    let mut queue = VecDeque::<(Vec2, I)>::new();
    
    // The starting position can be reached with zero steps
    queue.push_back((map.start, 0));

    let mut positions = vec![];
    let mut visited = HashSet::new();

    let mut count = 0;

    // Explore while there are still walkable tiles in the queue
    while let Some((pos, dist)) = queue.pop_front() {
        // Did we get here by taking the needed number of steps? Then this is one of the reachable tiles.
        if dist == steps {
            positions.push(pos);
            count += 1;
        } else {
            for &dir in &DIRECTIONS {
                let new_pos = pos + dir;
                
                let is_floor = if infinite_map {
                    map.is_floor(new_pos)
                } else {
                    floor_tiles.contains(&new_pos)
                };

                if is_floor && !visited.contains(&(new_pos, dist + 1)) {
                    queue.push_back((new_pos, dist + 1));
                    visited.insert((new_pos, dist + 1));
                }
            }
        }
    }

    count
}

/// Solution for large step counts, uses solve_small to simulate a few steps
/// and extrapolates from there.
fn solve_large(input: &str, steps: I) -> I {
    let map = parse(input);

    if steps < map.w * 2 {
        return solve_small(input, steps, true);
    }

    println!("Map width: {}", map.w);
    let map_width_offset = steps % map.w;

    println!("Offset: {}", map_width_offset);

    // The input map has no obstacles along the axes from the start position.
    // Its width and height are the same.
    // So after walking map_length steps, we reach the starting position in the repeated map tiles.
    // the number of reachable tiles grows quadratically with the number of steps.
    // (at least on multiples of the map size)
    // So we compute three points using our bfs solver, and extrapolate the result.

    let known = [
        map_width_offset,
        map_width_offset + map.w,
        map_width_offset + 2 * map.w,
    ].map(|step_count| (step_count as i128, solve_small(input, step_count, true) as i128)).to_vec();

    println!("Known: {:?}", known);

    // Compute the lagrange interpolating polynomial:
    // https://mathworld.wolfram.com/LagrangeInterpolatingPolynomial.html
    let extrapolated_polynomial = |x: i128| {
        (0..3).map(|j| {
            let range = (0..3).filter(|&k| k != j);
            let a = range.clone().map(|k|(x - known[k].0)).product::<i128>();
            let b =  range.map(|k|(known[j].0 - known[k].0)).product::<i128>();
            (known[j].1 as i128 * a) / b
        }).sum::<i128>()
    };
    
    extrapolated_polynomial(steps as i128) as i64
}

/// Part 1: Reachable tiles after taking 64 steps
pub fn part1(input: &str) -> I {
    solve_small(input, 64, false)
}

/// Part 2: Reachable tiles after taking 26501365 steps
pub fn part2(input: &str) -> I {
    solve_large(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........";

            assert_eq!(solve_small(input, 1, false), 2);
            assert_eq!(solve_small(input, 2, false), 4);
            assert_eq!(solve_small(input, 6, false), 16);
    }

    #[test]
    fn test_free() {
        let input = "\
            ...
            .S.
            ...";

        assert_eq!(solve_small(input, 17, true), 324);
        assert_eq!(solve_large(input, 17), 324);
    }

    #[test]
    fn test_basic() {
        let input = "\
            ..#
            .S.
            ..#";

        assert_eq!(solve_small(input, 5, true), solve_large(input, 5));
        assert_eq!(solve_small(input, 13, true), solve_large(input, 13));
        assert_eq!(solve_small(input, 51, true), solve_large(input, 51));
    }

    #[test]
    fn test_basic2() {
        let input = "\
            .#...
            ...#.
            ..S..
            .#..#
            .#...";

        assert_eq!(solve_small(input, 5, true), solve_large(input, 5));
        assert_eq!(solve_small(input, 15, true), solve_large(input, 15));
        assert_eq!(solve_small(input, 13, true), solve_large(input, 13));
        assert_eq!(solve_small(input, 51, true), solve_large(input, 51));
    }

    #[test]
    fn test_open_axes() {
        let input = "\
            ...........
            ......##.#.
            .###..#..#.
            ..#.#...#..
            ....#.#....
            .....S.....
            .##......#.
            .......##..
            .##.#.####.
            .##...#.##.
            ...........";

            assert_eq!(solve_large(input, 6), solve_small(input, 6, true));
            assert_eq!(solve_large(input, 10), solve_small(input, 10, true));
            assert_eq!(solve_large(input, 62), solve_small(input, 62, true));
            assert_eq!(solve_large(input, 61), solve_small(input, 61, true));
            assert_eq!(solve_large(input, 60), solve_small(input, 60, true));
            assert_eq!(solve_large(input, 100), solve_small(input, 100, true));
    }
}
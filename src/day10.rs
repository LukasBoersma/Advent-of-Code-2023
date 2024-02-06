/// Advent of Code 2023 - Day 10
/// https://adventofcode.com/2023/day/10
/// 
/// Here we have a 2D map of pipes.
/// - Part 1 basically asks for the length of the pipe loop
///   (or half of it, to be precise).
/// - Part 2 asks for the number of cells that are inside the loop.
///   I solved with a flood-fill approach.

use crate::{utils::*, vec2::Vec2};

type Map = HashMap::<Vec2, Pipe>;
type Pipe = (char, Vec2);
const DIRS: [Vec2;4] = [
    Vec2(0,1),
    Vec2(0,-1),
    Vec2(1,0),
    Vec2(-1,0),
];

fn parse_pipes(input: &str) -> Vec<Pipe> {
    input.split("\n").enumerate().map(|(y, line)| {
        line.trim().chars().enumerate().map(move |(x, shape)| {
            (shape, Vec2(x as I, y as I))
        })
    }).flatten().collect()
}

fn parse(input: &str) -> (Map, Vec2) {
    // Parse the map into a hashmap of (shape, position) tuples, indexed by position
    let map = HashMap::<Vec2, Pipe>::from_iter(
        parse_pipes(input)
        .map(|&(shape, pos)| (pos, (shape, pos)))
    );
    // Find the start position
    let start = map.values().find(|&&(shape, _)| shape == 'S').unwrap().1;
    (map, start)
}

// Gets the two directions that a pipe can connect to
fn pipe_directions(shape: char) -> [Vec2;2] {
    match shape {
        '|' => [(0,1),(0,-1)],
        '-' => [(1,0),(-1,0)],
        'L' => [(1,0),(0,-1)],
        'J' => [(-1,0),(0,-1)],
        '7' => [(-1,0),(0,1)],
        'F' => [(1,0),(0,1)],
        'S' => [(0,0),(0,0)],
        _   => [(0,0),(0,0)],
    }.map(|p| p.into())
}

// Checks if a pipe can connect to a given incoming position
fn can_walk(from_pos: Vec2, (to_shape, to_pos): Pipe) -> bool {
    let incoming_dirs = pipe_directions(to_shape);
    let possible_incoming_positions = incoming_dirs.map(|p| p + to_pos);
    possible_incoming_positions.contains(&from_pos)
}

// Finds the loop in the map, returns the path and the furthest distance from the start
fn find_loop_and_distance(map: Map, start: Vec2) -> (Vec<Vec2>, I) {
    // From the start position, try to walk into all directions
    let mut paths = DIRS.iter()
        .map(|&dir| vec![start+dir])
        .filter(|p| map.contains_key(&p[0]) && can_walk(start, map[&p[0]])).vec();

    // Expand the four paths until we find a position that is already on a different path.
    // Then those two paths form the loop, and that position is the furthest point.
    loop {
        for i in 0..paths.len() {
            let path = &paths[i];
            // If the path is empty, skip it (we set dead ends to empty paths)
            if let Some(&pos) = path.last() {
                let (shape, _) = map[&pos];
                
                // Try to expand the path
                let maybe_next_pos = pipe_directions(shape).iter().map(|&dir| pos + dir).find(|new_pos| {
                    match map.get(new_pos) {
                        Some(&next_pipe) => can_walk(pos, next_pipe) && !path.contains(&next_pipe.1),
                        _ => false,
                    }
                });

                // If we were able to expand the path, check if the position is already on another path
                match maybe_next_pos {
                    Some(next_pos) => {
                        // Did we find our loop?
                        if let Some(connecting_path) = paths.iter().find(|other_path| other_path.contains(&next_pos)) {
                            // For the second path, skip the start position, reverse it, and concat both together
                            // Also add the start position to the front
                            return (
                                [
                                    vec![start],
                                    paths[i].clone(),
                                    connecting_path.iter().map(|&v| v).rev().vec(),
                                    vec![start],
                                ].concat().to_vec(),
                                1 + path.len() as I
                            );
                        }
                        paths[i].push(next_pos);
                    },
                    // If we could not find a direction into which we can expand the path, set the path to empty
                    _ => {
                        paths[i].clear();
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> I {
    let (map, start) = parse(input);
    find_loop_and_distance(map, start).1
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Side { Unknown, Left, Right, Path }

// For two points, get the two directions that point left/right from the direction defined by the two points
fn get_lr(from_pos: Vec2, to_pos: Vec2) -> [Vec2;2] {
    let d = (to_pos.0 - from_pos.0, to_pos.1 - from_pos.1);
    let [l, r] = match d {
        (0, 1) => [Vec2(1,0),Vec2(-1,0)],
        (0, -1) => [Vec2(-1,0),Vec2(1,0)],
        (1, 0) => [Vec2(0,-1),Vec2(0,1)],
        (-1, 0) => [Vec2(0,1),Vec2(0,-1)],
        _ => unreachable!(),
    };

    [l, r]
}

fn print_map(map: &HashMap<Vec2, Side>, pipemap: &Map) {
    let width = map.keys().map(|&Vec2(x, _y)| x).max().unwrap();
    let height = map.keys().map(|&Vec2(_x, y)| y).max().unwrap();

    for y in 0..height {
        for x in 0..width {
            let x = match map[&Vec2(x,y)] {
                Side::Path => match pipemap[&Vec2(x,y)] {
                    ('-', _) => "─".bright_blue(),
                    ('|', _) => "│".bright_blue(),
                    ('L', _) => "└".bright_blue(),
                    ('J', _) => "┘".bright_blue(),
                    ('7', _) => "┐".bright_blue(),
                    ('F', _) => "┌".bright_blue(),
                    ('S', _) => "S".on_yellow(),
                    _ => unreachable!(),
                },
                Side::Left => "L".on_red(),
                Side::Right => "R".on_green(),
                Side::Unknown => " ".on_white(),
            };
            print!("{}", x);
        }
        println!("");
    }
    println!("");
}

pub fn part2(input: &str) -> I {
    let (pipemap, start) = parse(input);
    let (path, _) = find_loop_and_distance(pipemap.clone(), start);
    let mut map = HashMap::<Vec2, Side>::from_iter(
        pipemap.iter().map(|(&pos, _)| (pos, Side::Unknown))
    );

    // Follow the path and mark all neighboring fields as L/R,
    // Unless they are part of the loop path, then mark as Path.
    for i in 1..path.len() {
        let from_pos = path[i-1];
        let pos = path[i];

        *map.get_mut(&pos).unwrap() = Side::Path;

        // Get the directions that point left/right from the current current path direction
        let [l, r] = get_lr(from_pos, pos);
        for (pos, side) in [
            (l+pos, Side::Left),
            (r+pos, Side::Right),
            (l+from_pos, Side::Left),
            (r+from_pos, Side::Right),
        ] {
            if let Some(value) = map.get_mut(&pos) && *value == Side::Unknown {
                *value = side;
            }
        }
    }

    print_map(&map, &pipemap);

    // Flood-fill the map by expanding L/R cells into unknown cells.
    // Repeat until nothing can be expanded anymore.
    let mut changed_something = true;
    while changed_something {
        changed_something = false;
        for &pos in pipemap.keys() {
            let &side = map.get(&pos).unwrap();
            if side == Side::Unknown {
                let neighbors = DIRS.map(|dir| dir+pos);
                for side in [Side::Right, Side::Left] {
                    if neighbors.iter().any(|p| match map.get(p) { Some(&neighbor_side) => side == neighbor_side, _ => false }) {
                        *map.get_mut(&pos).unwrap() = side;
                        changed_something = true;
                        break;
                    }
                }
            }
        }
    }

    print_map(&map, &pipemap);

    // Get the two sets of points that are on the left and right side of the path
    let left_points = map.iter().filter(|&(_, &side)| side == Side::Left);
    let right_points = map.iter().filter(|&(_, &side)| side == Side::Right);

    // To decide which side is inside, check if any of the points touch the boundary of the map
    // This fails if the path goes fully along the map boundary, but it works for all the inputs I saw
    let &left_min_x = left_points.clone().map(|(Vec2(x, _y), _)| x).min().unwrap();
    let &left_min_y = left_points.clone().map(|(Vec2(_x, y), _)| y).min().unwrap();
    if left_min_x == 0 || left_min_y == 0 {
        right_points.count() as I
    } else {
        left_points.count() as I
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            7-F7-
            .FJ|7
            SJLL7
            |F--J
            LJ.LJ";

        assert_eq!(part1(example_input), 8);
    }

    #[test]
    fn test_example_input_2() {
        let example_input = "\
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...";

        assert_eq!(part2(example_input), 8);
    }

    #[test]
    fn test_example_input_3() {
        let example_input = "\
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part2(example_input), 10);
    }
    
    #[test]
    fn test_example_input_4() {
        let example_input = "\
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........";

        assert_eq!(part2(example_input), 4);
    }
}
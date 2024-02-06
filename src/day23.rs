/// Advent of Code 2023 - Day 23
/// https://adventofcode.com/2023/day/23
/// 
/// Today's puzzle gives us a 2D maze grid, with narrow paths and intersections.
/// Some of the tiles are only accessible from one direction.
/// 
/// Part 1 asks for the length of the longest path from the top left to the
/// bottom right corner, while only accessing each tile once.
/// 
/// Part 2 is the same, but without the tile access restrictions (all tiles
/// can be accessed from any direction). The trick here is to not run the search
/// on the 2D grid, but to build the abstract graph formed by the intersections
/// and their connections, and run the search on that.

use crate::{utils::*, vec2::Vec2};

type Map = HashMap<Vec2, char>;

/// Parses the maze into a map of positions to tile types
fn parse(input: &str) -> Map {
    Map::from_iter(
        input.lines().filter(|line| line.trim().len() > 0).enumerate().map(|(y, line)| {
            line.trim().chars().enumerate().map(move |(x, c)| {
                (Vec2(x as I, y as I), c)
            })
        })
        .flatten()
        .filter(|&(_, c)| c != '#')
    )
}

/// Returns the accessible neighbors of a given position in the maze
/// (according to the part1 interpretation of the map)
fn get_neighbors(map: &Map, pos: Vec2, respect_slopes: bool) -> Vec<Vec2> {
    let tile = map.get(&pos.clone());
    let directions = if respect_slopes {
            match tile {
                Some('.') => vec![Vec2(1, 0), Vec2(-1, 0), Vec2(0, 1), Vec2(0, -1)],
                Some('v') => vec![Vec2(0, 1)],
                Some('>') => vec![Vec2(1, 0)],
                Some('^') => vec![Vec2(0, -1)],
                Some('<') => vec![Vec2(-1, 0)],
                _ => vec![],
            }
        } else {
            vec![Vec2(1, 0), Vec2(-1, 0), Vec2(0, 1), Vec2(0, -1)]
        };

    directions.into_iter().map(move |dir| pos + dir).filter(|pos| map.contains_key(pos)).vec()
}

/// Grid-based solution for finding the longest path
fn longest_path(map: &Map, start: Vec2, goal: Vec2, visited: &mut HashSet<Vec2>) -> Option<I> {
    get_neighbors(map, start, true).map(|&next_pos| {
        if next_pos == goal {
            Some(1)
        } else if visited.contains(&next_pos) {
            None
        } else {
            visited.insert(next_pos);
            let longest_length = longest_path(map, next_pos, goal, visited).map(|length| length + 1);
            visited.remove(&next_pos);
            longest_length
        }
    }).flatten().max()
}

/// Part 1: Find the longest path, while only accessing each tile once,
/// and accessing the "slope" tiles only from the correct direction.
pub fn part1(input: &str) -> I {
    let map = parse(input);

    // Find start and end (the only points in the top/bottom row)
    let &start = map.keys().min_by_key(|pos| pos.y()).unwrap();
    let &end = map.keys().max_by_key(|pos| pos.y()).unwrap();
    
    // Find the longest path and return its length.
    longest_path(&map, start, end, &mut HashSet::new()).unwrap()
}

/// Graph-based map: (intersection points => list of connected intersections)
type GraphMap = HashMap::<Vec2, Vec<(Vec2, I)>>;

/// Graph-based solution for finding the longest path
fn longest_path_graph(map: &GraphMap, start: Vec2, goal: Vec2, visited: &mut HashSet<Vec2>) -> Option<I> {
    let neighbors = map.get(&start).unwrap();
    neighbors.map(|&(next_pos, len)| {
        if next_pos == goal {
            Some(len)
        } else if visited.contains(&next_pos) {
            None
        } else {
            visited.insert(next_pos);
            let longest_length = longest_path_graph(map, next_pos, goal, visited).map(|length| length + len);
            visited.remove(&next_pos);
            longest_length
        }
    }).flatten().max()
}

fn build_graph_map(map: &Map) -> GraphMap {
    let &start = map.keys().min_by_key(|pos| pos.y()).unwrap();
    let &end = map.keys().max_by_key(|pos| pos.y()).unwrap();

    // Find the intersections of the maze.
    // We ignore dead ends, except for the start and end points.
    let crossings = map
        .keys()
        .copied()
        .filter(|&pos| get_neighbors(&map, pos, false).len() > 2 || pos == start || pos == end)
        .vec();

    // Build a graph of the maze, with the intersections as nodes and the
    // pathways between them as edges
    GraphMap::from_iter(
        // For each intersection point, get the connected intersections
        // and the length of the path between them.
        crossings.clone().into_iter().map(|crossing_point| {
                let neighbors = get_neighbors(&map, crossing_point, false);
                // Follow the four directions starting at the intersection,
                // until we find another intersection (or a dead end, which we ignore)
                let connected_crossings = neighbors.iter().map(|&neighbor| {
                    let mut prev = crossing_point;
                    let mut pos = neighbor;
                    let mut length = 1;
                    loop {
                        // Did we arrive at another intersection?
                        if crossings.contains(&pos) {
                            return Some((pos, length));
                        }
                        else {
                            // Otherwise, keep following the path
                            let neighbors = get_neighbors(&map, pos, false);
                            // Find the one accessible neighbor that is not the previous position
                            let maybe_next = neighbors.iter().filter(|&&next| next != prev).next();
                            if let Some(&next) = maybe_next {
                                prev = pos;
                                pos = next;
                                length += 1;
                            } else {
                                // No more neighbors, we reached a dead end
                                return None;
                            }
                        }
                    }
                }).flatten().vec();

                (crossing_point, connected_crossings)
        })
    )
}

/// Part 2: Find the longest path, while only accessing each tile once.
/// The slope tiles can now be accessed from any direction.
pub fn part2(input: &str) -> I {
    let map = parse(input);

    // Find start and end (the only points in the top/bottom row)
    let &start = map.keys().min_by_key(|pos| pos.y()).unwrap();
    let &end = map.keys().max_by_key(|pos| pos.y()).unwrap();
    
    // Build the maze graph
    let graph = build_graph_map(&map);

    // Find the longest path based on the graph, return its length
    longest_path_graph(&graph, start, end, &mut HashSet::new()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#";

            assert_eq!(part1(input), 94);
            assert_eq!(part2(input), 154);
    }
}
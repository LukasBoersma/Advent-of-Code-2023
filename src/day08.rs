/// Advent of Code 2023 - Day 08
/// https://adventofcode.com/2023/day/8
/// 
/// This puzzle is about a directed graph, with each node having two outgoing edges, "left" and "right".
/// The input is the graph plus a list of left/right instructions.
/// - Part 1: Just implement the puzzle desription:
///    - Start at the AAA node
///    - walk through the graph by cycling through the direction list, until we reach the ZZZ node
///    - return the number of steps taken
/// - Part 2: We still want to find the number of steps, but now we are in multiple positions simultaneously.
///   We start at all nodes that end with "A", we follow the direction list, applying each direction to all current nodes,
///   until we are only in nodes that end with "Z".
///   This does not work with the naive approach of just keeping track of the current position, the step count is very large.
///   Instead, we simulate the stepping for each starting node until we find a cycle
///   (i.e. we reach a node that we have already visited using the same index in the direction list).
///   The solution is then the least common multiple of the cycle lengths for each starting node.
///   This is not a general solution, but it turns out that each starting node only ever visits one target node,
///   and there is a clean single cycle for each one (if we would pass multiple different goal nodes, this would be
///   more complicated).

use crate::utils::*;
type Map = HashMap<String, (String, String)>;

fn parse(input: &str) -> (Vec<char>, Map) {
    let mut lines = input.split("\n");
    (
        // First line contains the directions (char array)
        lines.next().unwrap().trim().chars().vec(),
        // Other lines contain the map (node -> (left, right))
        Map::from_iter(lines.skip(1).map(|line| {
            let v = parse::alphanums(line);
            (v[0].to_string(), (v[1].to_string(), v[2].to_string()))
        }))
    )
}

fn step<'a>(pos: &str, dir: char, map: &'a Map) -> &'a str {
    let (l, r) = &map[pos];
    match dir {
        'L' => l,
        'R' => r,
        _ => unreachable!()
    }
}

pub fn part1(input: &str) -> I {
    let (directions, map) = parse(input);
    let dircycle = directions.iter().cycle();
    let mut steps = 0;
    let mut pos = "AAA";

    for &dir in dircycle {
        pos = step(pos, dir, &map);
        steps += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    steps
}

pub fn part2(input: &str) -> I {
    let (directions, map) = parse(input);

    let starts = map.keys().filter(|&node| node.ends_with("A")).vec();

    // For each starting position, find the cycle lengths for reaching target nodes
    let cycles = starts.map(|&start| {
        let mut cycle_lengths = Vec::<I>::new();
        let dircycle = directions.iter().enumerate().cycle();
        let mut steps = 0i64;
        let mut position: &str = start;
        let mut seen_states = Vec::<(&str, usize)>::new();

        // Step through the map until we find a position that we have seen at the
        // same direction index before.
        for (dir_index, &dir) in dircycle {
            position = step(position, dir, &map);

            steps += 1;

            if position.ends_with("Z") {
                let current_state = (position as &str, dir_index);
                if seen_states.contains(&current_state) {
                    break;
                } else {
                    seen_states.push(current_state);
                    cycle_lengths.push(steps);
                    steps = 0;
                }
            }
        }

        cycle_lengths
    });

    // Turns out that each starting node only ever visits one target node
    // and there is a clean single cycle for each one.
    // So we can just return the least common multiple of all the cycle lengths
    cycles
        .map(|lens| lens[0])
        .fold(1i64, |a, b| lcm(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(example_input), 6);
    }

    #[test]
    fn test_example_input_2() {
        let example_input = "\
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)";

        assert_eq!(part2(example_input), 6);
    }
}
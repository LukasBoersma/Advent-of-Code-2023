/// Advent of Code 2023 - Day 17
/// https://adventofcode.com/2023/day/17
/// 
/// This is a path finding puzzle.
/// 
/// The input is a 2D grid, with a costs value ("heat loss") for each cell.
/// The puzzle asks for the minimum cost for getting from the top left corner
/// to the bottom right corner.
/// 
/// - For part 1, we have the additional restriction that the path must not go
/// in the same direction for more than three steps in a row.
/// - Part 2 changes this limit to 10 steps. Additionally, we now have to move a
/// minimum of four steps into the same direction after changing direction.
/// It also has to reach the goal after going into the same direction for at
/// least 4 steps.
/// 
/// The solution uses Dijkstra to find the cheapest path. Adding the usual
/// optimistic distance heuristic does not help much because of the
/// same-direction restrictions.

use crate::{utils::*, vec2::Vec2};

const NEIGHBOR_DIRECTIONS: [Vec2; 4] = [
    Vec2(0, -1),
    Vec2(0, 1),
    Vec2(-1, 0),
    Vec2(1, 0)
];

type Map = Vec<Vec<I>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Node {
    pub pos: Vec2,
    pub previous_index: Option<usize>,
    pub same_dir_count: I,
    pub loss: I,
}

/// Parses the input grid into a vector of ints
fn parse_input(input: &str) -> Vec<Vec<I>> {
    input.lines().map(|line| {
        line.trim().chars().map(|c| {
            c.to_digit(10).unwrap() as I
        }).vec()
    }).vec()
}

/// Reconstructs the path from the closed list
fn reconstruct_path(node: &Node, closed: &Vec<Node>) -> Vec<Vec2> {
    let mut path = vec![node.pos];
    let mut cursor = node.clone();

    while let Some(index) = cursor.previous_index {
        cursor = closed[index];
        path.insert(0, cursor.pos);
    }

    path
}

/// Path finding, using Dijkstra
fn find_path(map: &Map, min_straight: I, max_straight: I) -> Vec<Vec2> {
    let width = map[0].len() as I;
    let height = map.len() as I;
    let goal = Vec2(width - 1, height - 1);

    let is_in_map = |Vec2(x, y): Vec2| x >= 0 && y >= 0 && x < width && y < height;
    
    // List of open nodes, i.e. nodes to be explored
    let mut open = vec![Node::default()];
    // List of closed nodes, i.e. nodes already explored. Kept for reconstructing the path
    let mut closed = Vec::<Node>::new();
    // List of visited nodes, for fast skipping of already visited nodes during exploration
    let mut visited = HashSet::<(Vec2, Option<Vec2>, I)>::new();

    // Keep exploring the open nodes until we reach the goal
    loop {
        // Get the node with the lowest heat loss
        let (node_index, &node) = open.iter().enumerate().min_by_key(|(_,node)| node.loss).unwrap();
        open.remove(node_index);
        // Add the node to the closed list (for reconstructing the path later)
        closed.push(node.clone());
        let node_index_in_closed = closed.len() - 1;

        // If we reach the goal, reconstruct the path and return it
        if node.pos == goal && node.same_dir_count >= min_straight-1 {
            return reconstruct_path(&node, &closed);
        }

        let previous_pos = node.previous_index.map(|prev_index| closed[prev_index].pos);
        let previous_dir = previous_pos.map(|previous_pos| node.pos - previous_pos);
    
        // Find all possible next nodes
        for &direction in &NEIGHBOR_DIRECTIONS {
            // Only produce neighbor nodes for inside the map
            if is_in_map(direction + node.pos) {
                let next_node = Node {
                    pos: node.pos + direction,
                    previous_index: Some(node_index_in_closed),
                    same_dir_count: if Some(direction) == previous_dir || previous_dir.is_none() { node.same_dir_count + 1 } else { 1 },
                    loss: node.loss + map[node.pos.1 as usize][node.pos.0 as usize]
                };

                // Consider this neighbor if:
                // - we are not going back to our old position, 
                // - we are either going straight or we can turn already (same_dir_count >= min_straight)
                // - we are not going straight too far.
                // - we have not visited this node before (we insert it while checking)
                if Some(next_node.pos) != previous_pos
                    && (node.same_dir_count >= min_straight || Some(direction) == previous_dir || previous_dir.is_none())
                    && next_node.same_dir_count <= max_straight
                    && visited.insert((next_node.pos, Some(node.pos), next_node.same_dir_count)) 
                {
                    // Everything ok? Then add the node to the open list
                    open.push(next_node);
                }
            }
        }
    }
}

/// Finds the minimum heat loss for getting from the top left corner to the bottom right corner,
/// with the given minimum and maximum steps that we can go in the same direction
/// (0 and 3 for part 1, 4 and 10 for part 2)
fn solve(input: &str, min_straight: I, max_straight: I) -> I {
    // Parse the map
    let map = parse_input(input);
    // Find the path
    find_path(&map, min_straight, max_straight).iter()
    // Skip the start node
    .skip(1)
    // get the heat loss of the remaining nodes
    .map(|&p| {
        map[p.1 as usize][p.0 as usize]
    })
    // Sum the heat loss
    .sum()
}

/// Part 1: Minimum 0, maximum 3 steps in the same direction
pub fn part1(input: &str) -> I {
    solve(input, 0, 3)
}

/// Part 2: Minimum 4, maximum 10 steps in the same direction
pub fn part2(input: &str) -> I {
    solve(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = "\
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533";

        assert_eq!(part1(input), 102);
        assert_eq!(part2(input), 94);
    }
}
/// Advent of Code 2023 - Day 25
/// https://adventofcode.com/2023/day/25
/// 
/// Today, we need to find the three edges in a given graph, that when removed,
/// split the graph into two separate graphs.
/// 
/// I just used the randomized Karger min cut algorithm. It needs to be run
/// several times to find the correct cut, but it's simple to implement and
/// works well enough here.
/// 
/// It is relatively slow, so this is the first and only time I am using
/// parallel processing. I just run the algorithm many times in parallel,
/// and I stop as soon as one of the random runs finds the solution.
/// It usually takes less than a second now.
/// 
/// There is no part 2 today, as this is the last puzzle, and part 2 consists
/// of clicking a button on the Advent of Code website.

use rand::Rng;

use crate::utils::*;
use rayon::prelude::*;

/// A node is identified by three chars (like "abc").
type Node = (char, char, char);

/// A graph is a map from nodes to their adjacent nodes
type Graph = HashMap<Node, Vec<Node>>;

/// Parses a node name
fn parse_node_id(name: &str) -> Node {
    (name.to_owned() + "__").chars()
        .next_tuple()
        .unwrap()
}

/// Parses the input graph
fn parse(input: &str) -> Graph {
    let items = input.lines().map(parse::alphanums).vec();
    let mut graph = HashMap::new();

    // Each line contains one node and **some** of its adjacent nodes
    // (adjacent nodes are not listed if they already list this node as adjacent)
    for item in items.iter() {
        let node_a = &item[0];
        // Connections are not listed for both directions, so add the edge to
        // the other node
        for node_b in item[1..].iter() {
            graph.entry(parse_node_id(node_a))
                 .or_insert(vec![])
                 .push(parse_node_id(node_b));
            
            graph.entry(parse_node_id(node_b))
                 .or_insert(vec![])
                 .push(parse_node_id(node_a));
        }
    }

    // Remove duplicates from the adjacency lists
    for (_, adjacent) in graph.iter_mut() {
        *adjacent = adjacent.iter().unique().cloned().vec();
    }

    graph
}

/// Finds a min cut candidate.
/// Uses Karger's algorithm for finding the minimum cut in a graph.
/// Returns the number of edges in the cut, and the two subgraphs sizes
pub fn karger_min_cut(graph: &Graph) -> (I, I, I) {
    let mut rng = rand::thread_rng();

    // For each node, in addition to the adjacency list, we keep a list of nodes
    // that have been merged into this node. Initially, the list contains the
    // node itself.
    let mut merged_graph = HashMap::<Node, (Vec<Node>, Vec<Node>)>::from_iter(
        graph.iter().map(|(node, adjacent)| (
            node.to_owned(),
            (adjacent.clone(), vec![node.to_owned()])
        ))
    );

    while merged_graph.len() > 2 {
        // Choose a node at random, to be merged into another node
        let merged_node = merged_graph.keys().nth(rng.gen_range(0..merged_graph.len())).unwrap().clone();
        // Remove it from the graph
        let (old_neighbors, previously_merged) = merged_graph.remove(&merged_node).unwrap();
        // Choose a neighbor at random
        let merge_into = old_neighbors[rng.gen_range(0..old_neighbors.len())].clone();

        // Merge the node into the neighbor
        let (mut new_merged_neighbors, mut new_merged_inner) = merged_graph.remove(&merge_into).unwrap();
        assert!(!new_merged_neighbors.contains(&merge_into));

        new_merged_inner.extend(previously_merged.into_iter());
        new_merged_neighbors.extend(old_neighbors.into_iter());
        new_merged_neighbors = new_merged_neighbors.into_iter().filter(|n| n != &merge_into && n != &merged_node).vec();
        assert!(!new_merged_neighbors.contains(&merge_into));
        merged_graph.insert(merge_into.clone(), (new_merged_neighbors, new_merged_inner));

        // Update all neighbors of the merged node to point to the neighbor instead
        for (_, (neighbors, _)) in merged_graph.iter_mut() {
            if neighbors.contains(&merged_node) {
                *neighbors = neighbors
                    .iter()
                    .map(|neighbor| if *neighbor == merged_node { &merge_into } else { neighbor })
                    .unique()
                    .cloned()
                    .vec();
            }
        }
    }

    // Get the two remaining nodes and reconstruct the subgraph information from
    // the "merged" lists in them
    let (node_a, node_b) = merged_graph.values().collect_tuple().unwrap();
    let (_, merged_a) = node_a;
    let (_, merged_b) = node_b;

    // The remaining edges are the ones that connect the two subgraphs
    let cut_edges = merged_a.iter().map(|node| {
        graph[node].iter()
            .filter(|&old_neighbor| merged_b.contains(old_neighbor))
    }).flatten().count() as I;

    (cut_edges, merged_a.len() as I, merged_b.len() as I)
}

/// Part 1: Find the three edges that, when removed, split the graph into two
/// separate graphs. Return the product of the sizes of the two subgraphs.
pub fn part1(input: &str) -> I {
    let graph = parse(input);
    (0..1000).par_bridge().map(|_| {
        let (cuts, subgraph_a, subgraph_b) = karger_min_cut(&graph);
        (cuts, (subgraph_a * subgraph_b) as I)
    }).find_any(|(cuts, _)| *cuts == 3).expect("No cut with three edges found").1
}

/// No part 2 today :)
pub fn part2(input: &str) -> I {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            jqt: rhn xhk nvd
            rsh: frs pzl lsr
            xhk: hfx
            cmg: qnr nvd lhk bvb
            rhn: xhk bvb hfx
            bvb: xhk hfx
            pzl: lsr hfx nvd
            qnr: nvd
            ntq: jqt hfx bvb xhk
            nvd: lhk
            lsr: lhk
            rzs: qnr cmg lsr rsh
            frs: qnr lhk lsr";

        assert_eq!(part1(input), 54);
    }

    #[test]
    fn test_small() {
        let input = "\
            la: lb lc ld
            lb: lc ld rb
            lc: ld rc
            ld: rd
            le: la lb lc ld
            ra: rb rc rd
            rb: rc rd
            rc: rd
            re: ra rb rc rd";

        assert_eq!(part1(input), 25);
    }

}
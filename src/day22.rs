/// Advent of Code 2023 - Day 22
/// https://adventofcode.com/2023/day/22
/// 
/// This puzzle gives us a list of 3D blocks of sand, and asks us to simulate
/// how they fall to the ground. There is a ground plane at Z=0.
/// Blocks are axis aligned boxes, defined by a minimum and maximum 3D position.
/// 
/// Part 1 asks for the number of blocks that, after all blocks have fallen down
/// and settled, could be removed without causing any other block to move.
/// 
/// Part 2 asks how many blocks would move if a single block X was removed, and
/// wants us to sum those numbers for all blocks X.
/// 
/// The implementation is straightforward, we just simulate the falling blocks.
/// The only trick is that we sort the blocks by their z position, and iterate
/// over the blocks in that order, so that when we determine how far a block
/// falls, any blocks below it have already fallen.

use crate::{utils::*, vec3::Vec3};

/// Blocks have a min and a max position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
struct Block { pub min: Vec3, pub max: Vec3, pub id: usize }

/// Parses the blocks
fn parse(input: &str) -> Vec<Block> {
    input.lines().enumerate().map(|(id, line)| {
        let (ax, ay, az, bx, by, bz) = parse::alphanums(line)
            .map(|num| num.parse::<I>().unwrap())
            .collect_tuple()
            .unwrap();

        Block {
            min: [ax.min(bx), ay.min(by), az.min(bz)].into(),
            max: [ax.max(bx), ay.max(by), az.max(bz)].into(),
            id
        }
    }).vec()
}

/// Tests if two blocks intersect in the xy plane
fn intersects_xy(a: &Block, b: &Block) -> bool {
    // test if a and b intersect in the xy plane
    a.min.x <= b.max.x     // a's left edge is left of b's right edge
    && a.max.x >= b.min.x  // a's right edge is right of b's left edge
    && a.min.y <= b.max.y  // a's top edge is above b's bottom edge
    && a.max.y >= b.min.y  // a's bottom edge is below b's top edge
}

/// Lets all blocks fall down as far as they can, and returns true iff any
/// block was moved. If check_only is true, it returns true iff any block
/// **would** be moved, but does not actually move any blocks.
fn gravity_step(blocks: &mut Vec<Block>, check_only: bool) -> bool {
    let mut changed_something = false;

    let max_z = blocks.iter().map(|other| other.max.z).max().unwrap();

    // Group blocks by their top z coordinate, for fast finding of the blocks
    // where other blocks will land on.
    let blocks_by_z = (0..=max_z).map(|z| {
        blocks.iter().cloned().filter(|block| block.max.z == z).collect_vec()
    }).vec();

    for i in 0..blocks.len() {
        let block = blocks[i].clone();

        // Find the closest Z coordinate below this block that contains a block where the
        // xy part intersects with the current block (so that the current block will fall onto it)
        let maybe_z_below = (1..block.min.z)
            .rev()
            .filter(|&z_below| {
                blocks_by_z[z_below as usize]
                    .iter()
                    .any(|other_block| intersects_xy(&block, other_block))
            })
            .next();

        if let Some(z_below) = maybe_z_below {
            // Found a block that is below the current one, and intersects it in the xy plane.
            // Unless the block is alredy directly above the block below, we move it down.
            let delta = block.min.z - z_below - 1;
            if delta > 0 {
                if check_only {
                    return true;
                }
                blocks[i].min.z -= delta;
                blocks[i].max.z -= delta;
                changed_something = true;
            }
        } else if block.min.z > 1 {
            // If no block is below this one, and it is not already at the bottom, move it to the ground.
            if check_only {
                return true;
            }
            let delta = blocks[i].min.z - 1;
            blocks[i].min.z -= delta;
            blocks[i].max.z -= delta;
            changed_something = true;
        }
    }

    changed_something
}

/// Part 1: How many blocks can be removed without moving any other block?
pub fn part1(input: &str) -> I {
    let mut blocks = parse(input);
    blocks.sort_by_key(|block| block.min.z);

    while gravity_step(&mut blocks, false) {}

    (0..blocks.len()).filter(|&i| {
        let mut with_block_removed = blocks.iter().enumerate().filter(|&(j, _)| j != i).map(|(_, b)| *b).vec();

        !gravity_step(&mut with_block_removed, true)
    }).count() as I
}

/// Part 2: How many blocks would move if a single block was removed?
/// Sum this number for all blocks.
pub fn part2(input: &str) -> I {
    let mut blocks = parse(input);
    blocks.sort_by_key(|block| block.min.z);

    while gravity_step(&mut blocks, false) {}

    (0..blocks.len()).map(|i| {
        // Make a copy of the block stack, with the i-th block removed
        let mut with_block_removed = blocks.clone();
        with_block_removed.remove(i);

        // Make a copy to compare with later
        let old_blocks = with_block_removed.clone();

        // Let the blocks fall down
        while gravity_step(&mut with_block_removed, false) {}

        // Count how many blocks moved
        old_blocks.iter()
            .filter(|old| !with_block_removed.contains(old))
            .count() as I
    }).sum::<I>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9";
        assert_eq!(part1(input), 5);
        assert_eq!(part2(input), 7);
    }

    #[test]
    fn test_example2() {
        let input = "\
        0,0,1~0,0,1
        1,0,2~1,0,2";
        assert_eq!(part1(input), 2);
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn test_example3() {
        let input = "\
        0,0,1~0,0,1
        0,0,2~0,0,2";
        assert_eq!(part1(input), 1);
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_example4() {
        let input = "\
        0,0,1~2,2,1
        2,2,2~3,3,2";
        assert_eq!(part1(input), 1);
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_example5() {
        let input = "\
        0,0,1~2,2,1
        2,2,2~3,3,2";
        assert_eq!(part1(input), 1);
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn test_parse() {
        let input = "\
        1,0,1~1,2,1
        0,0,2~2,0,2
        123,456,21~9876,543,789";

        let expected_blocks = vec![
            Block {
                min: [1, 0, 1].into(),
                max: [1, 2, 1].into(),
                id: 0
            },
            Block {
                min: [0, 0, 2].into(),
                max: [2, 0, 2].into(),
                id: 1
            },
            Block {
                min: [123, 456, 21].into(),
                max: [9876, 543, 789].into(),
                id: 2
            },

        ];

        assert_eq!(parse(input), expected_blocks);
    }
}
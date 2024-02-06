/// Advent of Code 2023 - Day 15
/// https://adventofcode.com/2023/day/15
/// 
/// This puzzle is a basic hash map implementation.
/// The puzzle defines the hash algorithm and a sequence of map operations.
/// The hash map is made up of 256 buckets ("boxes").
/// Each operation either inserts or removes a value ("lens") from the map.
/// The implementation is straightforward, there is a function to compute the
/// hash, and a function to apply a map operation.
///
///  - Part 1 asks for the sum of all hashed values
///  - Part 2 wants us to sort the values into buckets and calculate the
///    "focal power" of the lenses in the buckets.

use crate::utils::*;
use crate::utils::parse::*;

// Lenses are (label, focal length) pairs
type Lens = (String, I);
// Box is a list of lenses
type Boxes = Vec<Vec<Lens>>;

/// Hash function for a lens as defined in the puzzle
fn hash(input: &str) -> I {
    input.chars().fold(0i64, |acc, c| ((acc + (c as i64)) * 17) % 256)
}

fn apply_step(mut boxes: Boxes, step: &str) -> Boxes {
    // Parse the step
    let (label, op, focal_len) = (id, one_of(['=', '-']), opt(int)).parse(step).unwrap();
    let lensbox = &mut boxes[hash(&label) as usize];

    // Does the box contain a lens with the same label? Then replace or remove it
    // depending on the operation
    if let Some(index) = lensbox.iter().position(|(l, _)| l == &label) {
        if op == '=' {
            lensbox[index] = (label, focal_len.unwrap());
        } else {
            lensbox.remove(index);
        }
    } else if op == '=' {
        // No lens with this label yet, and op is '='? Then add a new lens
        lensbox.push((label, focal_len.unwrap()));
    }
    boxes
}

fn focal_power(boxes: &Boxes) -> I {
    // Focal power uses box index, lens index and focal length.
    // Iterate all lenses and return the sum of the focal powers
    boxes.iter().enumerate().map(|(box_index, lensbox)| {
        lensbox.iter().enumerate().map(move |(lens_index, (_, focal_len))| {
            (1+box_index as I) * (1+lens_index as I) * focal_len
        })
    }).flatten().sum()
}

/// Part 1: Sum of all hashes
pub fn part1(input: &str) -> I {
    input.split(",").map(hash).sum()
}

/// Part 2: Perform all the operations and calculate the focal power
pub fn part2(input: &str) -> I {
    let steps = input.split(",");
    // Start with 256 empty boxes and execute the steps on them
    let boxes = steps.fold(vec![vec![]; 256], apply_step);
    focal_power(&boxes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(example_input), 1320);
        assert_eq!(part2(example_input), 145);
    }
}
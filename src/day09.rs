/// Advent of Code 2023 - Day 09
/// https://adventofcode.com/2023/day/9
/// 
/// This puzzle provides a sequence of input numbers and asks us to extrapolate
/// the next number in the sequence.
/// This is done by computing discrete derivatives until the derivative is zero,
/// and integrating back up from there.
/// 
/// Part 2 is not really different from part 1, it just asks us to extrapolate
/// backwards instead of forwards.

use crate::utils::*;

fn extrapolate(values: Vec<i64>) -> (I, I) {
    // When all values are zero, return zeroes
    if values.iter().all(|&x| x == 0) {
        (0, 0)
    } else {
        // Otherwise get the differences between each pair of values and extrapolate from them
        let diffs = values.iter().map_windows(|[&a, &b]| b - a).vec();
        let (l, r) = extrapolate(diffs);
        (values[0] - l, values[values.len()-1] + r)
    }
}

fn solution(input: &str) -> (I,I) {
    input.split("\n")
        // Parse each line into a list of vectors
        .map(|l| l.split_whitespace().parse_i64().vec())
        // Extrapolate in both directions
        .map(extrapolate)
        // Sum left and right values independently
        .reduce(|a,b| (a.0+b.0, a.1+b.1))
        .unwrap()
}

pub fn part1(input: &str) -> I {
    solution(input).0
}

pub fn part2(input: &str) -> I {
    solution(input).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45";

        assert_eq!(solution(example_input), (2, 114));
    }
}
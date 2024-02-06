/// Advent of Code 2023 - Day 06
/// https://adventofcode.com/2023/day/6
/// 
/// For both parts, we need to find the number of ways to win.
/// - Part 1: There is a number of rounds, and we need to multiply the number of ways to win for each round.
/// - Part 2: We now interpret the numbers for each round as one single number, so we just remove the spaces from the
///   input and then use the same algorithm as for part 1. It sounds like the puzzle authors expected this to require
///   some smart solution, but just incrementing the number of different ways to win many times works fine
///   (less than a second)

use crate::utils::*;

fn ways_to_win((max_time, record_distance): (I, I)) -> I {
    let mut wins = 0i64;
    // There are 0..max_time ways to play.
    // We just check each way and increment the wins counter if we win.
    for t in 0..max_time {
        let speed = t;
        let remaining_time = max_time-t;
        let dist = remaining_time * speed;
        // We win if the traveled distance is greater than the record distance.
        if dist > record_distance {
            wins += 1;
        }
    }

    return wins;
}

pub fn part1(input: &str) -> I {
    let (times, distances) = input
        .lines()                    // For both lines:
        .map(|s| s
                .split(":")         // Split by ":"
                .last()             // Take the last part (containing the numbers)
                .unwrap()
                .trim()             // Remove whitespace
                .split_whitespace() // Split by whitespace
                .map(|s| s          // Parse each number>
                    .parse::<I>()
                    .unwrap()
                )
        ).pair();                   // Get the two lists of numbers as a pair

    // Build pairs for each round
    let rounds = times.zip(distances);
    // Compute the number of ways to win for each round, then multiply the results.
    rounds.map(ways_to_win).product()
}

pub fn part2(input: &str) -> I {
    // Just remove the spaces and use the same algorithm as for part 1
    part1(&input.replace(" ", ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            Time:      7  15   30
            Distance:  9  40  200";

        assert_eq!(part1(example_input), 288);
        assert_eq!(part2(example_input), 71503);
    }
}
/// Advent of Code 2023 - Day 12
/// https://adventofcode.com/2023/day/12
/// 
/// The dreaded spring puzzle! Took me way too long to solve this one.
/// The puzzle provides an incomplete map of springs that can either be
/// operational or broken. The states of some of the springs are unknown.
/// For each row in the map, it provides a list of constraints that must be satisfied,
/// in the form of a list of groups of consecutive damaged springs.
/// 
/// Both parts ask for the number of possible different arrangements of operational
/// and broken springs that meet the given criteria.
/// 
/// For part 1, this can easily be brute-forced, but part 2 asks us to concat 5 copies
/// of each row (and the constraints) and compute the number of different arrangements
/// for that larger map.
/// 
/// I ended up with a dynamic programming approach. It is described in full detail
/// below.

use memoize::memoize;

use crate::utils::*;

type I = usize;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Spring {
    Unknown,
    Operational,
    Broken,
}

type Constraints = Vec<I>;

type Row = (Vec<Spring>, Constraints);

fn parse_spring(c: char) -> Spring {
    match c {
        '.' => Spring::Operational,
        '#' => Spring::Broken,
        '?' => Spring::Unknown,
        _ => unreachable!(),
    }
}

fn parse_row(input: &str, multiply: I) -> Row {
    let (springs, constraints) = input.trim().split(" ").pair();
    (
        vec![springs]
            .repeat(multiply)
            .join("?")
            .chars()
            .map(parse_spring)
            .vec(),
        constraints
            .split(",")
            .map(|c| c.parse::<I>()
                .unwrap())
            .vec()
            .repeat(multiply)
    )
}

fn parse(input: &str, multiply: I) -> Vec<Row> {
    input.split("\n").map(|row| parse_row(row, multiply)).vec()
}

fn solve_row((row, constraints): Row) -> I {
    // Solve with two-dimensional dynamic programming over the number of groups and the maximum position of the last group

    // The memoization map is max_pos -> count: using only the first <group_index> groups, there are <count> ways to place them all in the first <max_pos> fields
    // We pass the map from group to group, so this initial map is for considering zero groups
    // There is exactly one way to place none of the groups considering an empty row, so we initialize with (0,1)
    constraints.iter().enumerate().fold(HashMap::<I, I>::from_iter([(0,1)]), |ways_to_solve, (group_index, group_size)| {
        // For each group, check all the possible positions to place it and build the memoized map for the next group
        // Find possible ways to solve the row for the previous groups plus this group
        ways_to_solve.iter().fold(HashMap::<I, I>::new(), |mut ways_to_solve, (&previous_max_pos, &previous_ways)| {
            // Consider all the possible positions to place this group after previous_max_pos
            // (previous_max_pos is the first index where none of the previous groups were placed)
            for pos in previous_max_pos..=(row.len() - group_size) {
                // Can the group be placed here?
                let can_place = 
                    row[pos..(pos + group_size)].iter().all(|&s| s != Spring::Operational)
                        // And if the row either ends after this group, or if it is followed by a possibly operational spring (otherwise it would form a group that is too large)
                        && (pos + group_size == row.len() || row[pos + group_size] != Spring::Broken)
                        // If the group is the last one, it can only be placed if no broken spring follows it (otherwise we have too many springs)
                        && (group_index < constraints.len() - 1 || row[(pos + group_size)..].iter().all(|&s| s != Spring::Broken));
                
                if can_place {
                    // If the group can be placed here, add the number of ways to solve the row for the previous groups to the number of ways to solve the row for the previous groups plus this group
                    let new_max_pos = pos + group_size + 1;
                    if ways_to_solve.contains_key(&new_max_pos) {
                        ways_to_solve.insert(new_max_pos, ways_to_solve[&new_max_pos] + previous_ways);
                    } else {
                        ways_to_solve.insert(new_max_pos, previous_ways);
                    }
                }

                // If there is a broken spring at this position, the group MUST be placed here or earlier, so we stop considering other positions
                // (otherwise the broken spring at this position would form its own group or make the next group too large when placing directly after)
                if row[pos] == Spring::Broken {
                    break;
                }
            }
            ways_to_solve
        })
    })
    .values()
    // We sum up all entries in the final map. Wach entry has the last group in a different position and is a different way of placing the groups.
    .sum()
}

pub fn part1(input: &str) -> i64 {
    parse(input, 1)
        .map(|row| solve_row(row.clone()))
        .sum::<I>() as i64
}

pub fn part2(input: &str) -> i64 {
    parse(input, 5)
        .map(|row| solve_row(row.clone()))
        .sum::<I>() as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        assert_eq!(part1(example_input), 21);
        assert_eq!(part2(example_input), 525152);
    }

    #[test]
    fn test_example_input_2() {
        let example_input = "? 1";

        assert_eq!(part1(example_input), 1);
        assert_eq!(part2(example_input), 1);
    }

    #[test]
    fn test_solve() {
        let rows = parse("? 1", 2);
        assert_eq!(solve_row(rows[0].clone()), 1);
    }


    #[test]
    fn test_example_input_3() {
        let example_input = "?? 1";

        assert_eq!(part1(example_input), 2);
    }

    #[test]
    fn test_example_input_4() {
        let example_input = "??.?? 1,1";

        assert_eq!(part1(example_input), 2 + 2);
    }

    #[test]
    fn test_example_input_42() {
        let example_input = "??.??.?? 1,1";
        assert_eq!(part1(example_input), 4 + 4 + 2 + 2);
    }

    #[test]
    fn test_example_input_5() {
        let example_input = "??? 1";

        assert_eq!(part1(example_input), 3);
    }

    #[test]
    fn test_example_input_6() {
        let example_input = "???? 1,1";

        assert_eq!(part1(example_input), 3);
    }

    #[test]
    fn test_solver_1() {
        assert_eq!(solve_row(parse("? 1", 1)[0].clone()), 1);
    }

    #[test]
    fn test_solver_2() {
        assert_eq!(solve_row(parse("?? 1", 1)[0].clone()), 2);
    }

    #[test]
    fn test_solver_3() {
        assert_eq!(solve_row(parse("??? 1", 1)[0].clone()), 3);
    }

    #[test]
    fn test_solver_4() {
        assert_eq!(solve_row(parse("??.?? 1,1", 1)[0].clone()), 4);
    }

}
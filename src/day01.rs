/// Advent of Code 2023 - Day 01
/// https://adventofcode.com/2023/day/1
/// 
/// For the first part, we need to sum the numbers formed by the first and last digit of each line
/// The second part is essentially the same, but with spelled out digits ("three" instead of "3").
/// Because the spelled out digits can overlap ("eightwo"), I dodn't bother writing a parser that can
/// handle overlapping tokens and instead just inject literal digits into the words ("eightwo" becomes "e8ght2o")
/// and then use the same method as in part 1.

use crate::utils::*;

/// Extracts the number from each line (formed by first and last digits)
/// and returns the sum of all numbers
fn sum_numbers(input: &str) -> I {
    input.lines().map(|line| {
        // Get the digits in this line
        let digits = line.trim().chars().filter(|c| c.is_digit(10)).vec();
        // Put the first and last digits together, then parse it as an integer
        [digits.first().unwrap(), digits.last().unwrap()]
            .into_iter()
            .collect::<String>()
            .parse::<I>()
            .unwrap()
    }).sum()
}

pub fn part1(input: &str) -> I {
    sum_numbers(input)
}

pub fn part2(input: &str) -> I {
    // No point in writing a parser that can handle "eightwo".
    // Just put the digits in there and use the same method as in part 1.
    let input_fixed = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "th3ee"),
        ("four", "f4ur"),
        ("five", "f5ve"),
        ("six", "s6x"),
        ("seven", "se7en"),
        ("eight", "ei8ht"),
        ("nine", "n9ne"),
    ].iter().fold(input.to_owned(), |s, (search, replace)| s.replace(search, replace).to_string());

    sum_numbers(&input_fixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "\
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        assert_eq!(part2(input), 281);
    }
}
/// Advent of Code 2023 - Day 04
/// https://adventofcode.com/2023/day/4
/// 
/// Here we have a list of lottery tickets, each with a list of ticker numbers and winning numbers.
///  - Part 1: For each ticket, count the number of winning numbers and compute the score: 2^(n-1)
///  - Part 2: Winning n numbers on ticket k now cause us to get an additional copy of each of the tickets (k+1..k+n)
///    We need to compute the total number of tickets that we get.

type I = i64;
type Card = (Vec<I>, Vec<I>);

fn parse(input: &str) -> Vec<Card> {
    input
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let numbers = line.trim().split(":").skip(1).next().unwrap();
            let parts = numbers
                .split("|")
                .map(|parts| parts
                    .trim()
                    .split_whitespace()
                    .map(|num| num.parse::<I>().unwrap())
                    .collect::<Vec<_>>()
                )
                .collect::<Vec<_>>();
            (parts[0].clone(), parts[1].clone())
        })
        .collect::<Vec<_>>()
}

fn winning_count((winning, own): &Card) -> I {
    winning.iter().filter(|n| own.contains(n)).count() as I
}

pub fn part1(input: &str) -> I {
    let cards = parse(input);
    let winning_counts = cards.iter().map(winning_count);
    let scores = winning_counts.map(|n| {
        if n > 0 {
            (2 as I).pow(n as u32 - 1)
        } else {
            0
        }
    });
    scores.sum()
}

pub fn part2(input: &str) -> I {
    let cards = parse(input);
    let winning_counts = cards.iter().map(winning_count).collect::<Vec<_>>();

    let mut card_counts = cards.iter().map(|_| 1).collect::<Vec<_>>();

    let mut total_count = 0;

    for i in 0..cards.len() {
        let count = card_counts[i];
        let winning = winning_counts[i];
        total_count += count;
        for j in i+1..=i+(winning as usize) {
            if j < card_counts.len() {
                card_counts[j] += count;
            }
        }
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part1(example_input), 13);
        assert_eq!(part2(example_input), 30);
    }
}
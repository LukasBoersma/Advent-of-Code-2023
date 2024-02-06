/// Advent of Code 2023 - Day 02
/// https://adventofcode.com/2023/day/2
/// 
/// The input contains a list of games, each with a list of hands.
/// Each hand can contain any number of cubes that can be colored red, green, or blue.
/// Both parts require the maximum number of cubes per color seen in any hand of each game:
///  - Part 1: Sum of the Game IDs where the maximum counts is above a threshold.
///  - Part 2: Sum of the products of the maximum counts.

use crate::{utils::*, parse::alphanums};

/// [red, green, blue]
type Hand = [I; 3];

/// (game id, hands)
type Game = (I, Vec<Hand>);

fn parse_hand(input: &str) -> Hand {
    // Input looks link "3 red, 5 green, 4 blue".
    // Split by comma, then split by space.
    let items = input
        .split(",")
        .map(|item| alphanums(item).pair())
        .map(|(count, color)| (color, count.parse::<I>().unwrap()))
        .vec();

    // If a color is not found in the input, it gets count zero
    let color_count = |color| items.iter().find(|(name, _)| name == color).map(|(_, n)| *n).unwrap_or(0);

    ["red", "green", "blue"].map(color_count)
}

fn parse(input: &str) -> Vec<Game> {
    // Input looks like "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    // Split by colon to separate the game id from the hands.
    // Then split by semicolons and parse each segment to get the hands.
    input
        .lines()
        .map(|line| {
            let (s_id, s_hands) = line.trim().split(":").pair();
            let id = s_id.split(" ").last().unwrap().parse::<I>().unwrap();
            let hands = s_hands
                .split(";")
                .map(parse_hand)
                .vec();
            (id, hands)
        })
        .vec()
}

fn get_min_counts(games: Vec<Game>) -> impl Iterator<Item = (I, Hand)> {
    // For each game, get the maximum count of each color seen in any hand of that game.
    games.into_iter().map(|(id, hands)| {
        (
            id,
            hands.into_iter().reduce(|[ar, ag, ab], [br, bg, bb]| [ar.max(br), ag.max(bg), ab.max(bb)]).unwrap()
        )
    })
}

pub fn part1(input: &str) -> I {
    // For each game, get the minimum count of cubes that must have been in the bag.
    // Then sum the game ids for all games where the maximum count is below the threshold (12, 13, 14).
    get_min_counts(parse(input))
        .filter(|&(_, [r, g, b])| r <= 12 && g <= 13 && b <= 14)
        .map(|(id, _)| id)
        .sum()
}

pub fn part2(input: &str) -> I {
    // For each game, get the minimum count of cubes that must have been in the bag.
    // Then sum the product of the minimum counts for all games.
    get_min_counts(parse(input))
        .map(|(_, hand)| hand.into_iter().product::<I>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part1(input), 8);
        assert_eq!(part2(input), 2286);
    }

}
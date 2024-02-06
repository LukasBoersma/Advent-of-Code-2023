/// Advent of Code 2023 - Day 07
/// https://adventofcode.com/2023/day/7
/// 
/// This puzzle is about comparing poker hands. There is no computer science challenge here, basically just implementing
/// the rules as described in the puzzle description:
/// - Part 1: Each hand wins an amount equal to its bid multiplied by its rank. What are the total winnings?
/// - Part 2: We still need to find the total winnings, but "J" now represents a joker card, and we need to replace each
///   joker card with the card that maximizes the hand's rank. We do this by just trying each replacement and taking the
///   maximum rank.

use std::cmp::Ordering;
use std::ops::Index;
use crate::utils::*;

type Card = char;
const CARD_ORDER: [char; 14] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '!'];

/// Returns the rank of a card
fn card_rank(card: Card) -> I {
    CARD_ORDER.iter().rev().position(|&v| v == card).unwrap() as I
}

/// Comparer for cards, uses card_rank
fn compare_card((card, other_card): (Card, Card)) -> Ordering {
    card_rank(card).cmp(&card_rank(other_card))
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn compare_hand_type(hand_type: HandType, other_type: HandType) -> Ordering {
    hand_type.partial_cmp(&other_type).unwrap()
}

fn detect_hand_type(cards: &Vec<Card>) -> HandType {
    let counts_by_type = CARD_ORDER.map(|c| (c, cards.iter().filter(|&&d| c == d).count())).to_vec();
    if counts_by_type.any(|(_, n)| *n == 5) {
        HandType::FiveOfAKind
    } else if counts_by_type.any(|(_, n)| *n == 4) {
        HandType::FourOfAKind
    } else if let Some((threePairCard, _)) = counts_by_type.iter().find(|(_, n)| *n == 3) {
        if counts_by_type.any(|(c, n)| *n == 2 && c != threePairCard) {
            HandType::FullHouse
        } else {
            HandType::ThreeOfAKind
        }
    } else if let Some((firstPairCard, _)) = counts_by_type.iter().find(|(_, n)| *n == 2) {
        if counts_by_type.any(|(c, n)| *n == 2 && c != firstPairCard) {
            HandType::TwoPair
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

type Hand = (HandType, Vec<Card>, I);

fn parse_hand(input: &str) -> Hand {
    let (cards_s, bid_s) = input.trim().split_whitespace().pair();

    let cards = cards_s.chars().collect::<Vec<_>>();

    (detect_hand_type(&cards), cards, bid_s.parse::<I>().unwrap())
}

/// Compares two hands according to the puzzle rules
fn compare_hand((hand_type, hand_cards, _): &Hand, (other_type, other_cards, _): &Hand) -> Ordering {
    match compare_hand_type(*hand_type, *other_type) {
        // If hand types are unequal, compare the cards pair by pair, find the first unequal ones, and return their ordering
        Ordering::Equal => hand_cards.iter()
            .zip(other_cards)
            .map(|(c,d)| compare_card((*c, *d)))
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal),
        x => x
    }
}

fn replace_joker_with(cards: &Vec<Card>, replacement: Card) -> Vec<Card> {
    cards.map(|&c| if c == '!' { replacement } else { c }).vec()
}

/// Finds the best hand type that can be achieved by replacing joker cards
fn max_joker_hand((hand_type, cards, bid): &Hand) -> Hand {
    let best_type = CARD_ORDER
        .iter()
        .map(|&joker_replacement| detect_hand_type(&replace_joker_with(cards, joker_replacement)))
        .max().unwrap();

    // We take the new hand type, but don't change the actual cards, so that ties are still resolved correctly.
    (best_type, cards.to_vec(), *bid)
}

// Returns the total winnings for a given input
fn play(input: &str) -> I {
    // Parse the hands
    let mut hands = input
        .split("\n")
        .map(parse_hand)
        .map(|hand| max_joker_hand(&hand))
        .vec();

    // Sort the hands by rank
    hands.sort_by(compare_hand);

    // Compute the total winnings (bid * rank index)
    hands.iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i+1) as I * bid)
        .sum()
}

pub fn part1(input: &str) -> I {
    play(input)
}

pub fn part2(input: &str) -> I {
    // Replace any "J" with "!", marking the J cards as jokers
    // (so we can still use the same static ranking for part 1 and 2)
    play(&input.replace("J", "!"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";

        assert_eq!(part1(example_input), 6440);
        assert_eq!(part2(example_input), 5905);
    }
}
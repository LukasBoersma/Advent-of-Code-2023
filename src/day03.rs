/// Advent of Code 2023 - Day 03
/// https://adventofcode.com/2023/day/3
/// 
/// The main challenge here is the parsing - after that, it's just a matter of
/// implementing the rules as described in the puzzle description:
///    - Part 1: Find all numbers with adjacent symbols and sum them
///    - Part 2: Find all stars with exactly two adjacent numbers,
///      multiply the two numbers and sum the results

type Row = Vec<char>;
type I = usize;

#[derive(Clone, Copy, PartialEq)]
struct PartNumber {
    pub value: I,
    pub y: I,
    pub min_x: I,
    pub max_x: I,
}

#[derive(Clone, Copy, PartialEq)]
struct Symbol {
    pub value: char,
    pub x: I,
    pub y: I,
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

/// Parses the first number in a row, with the minimum index given by min_x
fn parse_number(input: &Row, min_x: I, y: I) -> Option<PartNumber> {
    let range = 
        input.iter()
        .enumerate()
        .skip(min_x)
        .skip_while(|(_, c)| !c.is_digit(10))
        .take_while(|(_, c)| c.is_digit(10))
        .collect::<Vec<_>>();
    
    // No number found?
    if range.len() == 0 {
        None
    } else {
        let digits = range.iter().map(|(_,&c)| c);
        let indices = range.iter().map(|(i,_)| *i);

        Some(PartNumber {
            // Get the integer value by casting to a string and then parsing as int
            value: digits.collect::<String>().parse::<I>().expect("Can't parse part number"),
            y,
            // Min and max X coordinates
            min_x: indices.clone().min().unwrap(),
            max_x: indices.max().unwrap(),
        })
    }
}

/// Gets all numbers in a given row
fn parse_numbers_in_row(input: &Row, y: I) -> Vec<PartNumber> {
    let mut x = 0;
    let mut numbers = Vec::<PartNumber>::new();

    // Try reading a first number, then advance the x cursor so
    // that it is one char after the max_x value of the parsed number.
    // Repeat until no further number is found
    while let Some(new_num) = parse_number(input, x, y) {
        x = new_num.max_x + 1;
        numbers.push(new_num);
    }

    numbers
}

/// Parses all the numbers
fn get_numbers(input: &Vec<Row>) -> Vec<PartNumber> {
    input.iter()
        .enumerate()
        .map(|(y, row)| parse_numbers_in_row(row, y))
        .flatten()
        .collect()
}

/// Parses all symbols in a given row
fn parse_symbols_in_row(input: &Row, y: I) -> Vec<Symbol> {
    input.iter()
        .enumerate()
        .filter(|(_, &c)| is_symbol(c))
        .map(|(x, &c)| Symbol {value: c, x, y})
        .collect()
}

/// Parses all the Symbols, indexed by row
fn get_symbols_by_row(input: &Vec<Row>) -> Vec<Vec<Symbol>> {
    input.iter()
        .enumerate()
        .map(|(y, row)| parse_symbols_in_row(row, y))
        .collect()
}

/// Checks if given number and symbol are adjacent
fn is_adjacent(number: &PartNumber, symbol: &Symbol) -> bool {
    let dy = number.y.abs_diff(symbol.y);
    let dx = (number.min_x ..= number.max_x).map(|x| x.abs_diff(symbol.x)).min().unwrap();
    dy <= 1 && dx <= 1
}

/// Parses the board into two lists: numbers and symbols
fn parse(input: &str) -> (Vec<PartNumber>, Vec<Vec<Symbol>>) {
    let rows: Vec<Row> = input.lines().map(|line| line.chars().collect()).collect();
    (get_numbers(&rows), get_symbols_by_row(&rows))
}

/// Part 1 solution: Find numbers with adjacent symbols
pub fn part1(input: &str) -> i64 {
    let (part_numbers, symbols) = parse(input);
    let max_y = input.lines().count()-1;

    let parts_with_adjacent_symbol = part_numbers.iter().filter(|num| {
        let mut symbol_candidates = (&symbols[num.y.saturating_sub(1) ..= (num.y+1).min(max_y)]).iter().flatten();
        symbol_candidates.any(|sym| is_adjacent(num, sym))
    });

    parts_with_adjacent_symbol.map(|num| num.value).sum::<I>() as i64
}

/// Part 2 solution: Star symbols which have exactly two adjacent numbers
pub fn part2(input: &str) -> i64 {
    let (part_numbers, symbols) = parse(input);

    let stars_with_two_nums = symbols.iter()
        .flatten()
        .filter(|s| s.value == '*')
        .map(|s| {
            part_numbers.iter()
                .filter(|n| is_adjacent(n, s))
                .collect::<Vec<_>>()
        })
        .filter(|n| { n.len() == 2 });

    // Multiply each two numbers, then sum the results
    stars_with_two_nums
        .map(|n| { n[0].value * n[1].value })
        .sum::<I>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..";

        assert_eq!(part1(example_input), 4361);
        assert_eq!(part2(example_input), 467835);
    }
}
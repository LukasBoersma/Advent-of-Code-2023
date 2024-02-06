/// Advent of Code 2023 - Day 05
/// https://adventofcode.com/2023/day/5
/// 
/// This puzzle is about mapping ranges of numbers to other ranges of numbers.
///  - Part 1: Given a list of mapping rules and input numbers,
///    find the minimum output number after applying all mapping rules.
///  - Part 2: The task is still to find the minimum output number, but the inputs are now interpreted as intervals
///    instead of individual numbers. We solve this by mapping entire intervals. Since the mapping rules also cover 
///    intervals, the tricky part is to handle partially overlapping intervals correctly. A single interval may be
///    covered by multiple rules, and lead to multiple output intervals.

use crate::utils::*;

type MapRange = (I, I);
struct Map {
    ranges: Vec<(MapRange, MapRange)>,
    name: String
}

fn parse_range(input: &str) -> (MapRange, MapRange) {
    let items = input.split_whitespace().parse_i64().vec();
    let (destination_min, source_min, length) = (items[0], items[1], items[2]);
    ((source_min, source_min+length-1), (destination_min, destination_min+length-1))
}

fn parse_map(input: &Vec<String>) -> Vec<(MapRange, MapRange)> {
    input.map(|range| parse_range(range)).vec()
}

fn parse_maps(lines: Vec<String>) -> Vec<Map> {
    lines
        .split(|item| item.trim().len() == 0)
        .filter(|v| !v.is_empty())
        .map(|lines| {
            let lines_trimmed = lines.iter().map(|l| l.trim().to_string()).vec();
            let name = lines_trimmed[0].trim().split(" ").next().unwrap().to_string();
            Map { ranges: parse_map(&lines_trimmed[1..].to_vec()), name }
        })
        .vec()
}

fn do_ranges_overlap((from_min, from_max): MapRange, (to_min, to_max): MapRange) -> bool {
    assert!(from_min <= from_max && to_min <= to_max);
    from_max >= to_min && from_min <= to_max
}

/// Returns the intersected mapping range (or None if no intersection) and the leftover unmapped range (or None if fully contained)
fn intersect_ranges((a_min, a_max): MapRange, (b_min, b_max): MapRange) -> Option<(MapRange, Option<Vec<MapRange>>)> {
    if b_max < a_min || b_min > a_max { // no intersection
        None
    } else if b_min <= a_min && b_max >= a_max { // a fully contained in b
        Some(((a_min, a_max), None))
    } else if a_min < b_min && a_max <= b_max { // a is partially left of b
        Some(((b_min, a_max), Some(vec![(a_min, b_min-1)])))
    } else if a_max > b_max && a_min >= b_min { // a is partially right of b
        Some(((a_min, b_max), Some(vec![(b_max + 1, a_max)])))
    } else if a_min < b_min && a_max > b_max { // b is fully contained in a
        Some(((b_min, b_max), Some(vec![(a_min, b_min-1), (b_max+1, a_max), ])))
    } else {
        unreachable!();
    }
}

fn apply_map_range(map: &Map, range: MapRange) -> Vec<MapRange> {
    let mut unmapped_ranges = vec![range];
    let mut mapped_ranges = Vec::<MapRange>::new();

    while let Some(range) = unmapped_ranges.pop() {
        // find fitting map, apply the intersecting part of the input range, and add the remaining parts back to the unmapped list
        if let Some(&(map_from_range, map_to_range)) = map.ranges.iter().find(|&&(from_range, _)| do_ranges_overlap(range, from_range)) {
            let ((intersected_min, intersected_max), maybe_leftover) = intersect_ranges(range, map_from_range).unwrap();

            let offset = map_to_range.0 - map_from_range.0;
            mapped_ranges.push((intersected_min + offset, intersected_max + offset));
            if let Some(mut leftovers) = maybe_leftover {
                unmapped_ranges.append(&mut leftovers);
            }
        } else {
            // No range matching in map, so we "map" this range unmodified
            mapped_ranges.push(range);
        }
    }

    mapped_ranges
}

fn get_min_location(seed_ranges: Vec<MapRange>, maps: Vec<Map>) -> I {
    let mapped_ranges = seed_ranges.map(|&x| {
        //println!("Mapping {:?}", x);
        maps.iter().fold(vec![x], |from_ranges, map| {
            let mapped = from_ranges.map(|&range| apply_map_range(map, range)).flatten().vec();
            //println!("{} mapped {:?} to {:?}", map.name, from_ranges, mapped);
            mapped
        })
    }).flatten();

    mapped_ranges.map(|(min, _max)| min).min().unwrap()
}

pub fn part1(input: &str) -> I {
    let lines = input.split("\n").map(|l| l.to_string()).vec();

    let seed_ranges = lines[0]
        .split_whitespace()
        .skip(1)
        .parse_i64()
        .map(|v| (v, v))
        .vec();

    let maps = parse_maps(lines[1..].to_vec());
    get_min_location(seed_ranges, maps)
}

pub fn part2(input: &str) -> I {
    let lines = input.split("\n").map(|l| l.to_string()).vec();

    let seed_ranges = lines[0]
        .split_whitespace()
        .skip(1)
        .parse_i64()
        .array_chunks::<2>()
        .map(|c| (c[0], c[0] + c[1] - 1))
        .vec();

    let maps = parse_maps(lines[1..].to_vec());
    get_min_location(seed_ranges, maps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let example_input = "\
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4";

            assert_eq!(part1(example_input), 35);
            assert_eq!(part2(example_input), 46);
        }
}
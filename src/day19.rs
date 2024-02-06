/// Advent of Code 2023 - Day 19
/// https://adventofcode.com/2023/day/19
/// 
/// This puzzle is about applying conditions to "parts" (4-tuples of numbers).
/// The input consists of a list of "workflow" and a list of parts.
/// Each workflow consists of a list of rules.
/// Each rule is a condition that can be applied to one of the four numbers in an
/// part. When the condition matches, the part is sent to another workflow (defined by the rule).
/// If the condition does not match, the next rule in the workflow is checked.
/// Each workflow defines a default target workflow that an part is sent to
/// if none of the rules match.
/// Instead of another workflow, a rule can also send an part to "R" (reject)
/// or "A" (accept), which will terminate the rule checking process for that part.
/// 
/// Part 1 asks for the sum of all numbers in all parts that are accepted.
/// 
/// Part 2 asks for the number of distinct parts that are possibly accepted by
/// the workflows, given that all numbers are in the [1, 4000] range.
/// I solved this by implementing the rule simulation for intervals instead of
/// individual numbers, then running this new implementation on the input
/// interval [1, 4000].

use winnow::{stream::AsChar, ascii::alphanumeric1, token::take};
use crate::utils::{*, parse::id};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Property {
    X,
    M,
    A,
    S,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Part {
    pub x: I,
    pub m: I,
    pub a: I,
    pub s: I,
}

impl Part {
    pub fn get(&self, property: Property) -> I {
        match property {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }
    pub fn set(&mut self, property: Property, value: I) {
        match property {
            Property::X => self.x = value,
            Property::M => self.m = value,
            Property::A => self.a = value,
            Property::S => self.s = value,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Op {
    Lt,
    Gt,
}

struct Rule {
    pub property: Property,
    pub op: Op,
    pub value: I,
    pub send: String,
}

struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub default: String,
}

type Program = HashMap<String, Workflow>;

/// Parses a line in the input into a workflow definition
fn parse_workflow(input: &str) -> Workflow {
    let workflow_str = input.replace("}", "");
    let (name, rules) = workflow_str.split("{").pair();

    let mut rule_items = rules.split(",").collect_vec();
    let default = rule_items.pop().unwrap().to_owned();

    Workflow {
        name: name.to_owned(),
        default,
        rules: rule_items.iter().map(|rule_str| {
            let (property, op, value_str, _, send) = 
                (id, take(1usize), id, ":", id)
                .parse(rule_str)
                .unwrap();

            Rule {
                property: match property.as_str() {
                    "x" => Property::X,
                    "m" => Property::M,
                    "a" => Property::A,
                    "s" => Property::S,
                    _ => unreachable!(),
                },
                op: match op {
                    "<" => Op::Lt,
                    ">" => Op::Gt,
                    _ => unreachable!(),
                },
                value: value_str.parse::<I>().unwrap(),
                send: send.to_owned(),
            }
        }).vec()
    }
}

/// Parses a part definition (input looks like "{x=494,m=380,a=686,s=2820}")
fn parse_part(input: &str) -> Part {
    let mut part = Part { x: 0, m: 0, a: 0, s: 0 };
    let property_list = input.replace("{", "").replace("}", "");
    let items = property_list.split(',');
    for item in items {
        let (property, value_str) = parse::alphanums(item).pair();
        let value = value_str.parse::<I>().unwrap();
        match property.chars().next().unwrap() {
            'x' => part.x = value,
            'm' => part.m = value,
            'a' => part.a = value,
            's' => part.s = value,
            _ => unreachable!(),
        }
    }
    part
}

/// Parses the workflow and part lists
fn parse(input: &str) -> (Program, Vec<Part>) {
    let lines = input.lines().map(|line| line.trim()).vec();
    let (workflows, parts) = lines.split(|line| line.trim().is_empty()).pair();
    
    (
        HashMap::from_iter(workflows.iter().map(|workflow| {
            let workflow = parse_workflow(workflow);
            (workflow.name.clone(), workflow)
        })),
        parts.iter().map(|part| parse_part(part)).vec(),
    )
}

/// Evaluuates a rule condition on a given part
pub fn part_fulfills_rule(part: &Part, rule: &Rule) -> bool {
    match rule.op {
        Op::Lt => part.get(rule.property) < rule.value,
        Op::Gt => part.get(rule.property) > rule.value,
    }
}

/// Applies the workflow rules to a given part
pub fn process_part(part: &Part, program: &Program) -> bool {
    let mut workflow = &program["in"];
    loop {
        let mut send_to = &workflow.default as &str;
        for rule in &workflow.rules {
            if part_fulfills_rule(part, &rule) {
                send_to = &rule.send;
                break;
            }
        }
        match send_to {
            "R" => { return false; },
            "A" => { return true; },
            _ => { workflow = &program[send_to]; },
        }
    }
}

/// Part 1: Apply the workflow rules to the given parts and sum the numbers of
/// all accepted parts
pub fn part1(input: &str) -> I {
    let (program, parts) = parse(input);
    parts.iter()
        .filter(|part| process_part(part, &program))   // Get the accepted parts
        .map(|part| part.x + part.m + part.a + part.s) // Sum the numbers for each part
        .sum()                                         // Sum the sums of all parts
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PartRange {
    pub min: Part,
    pub max: Part,
}

/// For a given part range and rule, returns the subrange that fulfills the rule, and the one that does not
fn get_sub_range_for_rule(range: PartRange, rule: &Rule) -> (Option<PartRange>, Option<PartRange>) {
    let mut accepting_range = range.clone();
    let mut rejected_range = range.clone();

    // Since rule conditions are always "greater than" or "less than" operations,
    // the ranges are always split into two sub-ranges, at the value that the operator
    // compares against.
    match rule.op {
        Op::Lt => {
            accepting_range.max.set(rule.property, rule.value - 1);
            rejected_range.min.set(rule.property, rule.value);
        },
        Op::Gt => {
            accepting_range.min.set(rule.property, rule.value + 1);
            rejected_range.max.set(rule.property, rule.value);
        },
    }

    // Filter out empty ranges
    if accepting_range.min.get(rule.property) > accepting_range.max.get(rule.property) {
        (None, Some(range))
    } else if accepting_range == range {
        (Some(range), None)
    } else {
        (Some(accepting_range), Some(rejected_range))
    }
}

/// Calculate the number of distinct parts in the given range that are accepted by the given workflow
fn get_range_combinations(mut range: PartRange, workflow_name: &str, program: &Program) -> i64 {
    // "R" workflow always rejects
    if workflow_name == "R" {
        return 0;
    } else if workflow_name == "A" {
        // "A" workflow always accepts. The number of distinct accepted parts
        // is the product of the lengths of the ranges for each part property.
        let lengths = [
            range.max.x - range.min.x + 1,
            range.max.m - range.min.m + 1,
            range.max.a - range.min.a + 1,
            range.max.s - range.min.s + 1,
        ];

        return lengths.iter().product::<i64>();
    }
    
    let workflow = &program[workflow_name];

    // Accumulate the number of accepted combinations by splitting the ranges
    // into the sub-ranges that pass or fail the rule conditions, then applying 
    // the appropriate following rules for those sub-ranges (by applying the
    // workflow defined by the rule for the passing sub-range, and the next rule
    // in the current workflow for the failing sub-range)
    let mut accepted_combinations = 0i64;
    for rule in &workflow.rules {
        // Split into true/false ranges (might be None if they are empty)
        let (maybe_true_range, maybe_false_range) = get_sub_range_for_rule(range, rule);
        
        // The true-range is sent to the workflow defined by the rule
        if let Some(true_range) = maybe_true_range {
            accepted_combinations += get_range_combinations(true_range, &rule.send, program);
        }

        // The false-range will be applied to the next rule in the current workflow
        if let Some(false_range) = maybe_false_range {
            range = false_range;
        } else {
            // If the false range is empty, we can stop here
            return accepted_combinations;
        }
    }

    // Send the remaining range to the default workflow
    accepted_combinations += get_range_combinations(range, &workflow.default, program);

    // Return the number of accepted combinations
    accepted_combinations
}

/// Part 2: Calculate the number of distinct parts in the range [1, 4000] that
/// are accepted by the workflows
pub fn part2(input: &str) -> I {
    let (program, _) = parse(input);
    let initial_range = PartRange {
        min: Part { x: 1, m: 1, a: 1, s: 1 },
        max: Part { x: 4000, m: 4000, a: 4000, s: 4000 },
    };
    get_range_combinations(initial_range, "in", &program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}";

        assert_eq!(part1(input), 19114);
        assert_eq!(part2(input), 167409079868000);
    }
}
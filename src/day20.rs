/// Advent of Code 2023 - Day 20
/// https://adventofcode.com/2023/day/20
/// 
/// This puzzle gives us a network of logic gates and wants us to simulate it.
/// The network has a "button" as its input, and a single output gate "rx".
/// The logic gates operate on "high" and "low" signals. The gates can also not
/// output any signal at all (so in a way there three different output values).
/// 
/// Part 1 asks how many low pulses are emitted by all gates after pushing the
/// button 1000 times.
/// 
/// Part 2 asks how many times the button has to be pushed to get a low pulse
/// output from the "rx" gate. It needs a very a large number of button presses,
/// so we have to find a shortcut.
/// 
/// Solving this problem in general is NP-hard, but after analyzing my input
/// (used dot to visualize the network), I found that rx is connected to a
/// single gate that in turn is connected to a small set of gates that each have
/// something like a counter network behind them.
/// 
/// The counter networks have different cycle lengths, so I can simply simulate
/// the network until all counters have reached their cycle length, and then
/// return the least common multiple of all cycle lengths.
/// 
/// I always dislike when I can only solve a subclass of the problem, but
/// analyzing the input for clues is also fun. In this case, my solution should
/// at least work for any input that has one gate behind rx with a set of
/// conjunction gates behind it.

use std::collections::VecDeque;

use crate::utils::*;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum ModuleType {
    #[default]
    Broadcast,
    FlipFlop,
    Conjunction,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
struct Module {
    pub name: String,
    pub module_type: ModuleType,
    pub inputs: Vec<String>,
    pub input_values: Vec<bool>,
    pub outputs: Vec<String>,
}

impl Module {
    pub fn set_input(&mut self, from_module: &str, value: bool) {
        let index = self.inputs.iter().position(|x| x == from_module).unwrap();
        self.input_values[index] = value;
    }
}

type Circuit = HashMap<String, Module>;
// (from, to, value)
type Signal = (String, String, bool);

// Parsing functions
// Trying out parser combinators today. Nice if you want good error messages, but too much work for AoC.
mod parse {
    use super::*;
    use crate::utils::parse::*;

    pub fn module_type(input: &mut &str) -> PResult<ModuleType> {
        let type_symbol = opt(take_while(1, |c: char| !c.is_alphabetic())).parse_next(input)?;
        Ok(match type_symbol {
            Some("&") => ModuleType::Conjunction,
            Some("%") => ModuleType::FlipFlop,
            _ => ModuleType::Broadcast,
        })
    }

    pub fn connections(input: &mut &str) -> PResult<Vec<String>> {
        let connections: Vec::<&str> = separated(1.., take_while(1.., AsChar::is_alphanum), ", ").parse_next(input)?;
        Ok(connections.iter().map(|c| c.to_string()).vec())
    }

    pub fn module(input: &mut &str) -> PResult<Module> {
        let (module_type, name, _, outputs) = (
            module_type,
            id,
            " -> ",
            connections,
        ).parse_next(input)?;

        Ok(Module {
            module_type,
            name: name.to_string(),
            outputs,
            ..Default::default()
        })
    }

    pub fn circuit(input: &str) -> Circuit {
        let mut circuit = Circuit::from_iter(
            input
            .lines()
            .map(|line| module.parse(line.trim()).unwrap())
            .map(|module| (module.name.clone(), module))
        );

        // Connect outputs to the inputs
        for module in circuit.values().cloned().vec() {
            for output in &module.outputs {
                if let Some(to_module) = circuit.get_mut(output) {
                    to_module.inputs.push(module.name.clone());
                    to_module.input_values.push(false);
                }
            }
        }

        circuit.get_mut("broadcaster").unwrap().inputs.push("button".to_string());
        circuit.get_mut("broadcaster").unwrap().input_values.push(false);

        circuit
    }
}

fn process_signal(circuit: &mut Circuit, (from_module, to_module, value): &Signal) -> Option<bool> {
    if let Some(module) = circuit.get_mut(to_module) {
        match module.module_type {
            ModuleType::Broadcast => {
                Some(*value)
            },
            ModuleType::FlipFlop => {
                let state = module.input_values.get_mut(0).unwrap();
                if !value {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            },
            ModuleType::Conjunction => {
                module.set_input(&from_module, *value);
                if module.input_values.iter().all(|&x| x) {
                    Some(false)
                } else {
                    Some(true)
                }
            },
        }
    } else {
        None
    }
}

/// Part 1: simulate the circuit for 1000 button presses,
/// count the number of low pulses emitted by any gate.
pub fn part1(input: &str) -> I {
    let mut circuit = parse::circuit(input);

    let mut total_low_signals = 0i64;
    let mut total_high_signals = 0i64;

    for _ in 0..1000 {
        let mut signals = VecDeque::<Signal>::new();
        signals.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some(signal) = signals.pop_front() {
            let module_name = &signal.1;

            match signal.2 {
                true => total_high_signals += 1,
                false => total_low_signals += 1,
            }

            let new_signal = process_signal(&mut circuit, &signal);
            if let Some(new_signal_value) = new_signal {
                for output in circuit[module_name].outputs.clone() {
                    signals.push_back((module_name.clone(), output, new_signal_value));
                }
            }
        }
    }
    total_low_signals * total_high_signals
}

/// Part 2: Find the number of button presses required to get a low pulse from rx.
pub fn part2(input: &str) -> I {
    let mut circuit = parse::circuit(input);

    // The module graph has a single conjunction that sends to rx, with a second
    // layer of conjunctions that send to the first conjunction.
    // Each conjunction in the second layer has a different cycle period in which
    // they emit a high signal.
    // As soon as all of the second-layer conjunctions send a high signal,
    // the first conjunction will send a low signal to rx. To calculate the
    // number of button presses needed, we take the least common multiple of the
    // cycle periods of the second-layer conjunctions.

    // Get the conjunction that sends to rx
    let conjunction_to_rx = circuit.values()
        .find(|module| module.outputs.contains(&"rx".to_string()))
        .unwrap()
        .name
        .clone();

    // Get the conjunctions that send to the first conjunction
    let second_level_conjunctions = circuit.values()
        .filter(|module| module.outputs.contains(&conjunction_to_rx))
        .map(|m| m.name.clone())
        .vec();
    
    // Press the button repeatedly. When one of the second-level conjunctions sends a high signal, record the cycle period.
    // When we have all periods, return the least common multiple of them (the point where they all will send high at the same time)
    let mut periods_for_second_level = HashMap::<String, I>::new();
    for i in 1.. {
        let mut signals = VecDeque::<Signal>::new();
        signals.push_back(("button".to_string(), "broadcaster".to_string(), false));
        while let Some(signal) = signals.pop_front() {
            if signal.1 == "rx" && !signal.2 {
                return i;
            }

            let module_name = &signal.1;

            let new_signal = process_signal(&mut circuit, &signal);
            if let Some(new_signal_value) = new_signal {
                for output in circuit[module_name].outputs.clone() {
                    signals.push_back((module_name.clone(), output, new_signal_value));

                    // Is one of the second-level conjunctions sending a high signal?
                    // Then record the cycle period (the current number of iterations).
                    if second_level_conjunctions.contains(module_name) && new_signal_value {
                        periods_for_second_level.insert(module_name.to_string(), i);

                        // Do we have all periods together? Then we can return the LCM.
                        if periods_for_second_level.values().count() == second_level_conjunctions.len() {
                            let mut solution = 1i64;
                            for period in periods_for_second_level.values() {
                                solution = lcm(solution, *period);
                            }
                            return solution;
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "\
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";

        assert_eq!(part1(input), 11687500);
    }
}
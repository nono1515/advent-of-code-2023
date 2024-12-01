use day20::{lcm, Module, ModuleType};
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../input.txt");

    let mut modules = Module::parse_input(input);
    // println!("{:?}", modules);

    let mut count = 0;
    let mut periods = HashMap::new();

    'main_loop: loop {
        count += 1;

        let mut signals = VecDeque::new();
        signals.push_back(("", "broadcaster", false));
        while !signals.is_empty() {
            let (from, to, value) = signals.pop_front().unwrap();

            if modules[to].name == "lb" && value {
                // At first, I thought that there could be multiple periods for each
                // modules going to "lb" but there seems to be only one.
                // periods.entry(from).or_insert(vec![]).push(count);
                periods.insert(from, count);
            }
            if periods.len() == 4 {
                break 'main_loop;
            }

            match modules[to].type_ {
                ModuleType::FlipFlop => {
                    if !value {
                        modules.get_mut(to).unwrap().state = !modules[to].state;
                        for out in modules[to].outputs.iter() {
                            signals.push_back((to, *out, modules[to].state));
                        }
                    }
                }
                ModuleType::Conjunction => {
                    modules
                        .get_mut(to)
                        .unwrap()
                        .inputs
                        .iter_mut()
                        .for_each(|(input, state)| {
                            if input == from {
                                *state = value;
                            }
                        });
                    let out_signal = if modules[to].inputs.iter().all(|(_, state)| *state) {
                        false
                    } else {
                        true
                    };
                    for out in modules[to].outputs.iter() {
                        signals.push_back((to, *out, out_signal));
                    }
                }
                ModuleType::Broadcaster => {
                    for out in modules[to].outputs.iter() {
                        signals.push_back((to, *out, value));
                    }
                }
                ModuleType::Output => (),
            }
        }
    }

    let lcm = periods.values().fold(1, |acc, &p| lcm(acc, p as u64));
    println!("Part 2: {}", lcm);
}

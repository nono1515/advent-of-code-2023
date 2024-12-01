use day20::{Module, ModuleType};
use std::collections::VecDeque;

fn main() {
    let input = include_str!("../../input.txt");

    let mut modules = Module::parse_input(input);
    // println!("{:?}", modules);

    let mut low_count = 0;
    let mut high_count = 0;

    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_back(("", "broadcaster", false));
        while !signals.is_empty() {
            let (from, to, value) = signals.pop_front().unwrap();
            if value {
                high_count += 1
            } else {
                low_count += 1
            }
            // println!("{} {} {} ", from, to, value);
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
                ModuleType::Output => {}
            }
        }
    }

    println!("{} {}", low_count, high_count);
    println!("{}", low_count * high_count);
}

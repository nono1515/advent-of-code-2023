use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Output,
}

#[derive(Debug, Clone)]
pub struct Module<'a> {
    pub name: String,
    pub type_: ModuleType,
    pub inputs: Vec<(String, bool)>,
    pub outputs: Vec<&'a str>,
    pub state: bool,
}

impl Module<'_> {
    pub fn parse_input(input: &str) -> HashMap<String, Module> {
        let mut modules = HashMap::new();
        for line in input.lines() {
            let (module_str, outputs_str) = line.split_once(" -> ").unwrap();
            let mut outputs = vec![];
            for out in outputs_str.split(", ") {
                outputs.push(out);
            }
            let (name, type_) = match &module_str[..1] {
                "%" => (module_str[1..].to_string(), ModuleType::FlipFlop),
                "&" => (module_str[1..].to_string(), ModuleType::Conjunction),
                _ => (module_str.to_string(), ModuleType::Broadcaster),
            };

            modules.insert(
                name.clone(),
                Module {
                    name,
                    type_,
                    inputs: vec![],
                    outputs,
                    state: false,
                },
            );
        }

        let mut in_ = HashMap::new();
        for (name, module) in modules.iter() {
            for out in module.outputs.iter() {
                in_.entry(out.clone())
                    .or_insert(vec![])
                    .push((name.clone(), false));
            }
        }
        for (k, v) in in_ {
            if modules.contains_key(k) {
                modules.get_mut(k).unwrap().inputs = v;
            } else {
                modules.insert(
                    k.to_string(),
                    Module {
                        name: k.to_string(),
                        type_: ModuleType::Output,
                        inputs: v,
                        outputs: vec![],
                        state: false,
                    },
                );
            }
        }

        modules
    }
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

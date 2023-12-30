use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let input = include_str!("../../input.txt");

    let (workflows_str, _) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    for workflow in workflows_str.lines() {
        let (name, rules) = workflow.split_once("{").unwrap();
        workflows.insert(name, vec![]);
        for rule in rules[..rules.len() - 1].split(",") {
            workflows.get_mut(name).unwrap().push(Rule::parse(rule));
        }
    }

    let mut sum = 0;
    let mut todo: Vec<(&str, usize, [RangeInclusive<u32>; 4])> =
        vec![("in", 0, [1..=4000, 1..=4000, 1..=4000, 1..=4000])];

    while !todo.is_empty() {
        let (rule_name, rule_index, ranges) = todo.pop().unwrap();
        let rule = &workflows[rule_name][rule_index];

        for res in rule.apply(ranges, rule_name, rule_index) {
            if res.0 == "A" {
                sum += res.2.into_iter().fold(1, |acc, r| acc * r.count());
            } else if res.0 == "R" {
                ()
            } else {
                todo.push((res.0, res.1, res.2));
            }
        }
    }

    println!("sum: {}", sum);
}

#[derive(Debug)]
struct Rule {
    input: Option<char>,
    cond: Option<char>,
    threshold: Option<u32>,
    target: String,
}

impl Rule {
    fn parse(input: &str) -> Self {
        if input.contains(">") || input.contains("<") {
            let mut chars = input.chars().into_iter();
            let input = chars.next().unwrap();
            let cond = chars.next().unwrap();
            let rest = chars.collect::<String>();
            let (threshold, target) = rest.split_once(':').unwrap();
            let threshold = threshold.parse().unwrap();
            Rule {
                input: Some(input),
                cond: Some(cond),
                threshold: Some(threshold),
                target: target.to_string(),
            }
        } else {
            Rule {
                input: None,
                cond: None,
                threshold: None,
                target: input.to_string(),
            }
        }
    }

    fn apply<'a>(
        &'a self,
        [x, m, a, s]: [RangeInclusive<u32>; 4],
        rule_name: &'a str,
        rule_index: usize,
    ) -> Vec<(&str, usize, [RangeInclusive<u32>; 4])> {
        if let Some(mut n) = self.threshold {
            if self.cond == Some('>') {
                n = n + 1;
            }
            // let mut new_ranges;
            let new_range = match self.input {
                Some('x') => {
                    if x.contains(&self.threshold.unwrap()) {
                        (
                            [*x.start() as u32..=n - 1, m.clone(), a.clone(), s.clone()],
                            [n..=*x.end() as u32, m, a, s],
                        )
                    } else {
                        return vec![(self.target.as_str(), 0, [x, m, a, s])];
                    }
                }
                Some('m') => {
                    if m.contains(&self.threshold.unwrap()) {
                        (
                            [x.clone(), *m.start() as u32..=n - 1, a.clone(), s.clone()],
                            [x, n..=*m.end() as u32, a, s],
                        )
                    } else {
                        return vec![(self.target.as_str(), 0, [x, m, a, s])];
                    }
                }
                Some('a') => {
                    if a.contains(&self.threshold.unwrap()) {
                        (
                            [x.clone(), m.clone(), *a.start() as u32..=n - 1, s.clone()],
                            [x, m, n..=*a.end() as u32, s],
                        )
                    } else {
                        return vec![(self.target.as_str(), 0, [x, m, a, s])];
                    }
                }
                Some('s') => {
                    if s.contains(&self.threshold.unwrap()) {
                        (
                            [x.clone(), m.clone(), a.clone(), *s.start() as u32..=n - 1],
                            [x, m, a, n..=*s.end() as u32],
                        )
                    } else {
                        return vec![(self.target.as_str(), 0, [x, m, a, s])];
                    }
                }
                _ => panic!(),
            };

            match self.cond {
                Some('>') => {
                    vec![
                        (rule_name, rule_index + 1, new_range.0),
                        (self.target.as_str(), 0, new_range.1),
                    ]
                }
                Some('<') => {
                    vec![
                        (self.target.as_str(), 0, new_range.0),
                        (rule_name, rule_index + 1, new_range.1),
                    ]
                }
                _ => panic!(),
            }
        } else {
            vec![(self.target.as_str(), 0, [x, m, a, s])]
        }
    }
}

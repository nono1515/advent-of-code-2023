use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let piece_regex = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();

    let (workflows_str, pieces_str) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    for workflow in workflows_str.lines() {
        let (name, rules) = workflow.split_once("{").unwrap();
        workflows.insert(name, vec![]);
        for rule in rules[..rules.len() - 1].split(",") {
            workflows.get_mut(name).unwrap().push(Rule::parse(rule));
        }
    }

    // println!("workflows: {:?}", workflows);

    let mut sum = 0;

    for piece in pieces_str.lines() {
        let piece = piece_regex.captures(piece).unwrap();
        let x: u32 = piece[1].parse().unwrap();
        let m: u32 = piece[2].parse().unwrap();
        let a: u32 = piece[3].parse().unwrap();
        let s: u32 = piece[4].parse().unwrap();

        let mut curr_rules = &workflows["in"];
        let mut curr_index = 0;
        loop {
            let rule = &curr_rules[curr_index];
            if let Some(target) = rule.apply(&x, &m, &a, &s) {
                if target == "A" {
                    sum += x + m + a + s;
                    break;
                } else if target == "R" {
                    break;
                }
                curr_rules = &workflows[target];
                curr_index = 0;
            } else {
                curr_index += 1;
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

    fn apply(&self, x: &u32, m: &u32, a: &u32, s: &u32) -> Option<&str> {
        let n = match self.input {
            Some('x') => x,
            Some('m') => m,
            Some('a') => a,
            Some('s') => s,
            None => return Some(self.target.as_str()),
            _ => panic!(),
        };
        match self.cond {
            Some('>') => {
                if *n > self.threshold.unwrap() {
                    Some(&self.target)
                } else {
                    None
                }
            }
            Some('<') => {
                if *n < self.threshold.unwrap() {
                    Some(&self.target)
                } else {
                    None
                }
            }
            _ => panic!(),
        }
    }
}

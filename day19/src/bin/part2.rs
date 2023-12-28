use std::collections::HashMap;

fn main() {
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

    // let input = include_str!("../../input.txt");

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

    for x in 1..=4000 {
        println!("x: {}", x);
        for m in 1..=4000 {
            for a in 1..=4000 {
                for s in 1..=4000 {
                    let mut curr_rules = &workflows["in"];
                    let mut curr_index = 0;
                    loop {
                        let rule = &curr_rules[curr_index];
                        if let Some(target) = rule.apply(&x, &m, &a, &s) {
                            if target == "A" {
                                sum += 1;
                                break;
                            } else if target == "R" {
                                break;
                            }
                            // println!("{}", target);
                            curr_rules = &workflows[target];
                            curr_index = 0;
                        } else {
                            curr_index += 1;
                        }
                    }
                }
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

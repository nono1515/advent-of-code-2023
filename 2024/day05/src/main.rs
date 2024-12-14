use std::collections::HashMap;

fn part1(rules: &HashMap<u32, Vec<u32>>, operations: &str) -> u32 {
    let mut total = 0;

    for op in operations.lines() {
        let mut previous = vec![];
        let mut ok = true;
        for elem in op.split(',') {
            let num = elem.parse::<u32>().unwrap();
            if let Some(should_be_next) = rules.get(&num) {
                if should_be_next.iter().any(|x| previous.contains(x)) {
                    ok = false;
                    break;
                }
            }
            previous.push(num);
        }
        if ok {
            total += previous[previous.len() / 2];
        }
    }
    total
}

fn part2(rules: &HashMap<u32, Vec<u32>>, operations: &str) -> u32 {
    let mut total = 0;

    for op in operations.lines() {
        let mut previous = vec![];
        let mut ok = true;
        for elem in op.split(',') {
            let num = elem.parse::<u32>().unwrap();
            if let Some(should_be_next) = rules.get(&num) {
                if should_be_next.iter().any(|x| previous.contains(x)) {
                    ok = false;
                    let pos = should_be_next
                        .iter()
                        .filter(|x| previous.contains(x))
                        .map(|x| previous.iter().position(|y| x == y).unwrap())
                        .min()
                        .unwrap();
                    previous.insert(pos, num);
                } else {
                    previous.push(num);
                }
            } else {
                previous.push(num);
            }
        }
        if !ok {
            total += previous[previous.len() / 2];
        }
    }
    total
}

fn main() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let input = include_str!("../input.txt");

    let (rules_str, operations) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::with_capacity(rules_str.lines().count());
    for line in rules_str.lines() {
        let (left, right) = line.split_once("|").unwrap();
        rules
            .entry(left.parse::<u32>().unwrap())
            .or_insert(vec![])
            .push(right.parse::<u32>().unwrap());
    }

    println!("Part 1: {}", part1(&rules, operations));
    println!("Part 2: {}", part2(&rules, operations));
}

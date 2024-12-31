use std::{collections::HashMap, time::Instant};

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (template, design) = input.split_once("\n\n").unwrap();
    (template.split(", ").collect(), design.lines().collect())
}

fn is_valid(template: &Vec<&str>, design: &str) -> bool {
    template.iter().any(|&tem| {
        if design == tem {
            return true;
        } else if design.starts_with(tem) {
            return is_valid(template, &design[tem.len()..]);
        } else {
            return false;
        }
    })
}

fn part1(input: &str) -> usize {
    let (template, design) = parse_input(input);
    design.iter().filter(|d| is_valid(&template, d)).count()
}

fn count_valid<'a>(
    template: &Vec<&str>,
    design: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(c) = cache.get(design) {
        return *c;
    }

    let mut total = 0;
    template.iter().for_each(|&tem| {
        if design == tem {
            total += 1;
        }
        if design.starts_with(tem) {
            total += count_valid(template, &design[tem.len()..], cache);
        }
    });

    cache.insert(design, total);
    total
}

fn part2(input: &str) -> usize {
    let (template, design) = parse_input(input);
    design
        .iter()
        .map(|&tem| {
            let mut cache = HashMap::new();
            count_valid(&template, tem, &mut cache)
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {:?} in {:?}", part2(&input), now.elapsed());
}

#[test]
fn test_large() {
    let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    assert_eq!(part1(&input), 6);

    assert_eq!(part2(&input), 16);
}

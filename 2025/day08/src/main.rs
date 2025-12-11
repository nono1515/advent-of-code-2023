fn part1(input: &str, n: usize) -> u64 {
    let pos = input
        .lines()
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            [
                x.parse::<u64>().unwrap(),
                y.parse::<u64>().unwrap(),
                z.parse::<u64>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();

    let mut id_dist = (0..pos.len())
        .flat_map(|i| {
            (0..pos.len())
                .filter(|j| i < *j)
                .map(|j| (distance(&pos[i], &pos[j], 2), i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    id_dist.sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap());
    let mut grouped_ids: Vec<Vec<&usize>> = Vec::new();
    for (_, id1, id2) in id_dist.iter().take(n) {
        let i1 = grouped_ids.iter().position(|v| v.contains(&id1));
        let i2 = grouped_ids.iter().position(|v| v.contains(&id2));
        if i1.is_none() && i2.is_none() {
            grouped_ids.push(vec![id1, id2]);
        } else if i1.is_some() && i2.is_none() {
            grouped_ids[i1.unwrap()].push(id2);
        } else if i1.is_none() && i2.is_some() {
            grouped_ids[i2.unwrap()].push(id1);
        } else if i1.is_some() && i2.is_some() {
            if i1 == i2 {
                continue;
            }
            let (i1, i2) = (i1.unwrap(), i2.unwrap());
            let (left, right) = grouped_ids.split_at_mut(i1.max(i2));
            left[i1.min(i2)].append(&mut right[0]);
        } else {
            unreachable!();
        }
    }
    let mut lengths = grouped_ids
        .iter()
        .map(|v| v.len() as u64)
        .filter(|l| *l > 0)
        .collect::<Vec<_>>();
    lengths.sort();
    lengths.iter().rev().take(3).product()
}

fn distance(pos1: &[u64], pos2: &[u64], norm: u32) -> f64 {
    let sum: u64 = pos1
        .iter()
        .zip(pos2)
        .map(|(p1, p2)| (p2.max(p1) - p2.min(p1)).pow(norm))
        .sum();
    (sum as f64).powf(1. / norm as f64)
}

fn part2(input: &str) -> u64 {
    let pos = input
        .lines()
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            [
                x.parse::<u64>().unwrap(),
                y.parse::<u64>().unwrap(),
                z.parse::<u64>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();

    let mut id_dist = (0..pos.len())
        .flat_map(|i| {
            (0..pos.len())
                .filter(|j| i < *j)
                .map(|j| (distance(&pos[i], &pos[j], 2), i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    id_dist.sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap());
    let mut grouped_ids: Vec<Vec<&usize>> = Vec::new();
    let mut iter = id_dist.iter();
    loop {
        let (_, id1, id2) = iter.next().unwrap();
        let i1 = grouped_ids.iter().position(|v| v.contains(&id1));
        let i2 = grouped_ids.iter().position(|v| v.contains(&id2));
        if i1.is_none() && i2.is_none() {
            grouped_ids.push(vec![id1, id2]);
        } else if i1.is_some() && i2.is_none() {
            grouped_ids[i1.unwrap()].push(id2);
        } else if i1.is_none() && i2.is_some() {
            grouped_ids[i2.unwrap()].push(id1);
        } else if i1.is_some() && i2.is_some() {
            if i1 == i2 {
                continue;
            }
            let (i1, i2) = (i1.unwrap(), i2.unwrap());
            let (left, right) = grouped_ids.split_at_mut(i1.max(i2));
            left[i1.min(i2)].append(&mut right[0]);
        } else {
            unreachable!();
        }
        let populated = grouped_ids
            .iter()
            .filter(|v| v.len() > 0)
            .collect::<Vec<_>>();
        if populated.len() == 1 && populated[0].len() == pos.len() {
            break pos[*id1][0] * pos[*id2][0];
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part1: {}", part1(&input, 1000));
    println!("part2: {}", part2(&input));
}

#[test]
fn test_both_parts() {
    let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    assert_eq!(part1(&input, 10), 40);
    assert_eq!(part2(&input), 25272);
}

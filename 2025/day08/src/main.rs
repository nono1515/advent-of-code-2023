struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (root_x, root_y) = (self.find(x), self.find(y));
        if root_x == root_y {
            return false;
        }
        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
        true
    }

    fn group_sizes(&mut self) -> Vec<usize> {
        (0..self.parent.len())
            .filter(|&i| self.parent[i] == i)
            .map(|i| self.size[i])
            .collect()
    }
}

fn part1(input: &str, n: usize) -> u64 {
    let pos = get_pos(input);
    let id_dist = compute_distances(&pos);

    let mut uf = UnionFind::new(pos.len());
    for (_, id1, id2) in id_dist.iter().take(n) {
        uf.union(*id1, *id2);
    }

    let mut sizes = uf.group_sizes();
    sizes.sort();
    sizes.iter().rev().take(3).map(|x| *x as u64).product()
}

fn part2(input: &str) -> u64 {
    let pos = get_pos(input);
    let id_dist = compute_distances(&pos);

    let mut uf = UnionFind::new(pos.len());
    for (_, id1, id2) in id_dist.iter() {
        uf.union(*id1, *id2);
        let root = uf.find(0);
        if uf.size[root] == pos.len() {
            return pos[*id1][0] * pos[*id2][0];
        }
    }

    unreachable!(
        "According to the problem definition, we should awlays returning before the loop ends"
    );
}

fn get_pos(input: &str) -> Vec<[u64; 3]> {
    input
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
        .collect::<Vec<_>>()
}

fn compute_distances(pos: &Vec<[u64; 3]>) -> Vec<(f64, usize, usize)> {
    let mut id_dist = (0..pos.len())
        .flat_map(|i| {
            (i + 1..pos.len())
                .map(|j| (distance(&pos[i], &pos[j], 2), i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    id_dist.sort_by(|(d1, _, _), (d2, _, _)| d1.total_cmp(d2));
    id_dist
}

fn distance(pos1: &[u64], pos2: &[u64], norm: u32) -> f64 {
    let sum: u64 = pos1
        .iter()
        .zip(pos2)
        .map(|(p1, p2)| (p2.max(p1) - p2.min(p1)).pow(norm))
        .sum();
    (sum as f64).powf(1. / norm as f64)
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

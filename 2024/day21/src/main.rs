use std::{i32, ops::Add, time::Instant};

/// robot3
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

/// robot2
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

/// robot1
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

/// me
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

const DIR_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'], //
    ['<', 'v', '>'], //
];

const NUM_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_i32(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Add<Direction> for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, rhs: Direction) -> Self::Output {
        (self.0 + rhs.to_i32().0, self.1 + rhs.to_i32().1)
    }
}

fn generate_unique_permutations(dirs: &[Direction]) -> Vec<Vec<Direction>> {
    if dirs.len() <= 1 {
        return vec![dirs.to_vec()];
    }

    let mut result = Vec::new();
    let mut used_at_position = std::collections::HashSet::new();

    for i in 0..dirs.len() {
        let curr = dirs[i].clone();

        // Skip if we've already used this direction at this position
        if used_at_position.contains(&(i, curr.clone())) {
            continue;
        }
        used_at_position.insert((i, curr.clone()));

        let mut remaining = dirs.to_vec();
        remaining.remove(i);

        for mut perm in generate_unique_permutations(&remaining) {
            perm.insert(0, curr.clone());
            result.push(perm);
        }
    }

    // Remove duplicates from final result
    result.sort();
    result.dedup();
    result
}

fn keypad_possibilities(dx: i32, dy: i32) -> Vec<Vec<Direction>> {
    let mut all_dirs = Vec::with_capacity((dx.abs() + dy.abs()) as usize);

    match dx {
        1..=i32::MAX => {
            for _ in 0..dx {
                all_dirs.push(Direction::Right);
            }
        }
        i32::MIN..0 => {
            for _ in 0..-dx {
                all_dirs.push(Direction::Left);
            }
        }
        0 => (),
    }

    match dy {
        1..=i32::MAX => {
            for _ in 0..dy {
                all_dirs.push(Direction::Down);
            }
        }
        i32::MIN..0 => {
            for _ in 0..-dy {
                all_dirs.push(Direction::Up);
            }
        }
        0 => (),
    }

    generate_unique_permutations(&all_dirs)
}

fn keypad_cost(start: (i32, i32), end: (i32, i32), n: i32) -> i32 {
    if n == 0 {
        return (end.0 - start.0).abs() + (end.1 - start.1).abs() + 1;
    }

    let moves_to = keypad_possibilities(end.0 - start.0, end.1 - start.1);
    let move_back = keypad_possibilities(start.0 - end.0, start.1 - end.1);
    // full moves consist of any of move_to append with any of move_back
    let full_moves = moves_to
        .iter()
        .flat_map(|m1| {
            move_back
                .iter()
                .map(move |m2| [m1.clone(), m2.clone()].concat())
        })
        .collect::<Vec<_>>();

    full_moves
        .iter()
        .map(|m| {
            m.iter()
                .fold((0, start), |(cost, pos), d| {
                    (cost + keypad_cost(pos, pos + *d, n - 1), pos + *d)
                })
                .0
        })
        .min()
        .unwrap()
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        // get the integer representation of the 3 first chats
        let n = 3;
        let num = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut robot = (2, 3);
        let mut seq_len = 0;
        for c in line.chars() {
            let y = NUM_KEYPAD.iter().position(|row| row.contains(&c)).unwrap() as i32;
            let x = NUM_KEYPAD[y as usize].iter().position(|&v| v == c).unwrap() as i32;

            println!("ROBOT 3 going from {:?} to {:?}", robot, (x, y));
            seq_len += keypad_cost(robot, (x, y), n);

            println!("Seq len: {}", seq_len);

            robot = (x, y);
        }
        println!("Seq len: {}", seq_len);
        total += seq_len as usize * num;
        break;
    }
    total
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let input = "\
029A
980A
179A
456A
379A";

    // let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(&input), now.elapsed());

    let now = Instant::now();
    println!("Part 2: {:?} in {:?}", part2(&input), now.elapsed());
}

#[test]
fn test_large() {
    let input = "\
029A
980A
179A
456A
379A";

    assert_eq!(part1(&input), 126384);

    assert_eq!(part2(&input), 0);
}

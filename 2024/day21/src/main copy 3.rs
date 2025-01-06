use std::{i32, time::Instant};

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

#[derive(Debug, Clone)]
struct Move {
    directions: Vec<Direction>,
    n_press: usize,
}

impl Move {
    fn abs(&self) -> usize {
        self.directions.len()
    }
}

fn keypad_cost(dx: i32, dy: i32, n: usize) -> usize {
    if n == 0 {
        return (dx.abs() + dy.abs()) as usize;
    }

    match (dx, dy) {
        (0, 0) => 0,
        (0, 1..=i32::MAX) => keypad_cost(-1, 1, n - 1),
        (0, i32::MIN..0) => keypad_cost(1, 1, n - 1),
        (1..=i32::MAX, 0) => keypad_cost(1, -1, n - 1),
        (i32::MIN..0, 0) => keypad_cost(1, -1, n - 1),
        _ => panic!("Invalid input"),
    }
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        // get the integer representation of the 3 first chats
        let n = line
            .chars()
            .take(3)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut robot1 = (0, 2);
        let mut robot2 = (0, 2);
        let mut robot3 = (3, 2);
        let mut seq_len = 0;
        for c in line.chars() {
            let y = NUM_KEYPAD.iter().position(|row| row.contains(&c)).unwrap() as i32;
            let x = NUM_KEYPAD[y as usize].iter().position(|&v| v == c).unwrap() as i32;

            let dy = y - robot3.0;
            let dx = x - robot3.1;

            println!("ROBOT 3 dx: {} dy: {}", dx, dy);

            let robots2_moves = keypad_movement(dx, dy);
            println!("Robot 2 possible moves: {:?}", robots2_moves);

            for robot2_move in robots2_moves {}
        }
        println!("Seq len: {}", seq_len);
        total += seq_len * n;
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

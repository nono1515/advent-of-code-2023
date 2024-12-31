use std::time::Instant;

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

#[derive(Clone, Copy)]
struct Move {
    dx: i32,
    dy: i32,
    n_press: usize,
}

impl Move {
    fn abs(&self) -> usize {
        self.dx.abs() as usize + self.dy.abs() as usize
    }
}

fn keypad_movement(pos: &mut (i32, i32), dx: i32, dy: i32) -> Vec<Option<Move>> {
    let horizontal_move = match dx {
        0 => None,
        1.. => Some(Move {
            dy: 1 - pos.0,
            dx: 2 - pos.1,
            n_press: dx as usize,
        }),
        i32::MIN..0 => Some(Move {
            dy: 1 - pos.0,
            dx: -pos.1,
            n_press: -dx as usize,
        }),
    };

    let vertical_move = match dy {
        0 => None,
        1.. => Some(Move {
            dy: 1 - pos.0,
            dx: 1 - pos.1,
            n_press: dy as usize,
        }),
        i32::MIN..0 => Some(Move {
            dy: -pos.0,
            dx: 1 - pos.1,
            n_press: -dy as usize,
        }),
    };

    let (first_move, second_move) = {
        if horizontal_move.is_some() && vertical_move.is_some() {
            if horizontal_move.unwrap().abs() <= vertical_move.unwrap().abs() {
                (
                    horizontal_move,
                    Some(Move {
                        dx: vertical_move.unwrap().dx - horizontal_move.unwrap().dx,
                        dy: vertical_move.unwrap().dy - horizontal_move.unwrap().dy,
                        n_press: vertical_move.unwrap().n_press,
                    }),
                )
            } else {
                (
                    vertical_move,
                    Some(Move {
                        dx: horizontal_move.unwrap().dx - vertical_move.unwrap().dx,
                        dy: horizontal_move.unwrap().dy - vertical_move.unwrap().dy,
                        n_press: horizontal_move.unwrap().n_press,
                    }),
                )
            }
        } else {
            (horizontal_move, vertical_move)
        }
    };

    if first_move.is_some() {
        pos.0 += first_move.unwrap().dy;
        pos.1 += first_move.unwrap().dx;
    }
    if second_move.is_some() {
        pos.0 += second_move.unwrap().dy;
        pos.1 += second_move.unwrap().dx;
    }

    vec![first_move, second_move]
}

fn press_a(pos: &mut (i32, i32), n_press: usize) -> Move {
    *pos = (0, 2);
    Move {
        dy: 0 - pos.0,
        dx: 2 - pos.1,
        n_press,
    }
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
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

            let mut robot2_moves = keypad_movement(&mut robot2, dx, dy);
            robot2_moves.push(Some(press_a(&mut robot2, 1)));

            for optional_move in &robot2_moves {
                if let Some(move_) = optional_move {
                    println!(
                        "ROBOT 2: dx: {} dy: {} n: {}",
                        move_.dx, move_.dy, move_.n_press
                    );
                    let mut robot1_moves = keypad_movement(&mut robot1, move_.dx, move_.dy);
                    robot1_moves.push(Some(press_a(&mut robot1, move_.n_press)));
                    for optional_move in &robot1_moves {
                        if let Some(move_) = optional_move {
                            println!(
                                "ROBOT 1: dx: {} dy: {} n: {}",
                                move_.dx, move_.dy, move_.n_press
                            );
                            seq_len += move_.abs() + move_.n_press;
                        }
                    }
                }
            }

            robot3 = (robot3.0 + dy, robot3.1 + dx);
        }
        println!("Seq len: {}", seq_len);
        break;
    }
    0
}

fn part2(input: &str) -> usize {
    0
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
029A
980A
179A
456A
379A";

    assert_eq!(part1(&input), 126384);

    assert_eq!(part2(&input), 0);
}

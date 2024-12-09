use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
struct UsedSpace {
    id: usize,
    start_i: usize,
    size: usize,
}

#[derive(Debug)]
struct EmptySpace {
    start_i: usize,
    size: usize,
}

fn parse_input(input: &str) -> (Vec<EmptySpace>, Vec<UsedSpace>) {
    let mut empty_spaces = vec![];
    let mut used_spaces = vec![];

    let mut used = true;
    let mut start_i = 0;
    let mut id = 0;
    for c in input.chars() {
        if c == ' ' || c == '\n' {
            continue;
        }
        let size = c.to_digit(10).unwrap() as usize;
        if used {
            used_spaces.push(UsedSpace { id, start_i, size });
            id += 1;
        } else {
            empty_spaces.push(EmptySpace { start_i, size });
        }
        used = !used;
        start_i += size;
    }
    (empty_spaces, used_spaces)
}

fn part1(input: &str) -> usize {
    let (mut empty_spaces, mut used_spaces) = parse_input(input);

    empty_spaces.sort_by_key(|x| -(x.start_i as i32));
    while empty_spaces.last().unwrap().start_i < used_spaces.last().unwrap().start_i {
        let empty = empty_spaces.pop().unwrap();
        let used = used_spaces.pop().unwrap();
        match empty.size.cmp(&used.size) {
            std::cmp::Ordering::Less => {
                used_spaces.push(UsedSpace {
                    id: used.id,
                    start_i: empty.start_i,
                    size: empty.size,
                });
                used_spaces.push(UsedSpace {
                    id: used.id,
                    start_i: used.start_i,
                    size: used.size - empty.size,
                });
                empty_spaces.push(EmptySpace {
                    start_i: used.start_i + used.size,
                    size: empty.size,
                });
            }
            std::cmp::Ordering::Equal => {
                empty_spaces.push(EmptySpace {
                    start_i: used.start_i,
                    size: empty.size,
                });
                used_spaces.push(UsedSpace {
                    id: used.id,
                    start_i: empty.start_i,
                    size: used.size,
                });
            }
            std::cmp::Ordering::Greater => {
                used_spaces.push(UsedSpace {
                    id: used.id,
                    start_i: empty.start_i,
                    size: used.size,
                });
                empty_spaces.push(EmptySpace {
                    start_i: empty.start_i + used.size,
                    size: empty.size - used.size,
                });
                empty_spaces.push(EmptySpace {
                    start_i: used.start_i,
                    size: used.size,
                });
            }
        };
        used_spaces.sort_by_key(|x| x.start_i);
        empty_spaces.sort_by_key(|x| -(x.start_i as i32));
    }

    used_spaces
        .iter()
        .map(|x| x.id * (x.start_i..(x.start_i + x.size)).sum::<usize>())
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let (mut empty_spaces, used_spaces) = parse_input(input);

    let mut sorted_used_spaces = vec![];

    for space in used_spaces.iter().rev() {
        let mut to_add = vec![];
        let mut to_remove = None;
        let empty_with_index = empty_spaces
            .iter()
            .enumerate()
            .filter(|(_, empty)| empty.size >= space.size)
            .min_by_key(|(_, empty)| empty.start_i);

        if let Some((i, empty)) = empty_with_index {
            if empty.start_i < space.start_i {
                to_remove = Some(i);
                sorted_used_spaces.push(UsedSpace {
                    id: space.id,
                    start_i: empty.start_i,
                    size: space.size,
                });
                to_add.push(EmptySpace {
                    start_i: space.start_i,
                    size: space.start_i,
                });
                if empty.size > space.size {
                    to_add.push(EmptySpace {
                        start_i: empty.start_i + space.size,
                        size: empty.size - space.size,
                    });
                }
            } else {
                sorted_used_spaces.push(space.clone());
            }
        } else {
            sorted_used_spaces.push(space.clone());
        }

        if let Some(i) = to_remove {
            empty_spaces.remove(i);
            empty_spaces.extend(to_add);
        }
    }

    // let output = (0..100)
    //     .map(|i| {
    //         sorted_used_spaces
    //             .iter()
    //             .find(|x| (x.start_i..(x.start_i + x.size)).contains(&i))
    //             .map(|x| x.id.to_string())
    //             .unwrap_or(".".to_string())
    //     })
    //     .collect::<String>();
    // println!("{}", output);

    sorted_used_spaces
        .iter()
        .map(|x| x.id * (x.start_i..(x.start_i + x.size)).sum::<usize>())
        .sum::<usize>()
}

fn main() {
    let input = "2333133121414131402";

    let input = include_str!("../input.txt");

    let now = Instant::now();
    println!("Part 1: {} in {:?}", part1(input), now.elapsed());
    let now = Instant::now();
    println!("Part 2: {} in {:?}", part2(input), now.elapsed());
}

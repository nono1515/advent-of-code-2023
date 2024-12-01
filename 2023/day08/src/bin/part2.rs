use day08::{bisect_search, Node};

fn main() {
    let input = include_str!("../../input.txt");

    let mut nodes = input.lines();
    let instruction = nodes.next().unwrap().chars().collect::<Vec<_>>();

    let mut map = vec![];
    for node in nodes.skip(1) {
        let (curr, left, right) = (&node[..3], &node[7..10], &node[12..15]);
        let node = Node {
            node: curr.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        };
        map.push(node);
    }
    map.sort_by(|a, b| a.node.cmp(&b.node));

    let starting_nodes: Vec<_> = map.iter().filter(|n| n.node.ends_with('A')).collect();
    let periods: Vec<_> = starting_nodes
        .iter()
        .map(|n| {
            let mut steps = 0;
            let mut current_node = *n;
            'search_loop: loop {
                for i in instruction.iter() {
                    let next_node = match i {
                        'L' => &current_node.left,
                        'R' => &current_node.right,
                        _ => panic!("wrong instruction {}", i),
                    };
                    current_node = bisect_search(&map, next_node);
                    steps += 1;
                    if current_node.node.ends_with('Z') {
                        break 'search_loop;
                    }
                }
            }
            steps
        })
        .collect();

    println!("Least common multiple of: {:?}", periods);
    println!("{}", periods.iter().fold(1, |acc, x| lcm(acc, *x)));
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

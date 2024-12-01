use std::{
    sync::atomic::{AtomicUsize, Ordering},
    thread::panicking,
};

fn main() {
    let input: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    let input = include_str!("../../input.txt");

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '.' | '<' | '>' | '^' | 'v' => TileType::Path,
                    '#' => TileType::Wall,
                    c => panic!("Unknown char: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let starting_pos = (0, map[0].iter().position(|t| *t == TileType::Path).unwrap());
    let ending_pos = (
        map.len() - 1,
        map[map.len() - 1]
            .iter()
            .position(|t| *t == TileType::Path)
            .unwrap(),
    );

    let (nodes, edges) = build_graph(&map, starting_pos);

    // println!("{:?}", nodes);
    // println!("{:?}", edges);

    let mut stack = vec![(get_node_from_pos(&starting_pos, &nodes), 0, vec![])];
    let mut max_len = 0;

    while let Some((node, steps, mut visited)) = stack.pop() {
        if let Some(node) = node {
            if node.pos == ending_pos {
                max_len = max_len.max(steps);
                continue;
            }
            visited.push(node.id);

            // println!("{:?} {:?}", visited, stack);
            // println!(".--------------------------");

            edges
                .iter()
                .filter(|edge| edge.from == node.id)
                .filter(|edge| !visited.contains(&edge.to))
                .for_each(|edge| {
                    stack.push((
                        get_node(&edge.to, &nodes),
                        steps + edge.steps,
                        visited.clone(),
                    ));
                });
            edges
                .iter()
                .filter(|edge| edge.to == node.id)
                .filter(|edge| !visited.contains(&edge.from))
                .for_each(|edge| {
                    stack.push((
                        get_node(&edge.from, &nodes),
                        steps + edge.steps,
                        visited.clone(),
                    ));
                });
        } else {
            panic!("this should not happen");
        }
    }

    println!("{}", max_len);
}

fn get_node_from_pos<'a>(pos: &(usize, usize), nodes: &'a Vec<Node>) -> Option<&'a Node> {
    nodes.iter().find(|node| node.pos == *pos)
}

fn get_node<'a>(id: &usize, nodes: &'a Vec<Node>) -> Option<&'a Node> {
    nodes.iter().find(|node| node.id == *id)
}

// fn get_edges<'a>(from: usize, edges: &'a Vec<Edge>) -> &'a Vec<Edge> {
//     edges.iter().filter(|edge| edge.from == from)
// }

fn build_graph(map: &Vec<Vec<TileType>>, starting_pos: (usize, usize)) -> (Vec<Node>, Vec<Edge>) {
    let mut to_visit = map
        .iter()
        .map(|row| row.iter().map(|t| t == &TileType::Path).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let get_next_id = || NEXT_ID.fetch_add(1, Ordering::Relaxed);

    let id = get_next_id();
    let mut nodes = vec![Node {
        id,
        pos: starting_pos,
    }];
    let mut edges = vec![];
    let mut stack = vec![(starting_pos, id, 0)];

    while let Some((pos, mut node_from, prev_steps)) = stack.pop() {
        // this node is now visited
        to_visit[pos.0][pos.1] = false;

        let neighbors = [(0, -1), (0, 1), (1, 0), (-1, 0)]
            .iter()
            .map(|(dx, dy)| ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize))
            .filter(|new_pos| {
                map.get(new_pos.0).and_then(|row| row.get(new_pos.1)) == Some(&TileType::Path)
            })
            .collect::<Vec<_>>();

        let steps = match neighbors.len() {
            1 => {
                if let Some(new_pos) = neighbors.get(0) {
                    // if neighbor is already visited, we are in a deadend and add a node and a edge
                    if !to_visit[new_pos.0][new_pos.1] {
                        let id = get_next_id();
                        nodes.push(Node { id, pos });
                        edges.push(Edge {
                            from: node_from,
                            to: id,
                            steps: prev_steps,
                        });
                        node_from = id;
                    }
                }
                prev_steps + 1
            }
            2 => prev_steps + 1,
            3..=4 => {
                let id = get_next_id();
                nodes.push(Node { id, pos });
                edges.push(Edge {
                    from: node_from,
                    to: id,
                    steps: prev_steps,
                });
                node_from = id;
                1
            }
            0 => panic!("No neighbors, how did you get there?"),
            _ => panic!("More than 4 neighbors in a grid with 4-way connectivity (+)"),
        };

        neighbors.iter().for_each(|neighbor| {
            if to_visit[neighbor.0][neighbor.1] {
                stack.push((*neighbor, node_from, steps));
            }
            if let Some(node) = nodes.iter().find(|node| node.pos == *neighbor) {
                if node.id != node_from {
                    edges.push(Edge {
                        from: node_from,
                        to: node.id,
                        steps,
                    });
                }
            }
        });
    }

    (nodes, edges)
}

fn process_neighbors(
    neighbor: &(usize, usize),
    to_visit: &Vec<Vec<bool>>,
    nodes: &mut Vec<Node>,
    edges: &mut Vec<Edge>,
    stack: &mut Vec<((usize, usize), usize, usize)>,
    id: usize,
    steps: usize,
) {
    if to_visit[neighbor.0][neighbor.1] {
        stack.push((*neighbor, id, steps));
    }
    if let Some(node) = nodes.iter().find(|node| node.pos == *neighbor) {
        edges.push(Edge {
            from: id,
            to: node.id,
            steps,
        });
    }
}

#[derive(PartialEq, Debug)]
enum TileType {
    Path,
    Wall,
}

#[derive(Debug)]
struct Node {
    id: usize,
    pos: (usize, usize),
}

#[derive(Debug)]
struct Edge {
    from: usize,
    to: usize,
    steps: usize,
}

use std::fmt;

fn main() {
    let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    // let input = include_str!("../../input.txt");
    let map: Map = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    // println!("{}", map);

    let start = (0, 0);
    let end = (map.width - 1, map.height - 1);

    // let path = djikstra(start, end, &map);
    // println!("{:?}", path);
}

struct Point {
    x: usize,
    y: usize,
}

struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, p: Point) -> usize {
        self.map[p.y][p.x]
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.map {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl FromIterator<Vec<usize>> for Map {
    fn from_iter<T: IntoIterator<Item = Vec<usize>>>(iter: T) -> Self {
        let map: Vec<Vec<usize>> = iter.into_iter().collect();
        let width = map[0].len();
        let height = map.len();
        Map { map, width, height }
    }
}

fn djikstra(start: Point, end: Point, costs: Map) -> Map {
    let dist = Map {
        map: vec![vec![usize::MAX; costs.width]; costs.height],
        width: costs.width,
        height: costs.height,
    };
    

    dist
}

// fn a_star(start: ) {
//     let mut nodes = vec![vec![u32::MAX; map[0].len()]; map.len()];
//     let mut visited = vec![vec![false; map[0].len()]; map.len()];
//     nodes[0][0] = 0;
//     visited[0][0] = true;

//     let current = (0, 0);
// }

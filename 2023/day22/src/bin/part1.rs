fn falls(pieces: &mut Vec<Piece>, skip: Option<usize>) -> usize {
    let mut floor: Vec<Vec<usize>> =
        vec![
            vec![0; pieces.iter().map(|p| p.max_y()).max().unwrap() + 1];
            pieces.iter().map(|p| p.max_x()).max().unwrap() + 1
        ];

    let mut n_falls = 0;

    for (i, p) in pieces.iter_mut().enumerate() {
        if let Some(skip) = skip {
            if i == skip {
                continue;
            }
        }
        let level = p.bricks().iter().map(|b| floor[b[0]][b[1]]).max().unwrap();
        n_falls += (p.min_z() > level + 1) as usize;
        let z_diff = p.max_z() - p.min_z() + 1;
        p.start[2] = level + 1;
        p.end[2] = level + z_diff;
        p.bricks()
            .iter()
            .for_each(|b| floor[b[0]][b[1]] = level + z_diff);
    }

    n_falls
}

fn main() {
    //     let input: &str = "\
    // 1,0,1~1,2,1
    // 0,0,2~2,0,2
    // 0,2,3~2,2,3
    // 0,0,4~0,2,4
    // 2,0,5~2,2,5
    // 0,1,6~2,1,6
    // 1,1,8~1,1,9";

    let input = include_str!("../../input.txt");

    let mut pieces = input
        .lines()
        .map(|line| Piece::new_from_line(line))
        .collect::<Vec<_>>();
    pieces.sort_by_key(|p| p.min_z());

    falls(&mut pieces, None);

    // part 1
    let not_falling = (0..pieces.len())
        .map(|i| falls(&mut pieces.clone(), Some(i)))
        .filter(|&x| x == 0)
        .count();
    println!("{}", not_falling);

    // part 2
    let n_falls = (0..pieces.len())
        .map(|i| falls(&mut pieces.clone(), Some(i)))
        .sum::<usize>();
    println!("{:?}", n_falls);
}

#[derive(Clone, Debug)]
enum Orientation {
    X,
    Y,
    Z,
    SingleBlock,
}

#[derive(Clone, Debug)]
struct Piece {
    start: [usize; 3],
    end: [usize; 3],
    orientation: Orientation,
}

macro_rules! gen_minmax {
    ($(($name:ident, $idx:expr, $op:ident)),*) => {
        $(
            fn $name(&self) -> usize {
                self.start[$idx].$op(self.end[$idx])
            }
        )*
    }
}

impl Piece {
    fn new_from_line(s: &str) -> Self {
        let (start, end) = s.split_once('~').unwrap();
        let mut start = start.split(',').map(|x| x.parse().unwrap());
        let start = [
            start.next().unwrap(),
            start.next().unwrap(),
            start.next().unwrap(),
        ];
        let mut end = end.split(',').map(|x| x.parse().unwrap());
        let end = [
            end.next().unwrap(),
            end.next().unwrap(),
            end.next().unwrap(),
        ];
        let orientation = match (start[0] != end[0], start[1] != end[1], start[2] != end[2]) {
            (true, false, false) => Orientation::X,
            (false, true, false) => Orientation::Y,
            (false, false, true) => Orientation::Z,
            (false, false, false) => Orientation::SingleBlock,
            _ => panic!("Invalid input: start={:?}, end={:?}", start, end),
        };
        Self {
            start,
            end,
            orientation,
        }
    }

    fn bricks(&self) -> Vec<[usize; 3]> {
        match self.orientation {
            Orientation::X => (self.min_x()..=self.max_x())
                .map(|x| [x, self.start[1], self.start[2]])
                .collect(),
            Orientation::Y => (self.min_y()..=self.max_y())
                .map(|y| [self.start[0], y, self.start[2]])
                .collect(),
            Orientation::Z => (self.min_z()..=self.max_z())
                .map(|z| [self.start[0], self.start[1], z])
                .collect(),
            Orientation::SingleBlock => vec![self.start],
        }
    }

    gen_minmax!(
        (min_x, 0, min),
        (max_x, 0, max),
        (min_y, 1, min),
        (max_y, 1, max),
        (min_z, 2, min),
        (max_z, 2, max)
    );
}

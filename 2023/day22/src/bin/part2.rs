fn main() {
    let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    let input = include_str!("../../input.txt");

    let mut pieces = input
        .lines()
        .enumerate()
        .map(|(id, line)| Piece::new_from_line(id, line))
        .collect::<Vec<_>>();
    pieces.sort_by(|a, b| b.z0().cmp(&a.z0()));

    let mut stabilized_pieces = vec![];
    let max_x = pieces.iter().map(|p| p.start.0.max(p.end.0)).max().unwrap();
    let max_y = pieces.iter().map(|p| p.start.1.max(p.end.1)).max().unwrap();
    let mut z_level = vec![vec![0; max_x + 1]; max_y + 1];

    while let Some(mut piece) = pieces.pop() {
        let mut z_dist = usize::MAX;
        for (x, y, z) in piece.cubes() {
            z_dist = z_dist.min(z - z_level[y][x] - 1);
        }

        piece.slide_down(z_dist);
        for (x, y, z) in piece.cubes() {
            z_level[y][x] = z_level[y][x].max(z);
        }
        stabilized_pieces.push(piece);
    }

    let mut count = 0;
    let mut pairs = vec![];
    for z in (0..=stabilized_pieces.iter().map(|p| p.z1()).max().unwrap()).rev() {
        let curr_pieces: Vec<_> = stabilized_pieces.iter().filter(|p| p.z1() == z).collect();
        let above_piece: Vec<_> = stabilized_pieces
            .iter()
            .filter(|p| p.z0() == z + 1)
            .collect();

        for p0 in curr_pieces.iter() {
            for p1 in above_piece.iter() {
                if p0.cases().any(|c1| p1.cases().any(|c2| c1 == c2)) {
                    pairs.push((p0.id, p1.id));
                }
            }
        }
        // println!("Curr: {:?}, Above: {:?}", curr_pieces, above_piece);
        // println!("{:?}", pairs);
        'curr_pieces: for i in 0..curr_pieces.len() {
            for j in 0..above_piece.len() {
                if pairs.contains(&(i, j)) {
                    if !pairs.iter().any(|(ii, jj)| *ii != i && *jj == j) {
                        continue 'curr_pieces;
                    }
                }
            }
            // println!("{} is okay", i);
            count += 1;
        }
    }

    println!("{}", count);
}

#[derive(Debug)]
struct Piece {
    id: usize,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Piece {
    fn new_from_line(id: usize, s: &str) -> Self {
        let (start, end) = s.split_once('~').unwrap();
        let mut start = start.split(',').map(|x| x.parse().unwrap());
        let start = (
            start.next().unwrap(),
            start.next().unwrap(),
            start.next().unwrap(),
        );
        let mut end = end.split(',').map(|x| x.parse().unwrap());
        let end = (
            end.next().unwrap(),
            end.next().unwrap(),
            end.next().unwrap(),
        );
        Self { id, start, end }
    }

    fn z0(&self) -> usize {
        self.start.2.min(self.end.2)
    }

    fn z1(&self) -> usize {
        self.start.2.max(self.end.2)
    }

    fn cubes(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (self.start.0..=self.end.0).flat_map(move |x| {
            (self.start.1..=self.end.1)
                .flat_map(move |y| (self.start.2..=self.end.2).map(move |z| (x, y, z)))
        })
    }

    fn cases(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.start.0..=self.end.0)
            .flat_map(move |x| (self.start.1..=self.end.1).map(move |y| (x, y)))
    }

    fn slide_down(&mut self, n: usize) {
        self.start.2 -= n;
        self.end.2 -= n;
    }

    // fn collide(&self, other: &Piece) -> bool {
    //     true
    // }
}

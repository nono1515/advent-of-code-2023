fn main() {
    let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    // let input = include_str!("../../input.txt");

    let pieces = input.lines().map(|line| Piece::new_from_line(line)).collect::<Vec<_>>();

    println!("{:?}", pieces);
}

#[derive(Debug)]
struct Piece {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Piece {
    fn new_from_line(s: &str) -> Self {
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
        Self { start, end }
    }
}

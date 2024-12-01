#[derive(Clone, Copy, Debug)]
pub enum Pipe {
    Pipe {
        north: bool,
        south: bool,
        east: bool,
        west: bool,
    },
    NoPipe,
    StartingPoint,
}

impl Pipe {
    pub fn parse(c: char) -> Self {
        match c {
            '|' => Pipe::Pipe {
                north: true,
                south: true,
                east: false,
                west: false,
            },
            '-' => Pipe::Pipe {
                north: false,
                south: false,
                east: true,
                west: true,
            },
            'L' => Pipe::Pipe {
                north: true,
                south: false,
                east: true,
                west: false,
            },
            'J' => Pipe::Pipe {
                north: true,
                south: false,
                east: false,
                west: true,
            },
            '7' => Pipe::Pipe {
                north: false,
                south: true,
                east: false,
                west: true,
            },
            'F' => Pipe::Pipe {
                north: false,
                south: true,
                east: true,
                west: false,
            },
            '.' => Pipe::NoPipe,
            'S' => Pipe::StartingPoint,
            _ => panic!("Unknown pipe type: {}", c),
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub row: usize,
    pub col: usize,
    pub steps: u32,
    pub pipe: Pipe,
}

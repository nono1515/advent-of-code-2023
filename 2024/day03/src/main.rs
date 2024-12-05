#[derive(Debug, PartialEq)]
enum MulState {
    None,
    M,
    U,
    L,
    MulFunc,
    NUM1,
    COMMA,
    NUM2,
    // CLOSE,
    D,
    O,
    DoFunc,
    N,
    APOSTROPHE,
    T,
    DontFunc,
}

fn part1(lines: &str) -> u32 {
    let mut res = 0;
    let mut enabled = true;
    for line in lines.lines() {
        let mut state = MulState::None;
        let mut num1 = 0;
        let mut num2 = 0;
        for c in line.chars() {
            match c {
                'm' => {
                    state = MulState::M;
                }
                'u' if { state == MulState::M } => {
                    state = MulState::U;
                }
                'l' if { state == MulState::U } => {
                    state = MulState::L;
                }
                '(' if { state == MulState::L } => {
                    state = MulState::MulFunc;
                }
                '(' if { state == MulState::O } => {
                    state = MulState::DoFunc;
                }
                '(' if { state == MulState::T } => {
                    state = MulState::DontFunc;
                }
                '0'..='9' if { state == MulState::MulFunc } => {
                    num1 += c.to_digit(10).unwrap();
                    state = MulState::NUM1;
                }
                '0'..='9' if { state == MulState::NUM1 } => {
                    num1 *= 10;
                    num1 += c.to_digit(10).unwrap();
                }
                ',' if { state == MulState::NUM1 } => {
                    state = MulState::COMMA;
                }
                '0'..='9' if { state == MulState::COMMA } => {
                    num2 += c.to_digit(10).unwrap();
                    state = MulState::NUM2;
                }
                '0'..='9' if { state == MulState::NUM2 } => {
                    num2 *= 10;
                    num2 += c.to_digit(10).unwrap();
                }
                ')' => {
                    match state {
                        MulState::NUM2 => {
                            if num1 < 1000 && num2 < 1000 && enabled {
                                res += num1 * num2;
                            }
                        }
                        MulState::DoFunc => {
                            enabled = true;
                        }
                        MulState::DontFunc => {
                            enabled = false;
                        }
                        _ => {}
                    }
                    state = MulState::None;
                    num1 = 0;
                    num2 = 0;
                }
                'd' => {
                    state = MulState::D;
                }
                'o' if { state == MulState::D } => {
                    state = MulState::O;
                }
                'n' if { state == MulState::O } => {
                    state = MulState::N;
                }
                '\'' if { state == MulState::N } => {
                    state = MulState::APOSTROPHE;
                }
                't' if { state == MulState::APOSTROPHE } => {
                    state = MulState::T;
                }
                _ => {
                    state = MulState::None;
                    num1 = 0;
                    num2 = 0;
                }
            }
        }
    }
    res
}

fn main() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(&input));
}

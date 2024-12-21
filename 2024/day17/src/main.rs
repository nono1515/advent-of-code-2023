use std::{
    str::{Chars, FromStr},
    time::Instant,
};

#[derive(Debug, PartialEq)]
enum CommandResult {
    Nothing,
    Jump(usize),
    Out(u8),
}

#[derive(Debug, PartialEq)]
struct ComboError {
    msg: &'static str,
}

fn combo_operand(operand: u8, registers: &[u32; 3]) -> Result<u32, ComboError> {
    match operand {
        0..=3 => Ok(operand as u32),
        4 => Ok(registers[0]),
        5 => Ok(registers[1]),
        6 => Ok(registers[2]),
        _ => Err(ComboError {
            msg: "Invalid operand",
        }),
    }
}

fn apply_command(
    command: u8,
    arg: u8,
    registers: &mut [u32; 3],
) -> Result<CommandResult, ComboError> {
    match command {
        0 => {
            let combo = combo_operand(arg, &registers)?;
            registers[0] = registers[0] >> combo;
            Ok(CommandResult::Nothing)
        }
        1 => {
            registers[1] = registers[1] ^ arg as u32;
            Ok(CommandResult::Nothing)
        }
        2 => {
            registers[1] = combo_operand(arg, &registers)? % 8;
            Ok(CommandResult::Nothing)
        }
        3 => {
            if registers[0] == 0 {
                Ok(CommandResult::Nothing)
            } else {
                Ok(CommandResult::Jump(arg as usize))
            }
        }
        4 => {
            registers[1] = registers[1] ^ registers[2];
            Ok(CommandResult::Nothing)
        }
        5 => Ok(CommandResult::Out(
            (combo_operand(arg, &registers)? % 8) as u8,
        )),
        6 => {
            let combo = combo_operand(arg, &registers)?;
            registers[1] = registers[0] >> combo;
            Ok(CommandResult::Nothing)
        }
        7 => {
            let combo = combo_operand(arg, &registers)?;
            registers[2] = registers[0] >> combo;
            Ok(CommandResult::Nothing)
        }
        _ => Err(ComboError {
            msg: "Invalid command",
        }),
    }
}

fn parse_num<T: FromStr>(chars: &mut Chars) -> Result<T, <T as FromStr>::Err> {
    chars
        .skip_while(|c| !c.is_numeric())
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<T>()
}

fn parse_input(input: &str) -> (Vec<u8>, [u32; 3]) {
    let mut it = input.chars();

    let registers = [
        parse_num(&mut it).unwrap(),
        parse_num(&mut it).unwrap(),
        parse_num(&mut it).unwrap(),
    ];

    let mut commands = Vec::new();
    while let Ok(command) = parse_num::<u8>(&mut it) {
        commands.push(command);
    }

    (commands, registers)
}

fn part1(commands: &Vec<u8>, registers: &[u32; 3]) -> Vec<u8> {
    let mut pointer = 0;
    let mut registers = registers.clone();
    let mut out = vec![];

    while pointer < commands.len() - 1 {
        let result =
            apply_command(commands[pointer], commands[pointer + 1], &mut registers).unwrap();
        match result {
            CommandResult::Nothing => {}
            CommandResult::Jump(arg) => {
                pointer = arg;
                continue;
            }
            CommandResult::Out(arg) => out.push(arg),
        }
        pointer += 2;
    }

    out
}

fn part2(input: &str) -> usize {
    let mut i = 0;
    let (commands, registers) = parse_input(&input);

    loop {
        let res = part1(&commands, &registers);
        if res == commands {
            return i;
        }

        i += 1;
    }
}

fn main() {
    //     let input = "\
    // Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";

    let input = include_str!("../input.txt");

    let (commands, registers) = parse_input(&input);

    let now = Instant::now();
    let output = part1(&commands, &registers);
    let output = output
        .iter()
        .map(|u| u.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Part 1: {} in {:?}", output, now.elapsed());

    // let now = Instant::now();
    // println!("Part 2: {} in {:?}", part2(&input), now.elapsed());
}

#[test]
fn test_large() {
    let input = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let (commands, registers) = parse_input(&input);

    assert_eq!(
        part1(&commands, &registers),
        vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
    );

    let input = "\
    Register A: 2024
    Register B: 0
    Register C: 0

    Program: 0,3,5,4,3,0";

    assert_eq!(part2(&input), 117440);
}

fn test_division(cmd_num: u8, reg_num: usize) {
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(cmd_num, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[reg_num], 15);

    let _ = apply_command(cmd_num, 2, &mut registers);
    assert_eq!(registers[reg_num], 3);

    let _ = apply_command(cmd_num, 5, &mut registers);
    assert_eq!(registers[reg_num], 0);
}

#[test]
fn test_ops() {
    // Command 0
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(0, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[0], 15);

    let _ = apply_command(0, 2, &mut registers);
    assert_eq!(registers[0], 3);

    let _ = apply_command(0, 5, &mut registers);
    assert_eq!(registers[0], 0);

    // Command 6
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(6, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[1], 15);

    let _ = apply_command(6, 2, &mut registers);
    assert_eq!(registers[1], 7);

    let _ = apply_command(6, 5, &mut registers);
    assert_eq!(registers[1], 0);

    // Command 7
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(7, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[2], 15);

    let _ = apply_command(7, 2, &mut registers);
    assert_eq!(registers[2], 7);

    let _ = apply_command(7, 5, &mut registers);
    assert_eq!(registers[2], 0);

    // Commands 1
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(1, 8, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[1], 20 ^ 8);
    let _ = apply_command(1, 5, &mut registers);
    assert_eq!(registers[1], 20 ^ 8 ^ 5);
    let _ = apply_command(1, 0, &mut registers);
    assert_eq!(registers[1], 20 ^ 8 ^ 5 ^ 0);

    // Commands 2
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(2, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[1], 1);
    let _ = apply_command(2, 6, &mut registers);
    assert_eq!(registers[1], 2);
    let _ = apply_command(2, 4, &mut registers);
    assert_eq!(registers[1], 6);
    let _ = apply_command(2, 5, &mut registers);
    assert_eq!(registers[1], 6);

    // Commands 3
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(3, 6, &mut registers),
        Ok(CommandResult::Jump(6))
    );
    assert_eq!(
        apply_command(3, 0, &mut registers),
        Ok(CommandResult::Jump(0))
    );
    registers[0] = 0;
    assert_eq!(
        apply_command(3, 6, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(
        apply_command(3, 0, &mut registers),
        Ok(CommandResult::Nothing)
    );

    // Commands 4
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(4, 1, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[1], 20 ^ 10);
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(4, 7, &mut registers),
        Ok(CommandResult::Nothing)
    );
    assert_eq!(registers[1], 20 ^ 10);

    // Commands 5
    let mut registers: [u32; 3] = [30, 20, 10];
    assert_eq!(
        apply_command(5, 0, &mut registers),
        Ok(CommandResult::Out(0))
    );
    assert_eq!(
        apply_command(5, 4, &mut registers),
        Ok(CommandResult::Out(6))
    );
    assert_eq!(
        apply_command(5, 5, &mut registers),
        Ok(CommandResult::Out(4))
    );
    assert_eq!(
        apply_command(5, 6, &mut registers),
        Ok(CommandResult::Out(2))
    );
}

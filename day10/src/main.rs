use std::{collections::HashSet, fs};

#[derive(Debug, PartialEq)]
enum InstructionName {
    Addx,
    Noop,
}

struct Instruction {
    name: InstructionName,
    args: Vec<i32>,
}

impl Instruction {
    fn new_addx(str_value: &str) -> Instruction {
        let value = str_value.parse::<i32>().unwrap();
        return Instruction {
            name: InstructionName::Addx,
            args: vec![value],
        };
    }

    fn new_noop() -> Instruction {
        return Instruction {
            name: InstructionName::Noop,
            args: vec![],
        };
    }
}

fn get_instructions(input_str: &str) -> Vec<Instruction> {
    let instruction_lines: Vec<&str> = input_str.split("\n").collect();
    let mut instructions: Vec<Instruction> = vec![];
    for instruction_line in instruction_lines {
        let instruction_parts: Vec<&str> = instruction_line.split_whitespace().collect();
        if instruction_parts[0] == "noop" {
            instructions.push(Instruction::new_noop())
        } else if instruction_parts[0] == "addx" {
            instructions.push(Instruction::new_addx(instruction_parts[1]));
        }
    }
    instructions
}

fn check_cycle_and_compute(cycle_count: i32, register_x: i32) -> i32 {
    if cycle_count == 20 || (cycle_count - 20) % 40 == 0 {
        let signal_strength = cycle_count * register_x;
        println!(
            "Current Cycle: {}, signal_strength: {}",
            cycle_count, signal_strength
        );
        return signal_strength;
    }
    0
}

fn part_one(instructions: &Vec<Instruction>) {
    let mut cycle_count = 0;
    let mut register_x = 1;
    let mut signal_strength_sum = 0;
    for instruction in instructions {
        cycle_count += 1;
        signal_strength_sum += check_cycle_and_compute(cycle_count, register_x);
        if instruction.name == InstructionName::Addx {
            cycle_count += 1;
            signal_strength_sum += check_cycle_and_compute(cycle_count, register_x);
            register_x += instruction.args[0];
        }
    }
    println!("Part One Ans: {}", signal_strength_sum);
}

fn handle_cycle(cycle_count: i32, register_x: i32, crt: &mut Vec<Vec<char>>) {
    // get our sprit positions which are register x -1, register x, and register x + 1
    // get the index that the pixel is being drawn at by taking cycle count % 40
    // if pixel index value overlaps with register values then print # otherwise .
    if (cycle_count % 40) - 1 == 0 {
        crt.push(vec![]);
    }
    let crt_idx = crt.len() - 1;
    let sprite_set: HashSet<i32> = HashSet::from_iter(register_x - 1..=register_x + 1);
    let pixel_position = (cycle_count - 1) % 40;
    if sprite_set.contains(&pixel_position) {
        crt[crt_idx].push('#');
    } else {
        crt[crt_idx].push(' ');
    }
}

fn part_two(instructions: &Vec<Instruction>) {
    let mut cycle_count = 0;
    let mut register_x = 1;
    let mut crt: Vec<Vec<char>> = vec![];
    for instruction in instructions {
        cycle_count += 1;
        handle_cycle(cycle_count, register_x, &mut crt);
        if instruction.name == InstructionName::Addx {
            cycle_count += 1;
            handle_cycle(cycle_count, register_x, &mut crt);
            register_x += instruction.args[0];
        }
    }
    for crt_line in crt {
        let crt_str: String = crt_line.iter().collect();
        println!("{}", crt_str);
    }
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt").unwrap();
    let instructions = get_instructions(&input_str);
    part_one(&instructions);
    part_two(&instructions);
}

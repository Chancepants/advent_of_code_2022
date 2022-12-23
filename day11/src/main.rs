use std::f64;
use std::{collections::VecDeque, fs};

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

impl Operator {
    fn new(operator_char: char) -> Operator {
        if operator_char == '+' {
            return Operator::Add;
        } else if operator_char == '-' {
            return Operator::Subtract;
        } else if operator_char == '/' {
            return Operator::Divide;
        } else if operator_char == '*' {
            return Operator::Multiply;
        } else if operator_char == '%' {
            return Operator::Modulo;
        } else {
            panic!("Unsupported operator")
        }
    }
}
#[derive(Debug, Clone, PartialEq)]

enum Operand {
    Value(f64),
    Old,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

impl Operation {
    fn new_op(operation_str: &str) -> Operation {
        let operation_parts: Vec<&str> = operation_str.split_whitespace().skip(4).collect();
        let operand: Operand;
        if operation_parts[1] == "old" {
            operand = Operand::Old;
        } else {
            operand = Operand::Value(operation_parts[1].parse::<f64>().unwrap());
        }
        return Operation {
            operator: Operator::new(operation_parts[0].chars().next().unwrap()),
            operand,
        };
    }

    fn new_test(test_str: &str) -> Operation {
        let test_parts: Vec<&str> = test_str.split_whitespace().collect();
        let operand = Operand::Value(test_parts[test_parts.len() - 1].parse::<f64>().unwrap());
        return Operation {
            operator: Operator::Modulo,
            operand,
        };
    }
}
#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<f64>,
    operation: Operation,
    test: Operation,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    inspection_count: u32,
}

impl Monkey {
    fn new(input_monkey: &Vec<&str>) -> Monkey {
        let mut tmp_items: Vec<&str> = input_monkey[1].split_whitespace().collect();
        let last_item = tmp_items.pop().unwrap().parse::<f64>().unwrap();
        let mut items: VecDeque<f64> = tmp_items
            .into_iter()
            .skip(2)
            .map(|item| item[..item.len() - 1].parse::<f64>().unwrap())
            .collect();
        items.push_back(last_item);
        let operation = Operation::new_op(input_monkey[2]);
        let test = Operation::new_test(input_monkey[3]);
        let true_monkey_vec: Vec<&str> = input_monkey[4].split_whitespace().collect();
        let true_monkey_idx = true_monkey_vec[true_monkey_vec.len() - 1]
            .parse::<usize>()
            .unwrap();
        let false_monkey_vec: Vec<&str> = input_monkey[5].split_whitespace().collect();
        let false_monkey_idx = false_monkey_vec[false_monkey_vec.len() - 1]
            .parse::<usize>()
            .unwrap();

        return Monkey {
            items,
            operation,
            test,
            true_monkey_idx,
            false_monkey_idx,
            inspection_count: 0,
        };
    }
}

fn execute_operation(operator: &Operator, operand: &Operand, val: f64) -> f64 {
    if *operator == Operator::Add {
        return match operand {
            Operand::Value(x) => val + x,
            Operand::Old => val + val,
        };
    } else if *operator == Operator::Subtract {
        return match operand {
            Operand::Value(x) => val - x,
            Operand::Old => val - val,
        };
    } else if *operator == Operator::Multiply {
        return match operand {
            Operand::Value(x) => val * x,
            Operand::Old => val * val,
        };
    } else if *operator == Operator::Divide {
        return match operand {
            Operand::Value(x) => f64::floor(val / x),
            Operand::Old => f64::floor(val / val),
        };
    } else if *operator == Operator::Modulo {
        return match operand {
            Operand::Value(x) => val % x,
            Operand::Old => val % val,
        };
    } else {
        panic!("bad");
    }
}

fn execute_round(monkey_vec: &mut Vec<Monkey>, super_divisor: f64, relief_factor: f64) {
    for monkey_idx in 0..monkey_vec.len() {
        while monkey_vec[monkey_idx].items.len() > 0 {
            monkey_vec[monkey_idx].inspection_count += 1;
            let true_idx = monkey_vec[monkey_idx].true_monkey_idx;
            let false_idx = monkey_vec[monkey_idx].false_monkey_idx;
            let item = monkey_vec[monkey_idx].items[0];
            let op_operator = &monkey_vec[monkey_idx].operation.operator;
            let op_operand = &monkey_vec[monkey_idx].operation.operand;
            let test_operator = &monkey_vec[monkey_idx].test.operator;
            let test_operand = &monkey_vec[monkey_idx].test.operand;

            let op_res = execute_operation(op_operator, op_operand, item);
            let new_op_res = f64::floor(op_res / relief_factor);
            let new_worry_level = new_op_res % super_divisor;
            let test_res = execute_operation(test_operator, test_operand, new_worry_level);
            monkey_vec[monkey_idx].items.pop_front();
            if test_res == 0.0 {
                monkey_vec[true_idx].items.push_back(new_worry_level);
            } else {
                monkey_vec[false_idx].items.push_back(new_worry_level);
            }
        }
    }
}

fn get_super_divisor(monkey_vec: &mut Vec<Monkey>) -> f64 {
    let mut super_divisor = 1.0;
    // let super_divisor = &monkey_vec.into_iter().fold(1, |acc, x| match x {Operand::Value(x) => })
    for monkey_idx in 0..monkey_vec.len() {
        super_divisor = match monkey_vec[monkey_idx].test.operand {
            Operand::Value(x) => super_divisor * x,
            Operand::Old => panic!(),
        }
    }
    return super_divisor;
}

fn part_one(monkey_vec: &mut Vec<Monkey>) {
    let super_divisor = get_super_divisor(monkey_vec);
    for round in 0..20 {
        // println!("Starting Round {}\nState\n{:#?}", round + 1, monkey_vec);
        execute_round(monkey_vec, super_divisor, 3.0);
    }
    // println!("===========\nFinal State\n{:#?}", monkey_vec);
    let mut top_inspections = [0, 0];
    for monkey in monkey_vec {
        if monkey.inspection_count > top_inspections[0] {
            top_inspections[0] = monkey.inspection_count;
            top_inspections.sort();
        }
    }
    let ans = top_inspections[0] * top_inspections[1];
    println!(
        "Top Inspections: {}, {}",
        top_inspections[0], top_inspections[1]
    );
    println!("Part One Ans: {}", ans);
}

fn part_two(monkey_vec: &mut Vec<Monkey>) {
    let super_divisor = get_super_divisor(monkey_vec);
    for round in 0..10_000 {
        execute_round(monkey_vec, super_divisor, 1.0);
        // if (round + 1) % 1000 == 0  || (round + 1) == 20 {
        //     println!("Finished Round {}\nState\n{:#?}", round + 1, monkey_vec);
        // }
    }
    // println!("===========\nFinal State\n{:#?}", monkey_vec);
    let mut top_inspections = [0, 0];
    for monkey in monkey_vec {
        if monkey.inspection_count > top_inspections[0] {
            top_inspections[0] = monkey.inspection_count;
            top_inspections.sort();
        }
    }
    println!(
        "Top Inspections: {}, {}",
        top_inspections[0], top_inspections[1]
    );
    let ans = top_inspections[0] as u128 * top_inspections[1] as u128;
    println!("Part One Ans: {}", ans);
}

fn main() {
    let mut starting_monkey_vec: Vec<Monkey> = vec![];
    let input_str = fs::read_to_string("./src/input.txt").unwrap();
    let input_monkeys: Vec<&str> = input_str.split("\n\n").collect();
    for input_monkey in input_monkeys {
        let input_monkey: Vec<&str> = input_monkey.split("\n").collect();
        starting_monkey_vec.push(Monkey::new(&input_monkey));
    }
    let mut part_one_monkey_vec = starting_monkey_vec.clone();
    let mut part_two_monkey_vec = starting_monkey_vec.clone();
    part_one(&mut part_one_monkey_vec);
    part_two(&mut part_two_monkey_vec);
}

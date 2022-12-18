use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

fn part_one(input_str: &String) {
    let mut current: VecDeque<char> = VecDeque::new();
    for (idx, in_char) in input_str.chars().enumerate() {
        current.push_back(in_char);
        if current.len() == 4 {
            let current_copy = current.clone();
            let unique_code: HashSet<char> = current_copy.into_iter().collect();
            if unique_code.len() == 4 {
                println!("{}", idx + 1);
                break;
            }
            else {
                current.pop_front();
            }
        }
    }
}

fn part_two(input_str: &String) {
    let mut current: VecDeque<char> = VecDeque::new();
    for (idx, in_char) in input_str.chars().enumerate() {
        current.push_back(in_char);
        if current.len() == 14 {
            let current_copy = current.clone();
            let unique_code: HashSet<char> = current_copy.into_iter().collect();
            if unique_code.len() == 14 {
                println!("{}", idx + 1);
                break;
            }
            else {
                current.pop_front();
            }
        }
    }

}

fn main() {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    part_one(&input_str);
    part_two(&input_str);
}

use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

fn column_lookup(index: usize) -> u32{
    if index == 1 {
        return 1;
    }
    else {
        let val = (index / 4) as u32;
        return val + 1;
    }
}

fn build_crate_map(crate_input: &str) -> HashMap<u32, Vec<char>> {
    let mut crate_map: HashMap<u32, Vec<char>> = HashMap::new();
    let crate_lines: Vec<&str> = crate_input.split("\n").collect();
    for crate_line in crate_lines {
        for (index, my_char) in crate_line.chars().enumerate() {
            if my_char.is_ascii_alphabetic() {
                let column = column_lookup(index);
                if crate_map.contains_key(&column) {
                    crate_map
                        .entry(column)
                        .and_modify(|vec| vec.insert(0, my_char));
                }
                else {
                    crate_map.insert(column, vec![my_char]);
                }
            }
        }
    }
    println!("{:#?}", crate_map);
    return crate_map;
}


fn execute_direction(direction: &str, mut crate_map: HashMap<u32, Vec<char>>) -> HashMap<u32, Vec<char>> {
    let mut direction_parts: VecDeque<u32> = VecDeque::new();
    for my_char in direction.chars() {
        if my_char.is_ascii_digit() {
            direction_parts.push_back(my_char.to_digit(10).expect("ahh"));
        }
    }
    let num_crates = direction_parts.pop_front().expect("ahh");
    let from_column = direction_parts.pop_front().expect("ahh");
    let to_column = direction_parts.pop_front().expect("ahh");
    for i in 0..num_crates {
        let thing = crate_map.get_mut(&from_column).expect("ahh");
        let val = thing.pop().expect("ahh");
        crate_map
            .entry(to_column)
            .and_modify(|vec| vec.push(val));
    }
    return crate_map;

}

fn print_top_crates(final_crate_map: HashMap<u32, Vec<char>>) {
    for i in 1..10 {
        print!("{}", final_crate_map.get(&i).expect("ahh").last().expect("ahh"));
    }
    println!("")
}
 
fn part_one(crate_input: &str, directions_input: &str) {
    let directions: Vec<&str> = directions_input.split("\n").collect();
    let mut crate_map = build_crate_map(crate_input);
    for direction in directions {
        println!("{}", direction);
        let mut direction_parts: VecDeque<u32> = VecDeque::new();
        let words: Vec<&str> = direction.split(" ").collect();
        for word in words {
            let test_digit = word.parse::<u32>();
            if test_digit.is_ok() {
                direction_parts.push_back(test_digit.unwrap());
            }
        }
        let num_crates = direction_parts.pop_front().expect("ahh");
        let from_column = direction_parts.pop_front().expect("ahh");
        let to_column = direction_parts.pop_front().expect("ahh");
        println!("crates to move: {}, from column: {}, to column {}", num_crates, from_column, to_column);
        println!("PRE-MOVE: from: {:?}  | to: {:?}", crate_map.get(&from_column), crate_map.get(&to_column));
        for _ in 0..num_crates {
            // println!("here {:?}", crate_map.get(&from_column));
            let from_column_vec = crate_map.get_mut(&from_column).expect("ahh");
            let val = from_column_vec.pop().expect("ahh");
            // println!("index {}, from column vec {:?}", i, from_column_vec);
            crate_map
                .entry(to_column)
                .and_modify(|vec| vec.push(val));
        }
        println!("POST-MOVE: from: {:?}  | to: {:?}", crate_map.get(&from_column), crate_map.get(&to_column));
    }
    print_top_crates(crate_map);
}

fn part_two(input_str: &str) {

}


fn main() {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    let crate_structure: Vec<&str> = input_str.split("\n\n").collect();
    part_one(crate_structure[0], crate_structure[1]);
}

use std::fs;
use std::collections::HashMap;


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
    return crate_map;
}


fn get_direction_tup(direction: &str) -> (u32, u32, u32) {
    let mut direction_vec: Vec<u32> = vec![];
    for word in direction.split_whitespace() {
        if let Ok(num) = word.parse::<u32>() {
            direction_vec.push(num);
        }
    }
    return (direction_vec[0], direction_vec[1], direction_vec[2]);

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
        let (num_crates, from_column, to_column) = get_direction_tup(direction);
        for _ in 0..num_crates {
            // println!("here {:?}", crate_map.get(&from_column));
            let from_column_vec = crate_map.get_mut(&from_column).expect("ahh");
            let val = from_column_vec.pop().expect("ahh");
            // println!("index {}, from column vec {:?}", i, from_column_vec);
            crate_map
                .entry(to_column)
                .and_modify(|vec| vec.push(val));
        }
    }
    print_top_crates(crate_map);
}

fn part_two(crate_input: &str, directions_input: &str) {
    let directions: Vec<&str> = directions_input.split("\n").collect();
    let mut crate_map = build_crate_map(crate_input);

    for direction in directions {
        let (num_crates, from_column, to_column) = get_direction_tup(direction);
        let from_column_vec = crate_map
            .get_mut(&from_column)
            .expect("key does not exist");
        let tmp_vec: Vec<char> = from_column_vec
            .splice(from_column_vec.len() -  num_crates as usize..from_column_vec.len(), vec![])
            .collect();
        crate_map
            .entry(to_column)
            .and_modify(|vec| vec.extend(tmp_vec));
    }
    print_top_crates(crate_map);
}


fn main() {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    let crate_structure: Vec<&str> = input_str.split("\n\n").collect();
    part_one(crate_structure[0], crate_structure[1]);
    part_two(crate_structure[0], crate_structure[1]);
}

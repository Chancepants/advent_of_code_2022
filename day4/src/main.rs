use std::fs;


fn get_assignment_length(assignment: &Vec<i32>) -> i32{
    let length = assignment[1] - assignment[0];
    length
}


fn assignment_contains_left_end(first_left: i32, second_left: i32) -> bool {
    if first_left <= second_left {
        return true;
    }
    false
}


fn assignment_contains_right_end(first_right: i32, second_right: i32) -> bool {
    if first_right >= second_right {
        return true;
    }
    false
}

fn assignment_full_overlaps(large_assignment_ends: &Vec<i32>, small_assignment_ends: &Vec<i32>) -> bool {
    if 
        assignment_contains_left_end(large_assignment_ends[0], small_assignment_ends[0])
        &&
        assignment_contains_right_end(large_assignment_ends[1], small_assignment_ends[1])
    {
        return true
    }
    false
}



fn get_assignment_ends(assignment: &str) -> Vec<i32> {
    let assignment_ends: Vec<i32> = assignment
    .split("-")
    .map(|s| s.parse().unwrap())
    .collect();
    assignment_ends
}


fn part_one(input_str: &str) {
    let input_lines: Vec<&str> = input_str.split("\n").collect();
    let mut sum: u32 = 0;
    for line in input_lines {
        let assignments: Vec<&str> = line.split(",").collect();
        let first_assignment_ends = get_assignment_ends(assignments[0]);
        let second_assignment_ends = get_assignment_ends(assignments[1]);
        let first_assignment_length = get_assignment_length(&first_assignment_ends);
        let second_assignment_length = get_assignment_length(&second_assignment_ends);
        let assignment_difference = first_assignment_length - second_assignment_length;
        let overlaps: bool;
        if assignment_difference > 0 {
            overlaps = assignment_full_overlaps(&first_assignment_ends, &second_assignment_ends);
        }
        else if assignment_difference < 0 {
            overlaps = assignment_full_overlaps(&second_assignment_ends, &first_assignment_ends);
        }
        else {
            overlaps = assignment_full_overlaps(&second_assignment_ends, &first_assignment_ends);
        }
        if overlaps {
            sum += 1;
        }
    }
    println!("{}", sum)
}


fn assignment_overlaps(first_assignment_ends: &Vec<i32>, second_assignment_ends: &Vec<i32>) -> bool {
    if first_assignment_ends[1] < second_assignment_ends[0] || second_assignment_ends[1] < first_assignment_ends[0] {
        return false
    }
    else {
        return true
    }
}


fn part_two(input_str: &str) {
    let input_lines: Vec<&str> = input_str.split("\n").collect();
    let mut sum: u32 = 0;
    for line in input_lines {
        let assignments: Vec<&str> = line.split(",").collect();
        let first_assignment_ends = get_assignment_ends(assignments[0]);
        let second_assignment_ends = get_assignment_ends(assignments[1]);
        if assignment_overlaps(&first_assignment_ends, &second_assignment_ends) {
            sum += 1;
        }
    }
    println!("{}", sum)
}


fn main() {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    part_one(&input_str);
    part_two(&input_str);
}



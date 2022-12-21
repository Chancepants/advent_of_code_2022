use std::fs;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn build_tree_map(input_str: &String) -> Vec<Vec<u32>> {
    let input_rows: Vec<&str> = input_str.split("\n").collect();
    let mut tree_map: Vec<Vec<u32>> = vec![];
    for (idx, row) in input_rows.into_iter().enumerate() {
        tree_map.push(vec![]);
        for my_char in row.chars() {
            tree_map[idx].push(my_char.to_digit(10).unwrap());
        }
    }
    return tree_map;
}

fn traverse_direction(
    tree_map: &Vec<Vec<u32>>,
    current_row: usize,
    current_column: usize,
    direction: Direction,
    tree_value: u32,
) -> bool {
    if direction == Direction::Left {
        if current_row == 0 {
            return true;
        }
        let new_row = current_row - 1;
        let new_val = tree_map[current_column][new_row];
        if new_val < tree_value {
            return traverse_direction(tree_map, new_row, current_column, direction, tree_value);
        }
        return false;
    } else if direction == Direction::Right {
        if current_row == tree_map[0].len() - 1 {
            return true;
        }
        let new_row = current_row + 1;
        let new_val = tree_map[current_column][new_row];
        if new_val < tree_value {
            return traverse_direction(tree_map, new_row, current_column, direction, tree_value);
        }
        return false;
    } else if direction == Direction::Up {
        if current_column == 0 {
            return true;
        }
        let new_column = current_column - 1;
        let new_val = tree_map[new_column][current_row];
        if new_val < tree_value {
            return traverse_direction(tree_map, current_row, new_column, direction, tree_value);
        }
        return false;
    } else if direction == Direction::Down {
        if current_column == tree_map.len() - 1 {
            return true;
        }
        let new_column = current_column + 1;
        let new_val = tree_map[new_column][current_row];
        if new_val < tree_value {
            return traverse_direction(tree_map, current_row, new_column, direction, tree_value);
        }
        return false;
    } else {
        panic!("invalid direction");
    }
}

fn traverse_all(tree_map: &Vec<Vec<u32>>, current_row: usize, current_column: usize) -> u32 {
    let tree_value = tree_map[current_column][current_row];
    let visible_left = traverse_direction(
        tree_map,
        current_row,
        current_column,
        Direction::Left,
        tree_value,
    );
    let visible_right = traverse_direction(
        tree_map,
        current_row,
        current_column,
        Direction::Right,
        tree_value,
    );
    let visible_up = traverse_direction(
        tree_map,
        current_row,
        current_column,
        Direction::Up,
        tree_value,
    );
    let visible_down = traverse_direction(
        tree_map,
        current_row,
        current_column,
        Direction::Down,
        tree_value,
    );
    // println!("Row: {}, Column: {}, Visible Left: {}, Visible Right: {}, Visible Up: {}, Visible Down: {}", current_row, current_column, visible_left, visible_right, visible_up, visible_down);
    if visible_left || visible_right || visible_up || visible_down {
        // println!("Row: {}, Column: {}, Visible: {}", current_row, current_column, 1);
        return 1;
    } else {
        // println!("Row: {}, Column: {}, Visible: {}", current_row, current_column, 0);
        return 0;
    }
}

fn traverse_direction_and_sum(
    tree_map: &Vec<Vec<u32>>,
    current_row: usize,
    current_column: usize,
    direction: Direction,
    tree_value: u32,
    mut acc: u32,
) -> u32 {
    if direction == Direction::Left {
        if current_row == 0 {
            return acc;
        }
        let new_row = current_row - 1;
        let new_val = tree_map[current_column][new_row];
        if new_val < tree_value {
            acc += 1;
            return traverse_direction_and_sum(
                tree_map,
                new_row,
                current_column,
                direction,
                tree_value,
                acc,
            );
        }
        return acc + 1;
    } else if direction == Direction::Right {
        if current_row == tree_map[0].len() - 1 {
            return acc;
        }
        let new_row = current_row + 1;
        let new_val = tree_map[current_column][new_row];
        if new_val < tree_value {
            acc += 1;
            return traverse_direction_and_sum(
                tree_map,
                new_row,
                current_column,
                direction,
                tree_value,
                acc,
            );
        }
        return acc + 1;
    } else if direction == Direction::Up {
        if current_column == 0 {
            return acc;
        }
        let new_column = current_column - 1;
        let new_val = tree_map[new_column][current_row];
        if new_val < tree_value {
            acc += 1;
            return traverse_direction_and_sum(
                tree_map,
                current_row,
                new_column,
                direction,
                tree_value,
                acc,
            );
        }
        return acc + 1;
    } else if direction == Direction::Down {
        if current_column == tree_map.len() - 1 {
            return acc;
        }
        let new_column = current_column + 1;
        let new_val = tree_map[new_column][current_row];
        if new_val < tree_value {
            acc += 1;
            return traverse_direction_and_sum(
                tree_map,
                current_row,
                new_column,
                direction,
                tree_value,
                acc,
            );
        }
        return acc + 1;
    } else {
        panic!("invalid direction");
    }
}

fn traverse_all_and_compute_scenic(
    tree_map: &Vec<Vec<u32>>,
    current_row: usize,
    current_column: usize,
) -> u32 {
    let tree_value = tree_map[current_column][current_row];
    let left = traverse_direction_and_sum(
        tree_map,
        current_row,
        current_column,
        Direction::Left,
        tree_value,
        0,
    );
    let right = traverse_direction_and_sum(
        tree_map,
        current_row,
        current_column,
        Direction::Right,
        tree_value,
        0,
    );
    let up = traverse_direction_and_sum(
        tree_map,
        current_row,
        current_column,
        Direction::Up,
        tree_value,
        0,
    );
    let down = traverse_direction_and_sum(
        tree_map,
        current_row,
        current_column,
        Direction::Down,
        tree_value,
        0,
    );

    return left * right * up * down;
}

fn part_one(tree_map: &Vec<Vec<u32>>) {
    let mut sum: u32 = 0;
    // iterate through the whole thing
    // if we are at an edge then do not traverse but add 1 to total
    // otherwise traverse directions until we've traversed all directions or got a true
    for i in 0..tree_map.len() {
        for j in 0..tree_map[0].len() {
            sum += traverse_all(tree_map, j, i);
        }
    }
    println!("Part One Ans: {}", sum);
}

fn part_two(tree_map: &Vec<Vec<u32>>) {
    let mut top_score = 0;
    for i in 0..tree_map.len() {
        for j in 0..tree_map[0].len() {
            let new_scenic_score = traverse_all_and_compute_scenic(tree_map, j, i);
            if top_score < new_scenic_score {
                top_score = new_scenic_score;
            }
        }
    }
    println!("Part Two Ans: {}", top_score);
}

fn main() {
    let input_str = fs::read_to_string("./src/input.txt").unwrap();
    let tree_map = build_tree_map(&input_str);
    part_one(&tree_map);
    part_two(&tree_map);
}

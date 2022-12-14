use std::fs;
use criterion::{black_box, Criterion, criterion_group, criterion_main};

const MAX_DIR_SIZE: u32 = 100_000;

struct ArenaTree {
    arena: Vec<FileNode>,
}

impl ArenaTree {
    fn new() -> Self {
        Self { arena: vec![] }
    }

    fn add_node(&mut self, new_node: FileNode, current_node_idx: Option<usize>) -> Option<usize> {
        if current_node_idx.is_some() {
            for &child_idx in &self.arena[current_node_idx.unwrap()].children {
                if self.arena[child_idx].filename == new_node.filename {
                    return None;
                }
            }
        }
        self.arena.push(new_node);
        return Some(self.arena.len() - 1);
    }

    fn traverse_and_sum(&self, root: usize) -> u32 {
        let mut total_size = 0;
        if self.arena[root].filesize > 0 {
            return self.arena[root].filesize;
        }
        for &child in &self.arena[root].children {
            total_size += self.traverse_and_sum(child);
        }
        return total_size;
    }


    fn traverse_and_sum_vec(&self, root: usize, dir_size_vec: &mut Vec<u32>) -> u32 {
        let mut total_size = 0;
        if self.arena[root].filesize > 0 {
            return self.arena[root].filesize;
        }
        for &child in &self.arena[root].children {
            total_size += self.traverse_and_sum_vec(child, dir_size_vec);
        }
        if total_size <= MAX_DIR_SIZE {
            dir_size_vec.push(total_size);
        }
        return total_size;
    }


    fn print(&self, root: usize, indentation: usize) {
        let indent = String::from(" ").repeat(indentation);
        println!(
            "{}index: {}, filename: {}, filesize: {}, parent: {}, children: {:?}",
            indent,
            self.arena[root].idx,
            self.arena[root].filename,
            self.arena[root].filesize,
            self.arena[root].parent.unwrap_or_default(),
            self.arena[root].children
        );
        for &child in &self.arena[root].children {
            self.print(child, indentation + 2);
        }
    }
}

struct FileNode {
    idx: usize,
    filename: String,
    filesize: u32,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl FileNode {
    fn new(idx: usize, filename: String, filesize: u32) -> Self {
        Self {
            idx,
            filename,
            filesize,
            parent: None,
            children: vec![],
        }
    }
}

fn is_command(input_line: &str) -> bool {
    if input_line.chars().nth(0).unwrap() == '$' {
        return true;
    } else {
        return false;
    }
}

fn execute_cd(new_dir_name: &str, arena_tree: &mut ArenaTree, current_node_idx: usize) -> usize {
    if new_dir_name == "/" {
        if arena_tree.arena.len() > 0 {
            panic!("Expected arena tree to be empty when cd'ing to root directory")
        }
        let root = FileNode::new(0, "/".to_string(), 0);
        arena_tree.add_node(root, None);
        return 0;
    } else if new_dir_name == ".." {
        return arena_tree.arena[current_node_idx].parent.unwrap();
    } else {
        for child_idx in &arena_tree.arena[current_node_idx].children {
            if arena_tree.arena[*child_idx].filename == new_dir_name.to_string() {
                return *child_idx;
            }
        }
        panic!("couldn't find child!");
    }
}

fn execute_ls(
    directory_content: &Vec<String>,
    arena_tree: &mut ArenaTree,
    current_node_idx: usize,
) {
    // iterate through `directory_content` and ensure
    for file_description in directory_content {
        let file_parts: Vec<&str> = file_description.split_whitespace().collect();
        let mut filesize = 0;
        if file_parts[0] != "dir" {
            filesize = file_parts[0].parse().unwrap();
        }
        let mut new_node =
            FileNode::new(arena_tree.arena.len(), file_parts[1].to_string(), filesize);
        new_node.parent = Some(current_node_idx);
        let child_idx = arena_tree.add_node(new_node, Some(current_node_idx));
        if child_idx.is_some() {
            // println!("adding child with idx {} to current node with idx {}", child_idx.unwrap(), current_node_idx);
            arena_tree.arena[current_node_idx]
                .children
                .push(child_idx.unwrap());
        }
    }
}

fn execute_command(
    command: &String,
    command_output: &Vec<String>,
    arena_tree: &mut ArenaTree,
    current_node_idx: usize,
) -> usize {
    let mut new_node_idx = current_node_idx;
    let mut command_parts: Vec<&str> = command.split_whitespace().collect();
    command_parts.remove(0);
    if command_parts[0] == "cd" {
        new_node_idx = execute_cd(command_parts[1], arena_tree, current_node_idx);
    } else if command_parts[0] == "ls" {
        execute_ls(command_output, arena_tree, current_node_idx);
    } else {
        panic!("unexpected command");
    }
    return new_node_idx;
}

fn get_next_command(index: usize, input_lines: &Vec<&str>) -> (String, Vec<String>, usize) {
    let mut command = String::new();
    let mut command_output: Vec<String> = vec![];
    let mut new_index = index;
    for (idx, &input_line) in input_lines.into_iter().skip(index).enumerate() {
        if idx == 0 && is_command(input_line) == false {
            panic!("Unexpected line in input file: {}", input_line)
        } else if idx == 0 && is_command(input_line) {
            command = input_line.to_string();
        } else if idx > 0 && is_command(input_line) == true {
            new_index += idx;
            break;
        } else {
            command_output.push(input_line.to_string());
            if index + idx == input_lines.len() - 1 {
                new_index += idx + 1;
                break;
            }
        }
    }
    return (command, command_output, new_index);
}

fn calculate_total(arena_tree: &ArenaTree, root: usize, mut grand_total: u32) -> u32 {
    let root_total = arena_tree.traverse_and_sum(root);
    if root_total <= MAX_DIR_SIZE && arena_tree.arena[root].filesize == 0 {
        grand_total += root_total;
    }
    for &child in &arena_tree.arena[root].children {
        grand_total = calculate_total(arena_tree, child, grand_total);
    }
    return grand_total;
}

fn search_closest(arena_tree: &ArenaTree, root: usize, target_size: u32, mut closest: u32) -> u32 {
    let root_total = arena_tree.traverse_and_sum(root);
    if root_total >= target_size && root_total < closest && arena_tree.arena[root].filesize == 0 {
        closest = root_total;
    }
    for &child in &arena_tree.arena[root].children {
        closest = search_closest(arena_tree, child, target_size, closest);
    }
    return closest;
}

fn part_one(arena_tree: &ArenaTree) {
    let root: usize = 0;
    let ans = calculate_total(&arena_tree, root, 0);
}

fn part_one_better(arena_tree: &ArenaTree) {
    let mut init_vec: Vec<u32> = vec![];
    let mut total: u32 = 0;
    arena_tree.traverse_and_sum_vec(0, &mut init_vec);
    for val in init_vec {
        total += val;
    }
}

fn criterion_bench_part_one(c: &mut Criterion) {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    let input_lines: Vec<&str> = input_str.split("\n").collect();
    let mut current_idx = 0;
    let mut current_node_idx = 0;
    let mut arena_tree = ArenaTree::new();
    while current_idx < input_lines.len() {
        let (command, command_output, next_idx) = get_next_command(current_idx, &input_lines);
        current_node_idx =
            execute_command(&command, &command_output, &mut arena_tree, current_node_idx);
        current_idx = next_idx;
    }
    c.bench_function("slower hopefully", |b| b.iter(|| part_one(black_box(&arena_tree))));
}

fn criterion_bench_part_one_better(c: &mut Criterion) {
    let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
    let input_lines: Vec<&str> = input_str.split("\n").collect();
    let mut current_idx = 0;
    let mut current_node_idx = 0;
    let mut arena_tree = ArenaTree::new();
    while current_idx < input_lines.len() {
        let (command, command_output, next_idx) = get_next_command(current_idx, &input_lines);
        current_node_idx =
            execute_command(&command, &command_output, &mut arena_tree, current_node_idx);
        current_idx = next_idx;
    }
    c.bench_function("faster hopefully", |b| b.iter(|| part_one_better(black_box(&arena_tree))));
}

criterion_group!(benches, criterion_bench_part_one, criterion_bench_part_one_better);
criterion_main!(benches);

fn part_two(arena_tree: &ArenaTree) {
    let total_disk: u32 = 70_000_000;
    let required_free: u32 = 30_000_000;
    let total_used = arena_tree.traverse_and_sum(0);
    let total_remaining = total_disk - total_used;
    println!("Total Remaining: {}", total_remaining);
    let target_dir_size = required_free - total_remaining;
    let ans = search_closest(arena_tree, 0, target_dir_size, total_disk);
    println!("Target Dir Size: {}", target_dir_size);
    println!("Ans {}", ans);
}

// fn main() {
//     let input_str: String = fs::read_to_string("./src/input.txt").expect("failed to read file");
//     let input_lines: Vec<&str> = input_str.split("\n").collect();
//     let mut current_idx = 0;
//     let mut current_node_idx = 0;
//     let mut arena_tree = ArenaTree::new();
//     while current_idx < input_lines.len() {
//         let (command, command_output, next_idx) = get_next_command(current_idx, &input_lines);
//         current_node_idx =
//             execute_command(&command, &command_output, &mut arena_tree, current_node_idx);
//         current_idx = next_idx;
//     }
//     arena_tree.print(0, 0);

//     part_one(&arena_tree);
    // part_two(&arena_tree);
// }

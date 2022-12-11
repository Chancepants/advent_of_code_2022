use std::collections::HashSet;
use std::fs;

const UPPER_ORD_DIFF: u32 = 38;
const LOWER_ORD_DIFF: u32 = 96;


fn calculate_priority(item: char)  -> Option<u32> {
    let item_val: u32 = item as u32;
    let priority: u32;

    if item.is_ascii_uppercase() {
        priority = item_val - UPPER_ORD_DIFF;
    }
    else if item.is_ascii_lowercase() {
        priority = item_val - LOWER_ORD_DIFF;
    }
    else {
        return None;
    }

    Some(priority)
}


fn part_one(rucksacks: &Vec<&str>) {
    let mut sum: u32 = 0;
    for rucksack in rucksacks.iter() {
        let mut first_compartment_set: HashSet<char> = HashSet::new();
        let mut second_compartment_set: HashSet<char> = HashSet::new();
        let midpoint: usize = rucksack.chars().count() / 2;
        for (idx, item) in rucksack.chars().enumerate() {
            if idx < midpoint {
                first_compartment_set.insert(item);
            }
            else {
                second_compartment_set.insert(item);
            }
        }
        let dupes = first_compartment_set.intersection(&second_compartment_set);
        for &dupe in dupes {
            let priority = calculate_priority(dupe).expect("input char was not ascii lower or ascii upper");
            sum += priority;
        }
    }
    println!("{}", sum);
}

fn part_two(rucksacks: &Vec<&str>) {
    let mut sum: u32 = 0;
    for group in rucksacks.chunks(3) {
        let first_elf_set: HashSet<char> = HashSet::from_iter(group[0].chars());
        let second_elf_set: HashSet<char> = HashSet::from_iter(group[1].chars());
        let third_elf_set: HashSet<char> = HashSet::from_iter(group[2].chars());
        let first_intersection : HashSet<char> = first_elf_set.intersection(&second_elf_set).copied().collect();
        let final_dupes = first_intersection.intersection(&third_elf_set);
        
        for &dupe in final_dupes {
            let priority = calculate_priority(dupe).expect("input char was not ascii lower or ascii upper");
            sum += priority;
        }
    }
    println!("{}", sum)
}

fn main() {
    let input: String = fs::read_to_string("./src/input1.txt").expect("Should have read a file");
    let rucksacks: Vec<&str> = input.split("\n").collect();
    part_one(&rucksacks);
    part_two(&rucksacks);

}

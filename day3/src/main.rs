use std::collections::HashSet;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./src/input1.txt").expect("Should have read a file");
    let rucksacks: Vec<&str> = input.split("\n").collect();
    let upper_ord_diff: u32 = 38;
    let lower_ord_diff: u32 = 96;
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
        for dupe in dupes {
            let dupe_val: u32 = *dupe as u32;
            let mut priority: u32 = 0;
            if dupe.is_ascii_uppercase() {
                priority = dupe_val - upper_ord_diff;
            }
            else if dupe.is_ascii_lowercase() {
                priority = dupe_val - lower_ord_diff;
            }
            println!("{}: {}", dupe, priority);
            sum += priority;
        }
    }
    println!("{}", sum)

}

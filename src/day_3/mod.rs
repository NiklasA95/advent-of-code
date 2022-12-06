use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part_1() -> Result<()> {
    let mut file = File::open("./src/day_3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut priority_sum: usize = 0;

    for rucksack in contents.lines() {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);

        let mut matching_item: &str = "";
        for item_type_1 in compartment_1.split_terminator("").skip(1) {
            for item_type_2 in compartment_2.split_terminator("").skip(1) {
                if item_type_1 == item_type_2 {
                    matching_item = item_type_1;
                    break;
                }
            }
            if !matching_item.is_empty() {
                break;
            }
        }

        let priorities = [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z",
        ];

        let mut priority = 0;
        for (index, str) in priorities.iter().enumerate() {
            if matching_item == *str {
                priority = index + 1;
                break;
            }
        }

        priority_sum += priority;
    }

    println!("Solution Part 1: {}", priority_sum);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut file = File::open("./src/day_3/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut priority_sum: usize = 0;

    let mut index = 0;
    let rucksacks: Vec<&str> = contents.lines().collect();
    while index < (rucksacks.len()) {
        let mut badge: &str = "";

        for item_type_rucksack_1 in rucksacks[index].split_terminator("").skip(1) {
            for item_type_rucksack_2 in rucksacks[index + 1].split_terminator("").skip(1) {
                if item_type_rucksack_1 == item_type_rucksack_2 {
                    for item_type_rucksack_3 in rucksacks[index + 2].split_terminator("").skip(1) {
                        if item_type_rucksack_3 == item_type_rucksack_2 {
                            badge = item_type_rucksack_3;
                            index += 3;
                            break;
                        }
                    }
                }

                if !badge.is_empty() {
                    break;
                }
            }

            if !badge.is_empty() {
                break;
            }
        }

        let priorities = [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z",
        ];

        let mut priority = 0;
        for (index, str) in priorities.iter().enumerate() {
            if badge == *str {
                priority = index + 1;
                break;
            }
        }

        priority_sum += priority;
    }

    println!("Solution Part 2: {}", priority_sum);

    Ok(())
}

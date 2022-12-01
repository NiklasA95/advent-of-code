use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part_1() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let split = contents.split("\n\n");
    let mut max_calories: i32 = 0;

    for portion in split {
        let mut calories: i32 = 0;

        for item in portion.lines() {
            calories += item.parse::<i32>().unwrap();
        }

        if calories > max_calories {
            max_calories = calories;
        }
    }

    println!("Solution Part 1: {}", max_calories);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let split = contents.split("\n\n");
    let mut three_max_calories: [i32; 3] = [0, 0, 0];

    for portion in split {
        let mut calories: i32 = 0;

        for item in portion.lines() {
            calories += item.parse::<i32>().unwrap();
        }

        for max_calories in three_max_calories.iter_mut() {
            if calories > *max_calories {
                *max_calories = calories;
                break;
            }
        }
    }

    let mut three_max_calories_total: i32 = 0;

    for max_calories in three_max_calories {
        println!("Max Calories: {}", max_calories);
        three_max_calories_total += max_calories;
    }

    println!("Solution Part 2: {}", three_max_calories_total);

    Ok(())
}

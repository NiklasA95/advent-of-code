use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part_1() -> Result<()> {
    let mut file = File::open("./src/day_1/input.txt")?;
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
    let mut file = File::open("./src/day_1/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let blocks = contents.split("\n\n");
    let mut portions = Vec::new();

    for portion in blocks {
        let mut calories: i32 = 0;

        for item in portion.lines() {
            calories += item.parse::<i32>().unwrap();
        }

        portions.push(calories);
    }

    for i in 1..portions.len() {
        let cur = portions[i];
        let mut j = i - 1;

        while portions[j] > cur {
            portions[j + 1] = portions[j];
            if j == 0 {
                break;
            }
            j -= 1;
        }

        // we exit the loop from that break statement
        if j == 0 && portions[0] > cur {
            portions[0] = cur;
        } else {
            // `portions[j] > cur` is not satsified, exit from condition judgement
            portions[j + 1] = cur;
        }
    }

    let sum_top_three_portions =
        portions[portions.len() - 1] + portions[portions.len() - 2] + portions[portions.len() - 3];

    println!("Solution Part 2: {}", sum_top_three_portions);

    Ok(())
}

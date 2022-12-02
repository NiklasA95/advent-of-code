use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

pub fn part_1() -> Result<()> {
    let mut file = File::open("./src/day_2/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_score: i32 = 0;

    for round in contents.lines() {
        let mut round_score: i32 = 0;
        let (opponent_shape, my_shape) = round.split_at(round.find(' ').unwrap());

        match opponent_shape {
            "A" => {
                match my_shape {
                    " X" => round_score += 4,
                    " Y" => round_score += 8,
                    " Z" => round_score += 3,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            "B" => {
                match my_shape {
                    " X" => round_score += 1,
                    " Y" => round_score += 5,
                    " Z" => round_score += 9,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            "C" => {
                match my_shape {
                    " X" => round_score += 7,
                    " Y" => round_score += 2,
                    " Z" => round_score += 6,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            _ => println!("Unknown hand shape!"),
        }

        total_score += round_score
    }

    println!("Solution Part 1: {}", total_score);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut file = File::open("./src/day_2/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut total_score: i32 = 0;

    for round in contents.lines() {
        let mut round_score: i32 = 0;
        let (opponent_shape, my_shape) = round.split_at(round.find(' ').unwrap());

        match opponent_shape {
            "A" => {
                match my_shape {
                    " X" => round_score += 3,
                    " Y" => round_score += 4,
                    " Z" => round_score += 8,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            "B" => {
                match my_shape {
                    " X" => round_score += 1,
                    " Y" => round_score += 5,
                    " Z" => round_score += 9,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            "C" => {
                match my_shape {
                    " X" => round_score += 2,
                    " Y" => round_score += 6,
                    " Z" => round_score += 7,
                    _ => println!("Unknown opponent hand shape!"),
                };
            }
            _ => println!("Unknown hand shape!"),
        }

        total_score += round_score
    }

    println!("Solution Part 2: {}", total_score);

    Ok(())
}

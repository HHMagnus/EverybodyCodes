mod input;

use std::env;
use dotenv;
use std::fmt::Display;
use std::fs::read_to_string;

fn solve_part<F, R>(puzzle: String, file: &str, solver: F)
where F: Fn(&str) -> R,
      R: Display
{
    let result = solver(file);
    println!("{}: {}", puzzle, result);
}

pub fn solve<F1, F2, F3, R1, R2, R3>(puzzle: &str, part1: F1, part2: F2, part3: F3)
where F1: Fn(&str) -> R1,
      R1: Display,
      F2: Fn(&str) -> R2,
      R2: Display,
      F3: Fn(&str) -> R3,
      R3: Display,
{
    dotenv::dotenv().ok();
    run_part(puzzle, part1, "1");
    run_part(puzzle, part2, "2");
    run_part(puzzle, part3, "3");
}

fn run_part<F, R>(puzzle: &str, solver: F, part: &str)
where
    F: Fn(&str) -> R,
    R: Display
{
    let title = format!("Quest {} part {}", puzzle, part);
    let input = get_input(puzzle, part);
    solve_part(title, &input, solver);
}

fn get_input(quest: &str, part: &str) -> String {
    let dir = format!("input/quest{}", quest);
    let file_name = format!("{}/input{}.txt", dir, part);
    let file = read_to_string(&file_name);

    if let Ok(file) = file {
        if !file.is_empty() {
            return file;
        }
    }

    println!("Input file not found. Retrieving...");

    let uuid = env::var("SEED")
        .expect("SEED env var not set");

    let input = input::get_input(quest, &uuid);

    let output = match part {
        "1" => input.0,
        "2" => input.1,
        "3" => input.2,
        _ => panic!("Unknown part {}", part)
    }.expect(&format!("Quest {} part {} input not found", quest, part));

    std::fs::create_dir_all(&dir)
        .expect("Failed to create dir");
    std::fs::write(&file_name, &output)
        .expect("Unable to write file");

    output
}
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
    run_part(puzzle, part1, "1");
    run_part(puzzle, part2, "2");
    run_part(puzzle, part3, "3");
}

fn run_part<F, R>(puzzle: &str, part1: F, input: &str)
where
    F: Fn(&str) -> R,
    R: Display
{
    let title = format!("Quest {} part {}", puzzle, input);
    let file = read_to_string(format!("input/quest{}/input{}.txt", puzzle, input));
    if let Ok(file) = file {
        solve_part(title, &file, part1);
    } else {
        println!("{}: Input missing", title);
    }
}
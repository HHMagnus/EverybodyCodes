use std::fmt::Display;

pub fn solve<'a, F, R>(puzzle: &'a str, file: &'a str, solver: F)
where F: Fn(&'a str) -> R,
      R: Display
{
    let result = solver(file);
    println!("{}: {}", puzzle, result);
}
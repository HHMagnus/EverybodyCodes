use std::collections::HashMap;
use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("18", part1, part2, part3);
}

#[derive(Debug)]
struct Plant {
    id: usize,
    thickness: i64,
    branches: Vec<Branch>,
}

#[derive(Debug)]
enum Branch {
    Free(i64),
    Other(usize, i64)
}

impl Plant {
    fn parse(file: &str) -> (Vec<Plant>, Vec<Vec<i64>>) {
        let mut overall = file.split("\n\n\n");
        let plants = Self::parse_list(overall.next().unwrap());
        let activations = overall.next().unwrap().lines().map(|line| {
            line.split(" ").map(|i| i.parse::<i64>().unwrap()).collect()
        }).collect();
        (plants, activations)
    }

    fn parse_list(file: &str) -> Vec<Plant> {
        file.split("\n\n")
            .map(|plant| {
                let mut plants = plant.split('\n');
                let plant_line = plants.next().unwrap();
                let mut plant_line_split = plant_line.split(" with thickness ");
                let plant_id = plant_line_split.next().unwrap()
                    .replace("Plant ", "").parse::<usize>().unwrap();
                let plant_thickness = plant_line_split.next().unwrap()
                    .replace(":", "").parse::<i64>().unwrap();

                let branches = plants.into_iter().map(|branch| {
                    let free_branch_text = "- free branch with thickness ";
                    if branch.starts_with(free_branch_text) {
                        let free_branch_thickness = branch[free_branch_text.len()..].parse::<usize>().unwrap();
                        return Branch::Free(free_branch_thickness as i64)
                    }

                    let mut split = branch.split(" with thickness ");
                    let other_plant_id = split.next().unwrap()
                        .replace("- branch to Plant ", "").parse::<usize>().unwrap();
                    let other_plant_thickness = split.next().unwrap()
                        .parse::<i64>().unwrap();
                    return Branch::Other(other_plant_id, other_plant_thickness);
                }).collect::<Vec<_>>();

                Plant::new(plant_id, plant_thickness, branches)
            }).collect()
    }

    fn new(id: usize, thickness: i64, branches: Vec<Branch>) -> Plant {
        Plant { id, thickness, branches }
    }
}

fn part1(file: &str) -> i64 {
    let plants = Plant::parse_list(file);

    activate(&plants, vec![])
}

fn activate(plants: &Vec<Plant>, forced_activation: Vec<i64>) -> i64 {
    let mut forced_activation = forced_activation.into_iter();
    let mut plant_energy = HashMap::new();

    for plant in plants {
        let flow = plant.branches.iter()
            .map(|branch| {
                match branch {
                    Branch::Free(thickness) => forced_activation.next().unwrap_or(*thickness),
                    Branch::Other(id, thickness) => plant_energy.get(id).unwrap() * thickness,
                }
            }).sum();
        if plant.thickness > flow {
            plant_energy.insert(plant.id, 0);
        } else {
            plant_energy.insert(plant.id, flow);
        }
    }

    plant_energy[&plants[plants.len() - 1].id]
}

fn part2(file: &str) -> i64 {
    let (plants, activations) = Plant::parse(file);

    activations.into_iter()
        .map(|activation| activate(&plants, activation))
        .sum()
}

fn part3(file: &str) -> i32 {
    let (plants, activations) = Plant::parse(file);

    let ducks = activations.into_iter()
        .map(|activation| activate(&plants, activation))
        .filter(|&x| x != 0)
        .collect::<Vec<_>>();


    123
}
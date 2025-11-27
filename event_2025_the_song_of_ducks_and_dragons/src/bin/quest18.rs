use event_2025_the_song_of_ducks_and_dragons::solve;

fn main() {
    solve("18", part1, part2, part3);
}

#[derive(Debug)]
struct Plant {
    id: usize,
    thickness: usize,
    branches: Vec<Branch>,
}

#[derive(Debug)]
enum Branch {
    Free(usize),
    Other(usize, usize)
}

impl Plant {
    fn parse(file: &str) -> Vec<Plant> {
        file.split("\n\n")
            .map(|plant| {
                let mut plants = plant.split('\n');
                let plant_line = plants.next().unwrap();
                let mut plant_line_split = plant_line.split(" with thickness ");
                let plant_id = plant_line_split.next().unwrap()
                    .replace("Plant ", "").parse::<usize>().unwrap();
                let plant_thickness = plant_line_split.next().unwrap()
                    .replace(":", "").parse::<usize>().unwrap();

                let branches = plants.into_iter().map(|branch| {
                    let free_branch_text = "- free branch with thickness ";
                    if branch.starts_with(free_branch_text) {
                        let free_branch_thickness = branch[free_branch_text.len()..].parse::<usize>().unwrap();
                        return Branch::Free(free_branch_thickness)
                    }

                    let mut split = branch.split(" with thickness ");
                    let other_plant_id = split.next().unwrap()
                        .replace("- branch to Plant ", "").parse::<usize>().unwrap();
                    let other_plant_thickness = split.next().unwrap()
                        .parse::<usize>().unwrap();
                    return Branch::Other(other_plant_id, other_plant_thickness);
                }).collect::<Vec<_>>();

                Plant::new(plant_id, plant_thickness, branches)
            }).collect()
    }

    fn new(id: usize, thickness: usize, branches: Vec<Branch>) -> Plant {
        Plant { id, thickness, branches }
    }
}

fn part1(file: &str) -> usize {
    let plants = Plant::parse(file);
    println!("{:?}", plants);
    0
}

fn part2(file: &str) -> usize {
    0
}

fn part3(file: &str) -> usize {
    0
}
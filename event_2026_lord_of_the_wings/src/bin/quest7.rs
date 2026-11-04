use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/quest7/input1.txt").unwrap();

    let part1 = total_start_at_most_connections(file, 5);

    println!("Part 1: {}", part1);

    let file = read_to_string("input/quest7/input2.txt").unwrap();

    let part2 = total_start_at_most_connections(file, 10);

    println!("Part 2: {}", part2);

    let file = read_to_string("input/quest7/input3.txt").unwrap();

    let clouds = parse(file);

    let cloud_ids = clouds.keys()
        .cloned()
        .collect::<Vec<u32>>();

    let part3 = cloud_ids.into_iter()
        .map(|cloud_id| total_charge_after_power(clouds.clone(), cloud_id, 10, 1000000))
        .max()
        .unwrap();

    println!("Part 3: {}", part3);
}

#[derive(Clone, Debug)]
struct Cloud {
    id: u32,
    energy: u32,
    connections: Vec<u32>
}

impl Cloud {
    fn parse(line: &str) -> Cloud {
        let mut split = line.split(", ");
        let id = split.next().unwrap().replace("id=", "").parse().unwrap();
        let energy = split.next().unwrap().replace("energy=", "").parse().unwrap();
        let connections = split.next().unwrap().replace("connections=", "")
            .split(",")
            .map(|s| s.parse().unwrap()).collect::<Vec<u32>>();

        Cloud {
            id,
            energy,
            connections
        }
    }
}

fn total_start_at_most_connections(file: String, power: u32) -> u32 {
    let clouds = parse(file);

    let most_connections = clouds.values()
        .max_by_key(|cloud| cloud.connections.len())
        .unwrap()
        .id;

    total_charge_after_power(clouds, most_connections, power, u32::MAX)
}

fn parse(file: String) -> HashMap<u32, Cloud> {
    file.lines()
        .map(Cloud::parse)
        .map(|cloud| (cloud.id, cloud))
        .collect::<HashMap<_, _>>()
}

fn total_charge_after_power(mut clouds: HashMap<u32, Cloud>, start_at: u32, power: u32, explode_at: u32) -> u32 {
    charge(start_at, power, explode_at, &mut clouds);

    clouds.values()
        .map(|cloud| cloud.energy)
        .filter(|energy| *energy < explode_at)
        .sum::<u32>()
}

fn charge(id: u32, power: u32, explode_at: u32, clouds: &mut HashMap<u32, Cloud>) {
    let mut charge_wave = [(id, vec![(power, 1)])].into_iter().collect::<HashMap<_, _>>();

    while !charge_wave.is_empty() {
        let mut new_charge_wave = HashMap::new();

        for (&id, power) in charge_wave.iter() {
            let cloud = clouds.get_mut(&id).unwrap();
            if cloud.energy >= explode_at {
                continue;
            }

            let power_in_wave = power.iter().map(|&(energy, times)| energy * times).sum::<u32>();
            cloud.energy += power_in_wave;

            let mut power = power.iter()
                .map(|&(power, times)| (power - 1, times))
                .filter(|&(power, _)| power != 0)
                .collect::<Vec<_>>();

            if cloud.energy >= explode_at {
                power.push((10, 1));
            }

            if power.is_empty() {
                continue;
            }

            for connection in cloud.connections.iter() {
                let entry = new_charge_wave.entry(*connection).or_insert(vec![]);
                entry.extend(power.clone());
                entry.sort();
                *entry = entry.chunk_by(|(power1, _), (power2, _)| power1 == power2)
                    .map(|chunk| (chunk[0].0, chunk.iter().map(|(_, times)| times).sum::<u32>()))
                    .collect()
            }
        }

        charge_wave = new_charge_wave;
    }
}
use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

pub fn run() {
    let input = fs::read_to_string("./inputs/day08/input.txt")
        .expect("Should have been able to read the file");
    parts(&input);
}

fn parts(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut junctions: Vec<(i64, i64, i64)> = vec![];

    for line in lines.iter() {
        let coords: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        junctions.push((coords[0], coords[1], coords[2]));
    }

    let mut distances_map: HashMap<String, ((i64, i64, i64), (i64, i64, i64))> = HashMap::new();
    let mut distances: Vec<f64> = vec![];

    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            let distance = dist(junctions[j], junctions[i]);
            distances.push(distance);
            distances_map.insert(distance.to_string(), (junctions[i], junctions[j]));
        }
    }

    let mut circuits: Vec<HashSet<(i64, i64, i64)>> = vec![];
    for junction in junctions {
        circuits.push(HashSet::from([junction]));
    }
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distances.dedup();
    let mut count = 0;
    for distance in distances.iter() {
        if count == 1000 {
            circuits.sort_by(|a, b| b.len().cmp(&a.len()));
            println!(
                "p1: {}",
                circuits[0].len() * circuits[1].len() * circuits[2].len()
            );
        }
        count += 1;

        let j_pairs = distances_map[&distance.to_string()];
        let mut idx1 = None;
        let mut idx2 = None;
        for i in 0..circuits.len() {
            if circuits[i].contains(&j_pairs.0) {
                idx1 = Some(i);
            }
            if circuits[i].contains(&j_pairs.1) {
                idx2 = Some(i);
            }
        }
        if idx1 == idx2 {
            continue;
        }
        for junc in circuits[idx2.unwrap()]
            .iter()
            .cloned()
            .collect::<Vec<(i64, i64, i64)>>()
        {
            circuits[idx1.unwrap()].insert(junc);
        }
        circuits.remove(idx2.unwrap());

        if circuits.len() == 1 {
            println!("p2: {}", j_pairs.0.0 * j_pairs.1.0);
            break;
        }
    }
}

fn dist(junc1: (i64, i64, i64), junc2: (i64, i64, i64)) -> f64 {
    let res = ((junc2.0 - junc1.0) * (junc2.0 - junc1.0)
        + (junc2.1 - junc1.1) * (junc2.1 - junc1.1)
        + (junc2.2 - junc1.2) * (junc2.2 - junc1.2)) as f64;
    return res.sqrt();
}

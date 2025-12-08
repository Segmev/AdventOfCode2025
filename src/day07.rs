use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn run() {
    let input = fs::read_to_string("./inputs/day07/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut splitters: HashMap<(usize, usize), u64> = HashMap::new();
    let mut beams: Vec<(usize, usize)> = vec![];
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();

        for x in 0..line.len() {
            if line[x] == 'S' {
                beams.push((x, y));
            } else if line[x] == '^' {
                splitters.insert((x, y), 0);
            }
        }
    }

    let mut output = 0;
    let mut shined: HashSet<(usize, usize)> = HashSet::new();

    while beams.len() > 0 {
        let current_beam = beams.pop().unwrap();
        if current_beam.1 == lines.len() - 1 || shined.contains(&current_beam) {
            continue;
        }
        shined.insert(current_beam);

        match splitters.get(&(current_beam.0, current_beam.1 + 1)) {
            Some(_) => {
                output += 1;
                if current_beam.0 < lines[0].len() {
                    beams.push((current_beam.0 + 1, current_beam.1 + 1));
                }
                if current_beam.0 > 0 {
                    beams.push((current_beam.0 - 1, current_beam.1 + 1));
                }
            }
            _ => {
                beams.push((current_beam.0, current_beam.1 + 1));
            }
        }
    }

    println!("p1: {output}");
}

fn part2(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut splitters: HashMap<(usize, usize), u64> = HashMap::new();
    let mut beams: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut beam_timelines: HashMap<(usize, usize), u64> = HashMap::new();
    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();

        for x in 0..line.len() {
            if line[x] == 'S' {
                beams.insert(y, vec![x]);
                beam_timelines.insert((x, y), 1);
            } else if line[x] == '^' {
                splitters.insert((x, y), 0);
            }
        }
    }

    let mut output = 0;
    let mut shined: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..lines.len() {
        beams.insert(y + 1, vec![]);
        let beams_line = beams[&y].clone();
        for x in beams_line.iter() {
            let current_beam = (*x, y);
            if shined.contains(&current_beam) {
                continue;
            }
            shined.insert(current_beam);

            match splitters.get(&(current_beam.0, current_beam.1 + 1)) {
                Some(_) => {
                    if current_beam.0 < lines[0].len() {
                        beams.get_mut(&(y + 1)).unwrap().push(current_beam.0 + 1);
                        *beam_timelines.entry((*x + 1, y + 1)).or_insert(0) +=
                            *beam_timelines.get(&(*x, y)).unwrap();
                    }
                    if current_beam.0 > 0 {
                        beams.get_mut(&(y + 1)).unwrap().push(current_beam.0 - 1);
                        *beam_timelines.entry((*x - 1, y + 1)).or_insert(0) +=
                            *beam_timelines.get(&(*x, y)).unwrap();
                    }
                }
                _ => {
                    beams.get_mut(&(y + 1)).unwrap().push(current_beam.0);
                    *beam_timelines.entry((*x, y + 1)).or_insert(0) +=
                        *beam_timelines.get(&(*x, y)).unwrap();
                }
            }
        }
    }

    for x in 0..lines[0].len() {
        if let Some(v) = beam_timelines.get(&(x, lines.len())) {
            output += v;
        }
    }

    println!("p2: {output}");
}

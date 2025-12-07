use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("./inputs/day05/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut output: u64 = 0;
    let blocs: Vec<&str> = input.split("\n\n").collect();

    let mut fresh_ranges: HashSet<(i64, i64)> = HashSet::new();

    for line in blocs[0].split('\n').collect::<Vec<&str>>() {
        let range: Vec<&str> = line.split('-').collect();
        fresh_ranges.insert((range[0].parse().unwrap(), range[1].parse().unwrap()));
    }

    for line in blocs[1].split('\n').collect::<Vec<&str>>() {
        let id: i64 = line.parse().unwrap();

        for range in fresh_ranges.iter() {
            if range.0 <= id && id <= range.1 {
                output += 1;
                break;
            }
        }
    }

    println!("p1: {output}");
}

fn part2(input: &String) {
    let mut output: i64 = 0;
    let blocs: Vec<&str> = input.split("\n\n").collect();
    let mut ranges: Vec<Vec<&str>> = vec![];

    let mut fresh_ranges: HashSet<(i64, i64)> = HashSet::new();

    for line in blocs[0].split('\n').collect::<Vec<&str>>() {
        let range: Vec<&str> = line.split('-').collect();
        ranges.push(range);
    }
    ranges.sort();

    for range in ranges.iter() {
        let mut low_bound: i64 = range[0].parse().unwrap();
        let mut high_bound: i64 = range[1].parse().unwrap();
        let mut indexes_to_remove: Vec<(i64, i64)> = vec![];
        let mut skip = false;

        for range in fresh_ranges.iter() {
            if low_bound < range.0 && range.0 <= high_bound && high_bound <= range.1 {
                high_bound = range.1;
                indexes_to_remove.push(*range);
            }
            if range.0 <= low_bound && low_bound <= range.1 && range.1 < high_bound {
                low_bound = range.0;
                indexes_to_remove.push(*range);
            }
            if range.0 <= low_bound
                && low_bound <= range.1
                && range.0 <= high_bound
                && high_bound <= range.1
            {
                skip = true;
            }
        }

        if !skip {
            for range in indexes_to_remove {
                fresh_ranges.remove(&range);
            }
            fresh_ranges.insert((low_bound, high_bound));
        }
    }

    for range in fresh_ranges.iter() {
        output += 1 + range.1 - range.0;
    }
    println!("p2: {output}");
}

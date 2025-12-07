use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/day02/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn is_valid_id(nb: u64) -> bool {
    match nb.checked_ilog10().unwrap_or(0) + 1 {
        1 => false,
        2 => nb.is_multiple_of(11),
        3 => nb.is_multiple_of(111),
        4 => nb.is_multiple_of(101),
        5 => nb.is_multiple_of(11111),
        6 => nb.is_multiple_of(1001),
        7 => nb.is_multiple_of(1111111),
        8 => nb.is_multiple_of(1010101) || nb.is_multiple_of(10001),
        9 => nb.is_multiple_of(1001001),
        10 => nb.is_multiple_of(100001),
        _ => panic!(),
    }
}

fn part1(input: &String) {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut count = 0;

    for range in ranges.iter().map(|r| {
        r.split('-')
            .map(|nb| nb.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }) {
        let min = range[0];
        for nb in min..(range[1] + 1) {
            if is_valid_id(nb) && nb.to_string().len() % 2 == 0 {
                count += nb;
            }
        }
    }
    println!("p1: {count}");
}

fn is_valid_id_rep(nb: u64) -> bool {
    match nb.checked_ilog10().unwrap_or(0) + 1 {
        1 => false,
        2 => nb.is_multiple_of(11),
        3 => nb.is_multiple_of(111),
        4 => nb.is_multiple_of(101),
        5 => nb.is_multiple_of(11111),
        6 => nb.is_multiple_of(1001) || nb.is_multiple_of(10101),
        7 => nb.is_multiple_of(1111111),
        8 => nb.is_multiple_of(1010101) || nb.is_multiple_of(10001),
        9 => nb.is_multiple_of(1001001),
        10 => nb.is_multiple_of(100001) || nb.is_multiple_of(101010101),
        _ => panic!(),
    }
}

fn part2(input: &String) {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut count = 0;

    for range in ranges.iter().map(|r| {
        r.split('-')
            .map(|nb| nb.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }) {
        let min = range[0];
        for nb in min..(range[1] + 1) {
            if is_valid_id_rep(nb) {
                count += nb;
            }
        }
    }
    println!("p1: {count}");
}

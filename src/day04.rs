use std::{collections::HashSet, fs};

pub fn run() {
    let input = fs::read_to_string("./inputs/day04/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut output: u64 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let mut rolls: HashSet<(usize, usize)> = HashSet::new();
    let mut rolls_coors: Vec<(usize, usize)> = vec![];
    for y in 0..lines.len() {
        let line_chars = lines[y].chars().collect::<Vec<char>>();
        for x in 0..line_chars.len() {
            if line_chars[x] == '@' {
                rolls.insert((x, y));
                rolls_coors.push((x, y));
            }
        }
    }
    for coors in rolls_coors.iter() {
        let mut count = 0;
        for x in -1..2 {
            for y in -1..2 {
                if x == 0 && y == 0 {
                    continue;
                }
                if rolls.contains(&((coors.0 as i32 + x) as usize, (coors.1 as i32 + y) as usize)) {
                    count += 1;
                }
            }
        }
        output = if count <= 3 { output + 1 } else { output };
    }
    println!("p1: {output}");
}

fn part2(input: &String) {
    let mut output: u64 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    let mut rolls: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..lines.len() {
        let line_chars = lines[y].chars().collect::<Vec<char>>();
        for x in 0..line_chars.len() {
            if line_chars[x] == '@' {
                rolls.insert((x, y));
            }
        }
    }
    let mut to_be_removed: Vec<(usize, usize)> = vec![];
    loop {
        let rolls_coors: Vec<&(usize, usize)> = rolls.iter().collect();
        for coors in rolls_coors.iter() {
            let mut count = 0;
            for x in -1..2 {
                for y in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if rolls
                        .contains(&((coors.0 as i32 + x) as usize, (coors.1 as i32 + y) as usize))
                    {
                        count += 1;
                    }
                }
            }
            if count <= 3 {
                to_be_removed.push(**coors);
            }
        }
        output += to_be_removed.len() as u64;
        if to_be_removed.len() == 0 {
            break;
        }
        for coors in to_be_removed.iter() {
            rolls.remove(coors);
        }
        to_be_removed.clear();
    }
    println!("p2: {output}");
}

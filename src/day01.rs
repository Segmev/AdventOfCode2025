use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/day01/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let mut pos = 50;
    let max = 100;

    let lines: Vec<&str> = input.split('\n').collect();
    let mut zeros = 0;

    for command in lines.iter() {
        let command_chars: Vec<char> = command.chars().collect();
        let mut steps = 0;
        for command_char in command_chars.iter() {
            if *command_char == 'L' || *command_char == 'R' {
                continue;
            }
            steps = steps * 10;
            steps += command_char.to_digit(10).unwrap() as i32;
        }
        if command_chars[0] == 'R' {
            pos = (pos + steps) % max;
        } else {
            pos = ((pos - steps) % max + max) % max;
        }
        if pos == 0 {
            zeros += 1;
        }
    }
    println!("p1: {}", zeros);
}

fn part2(input: &String) {
    let mut pos = 50;
    let max = 100;

    let lines: Vec<&str> = input.split('\n').collect();
    let mut zeros = 0;

    for command in lines.iter() {
        let command_chars: Vec<char> = command.chars().collect();
        let mut steps = 0;
        for command_char in command_chars.iter() {
            if *command_char == 'L' || *command_char == 'R' {
                continue;
            }
            steps = steps * 10;
            steps += command_char.to_digit(10).unwrap() as i32;
        }
        if command_chars[0] == 'R' {
            zeros += (pos + steps) / max;
            pos = (pos + steps) % max;
        } else {
            if pos != 0 {
                zeros += ((max - pos) + steps) / max;
            } else {
                zeros += ((pos) + steps) / max;
            }
            pos = ((pos - steps) % max + max) % max;
        }
    }
    println!("p2: {}", zeros);
}

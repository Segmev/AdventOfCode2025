use regex::Regex;
use std::{fs, vec};

pub fn run() {
    let input = fs::read_to_string("./inputs/day06/input.txt")
        .expect("Should have been able to read the file");
    part1(&input);
    part2(&input);
}

fn part1(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let re = Regex::new(r" +").unwrap();
    let operators: Vec<&str> = re.split(lines[lines.len() - 1].trim()).collect();

    let mut results: Vec<i64> = vec![0; operators.len()];
    for l_i in 0..lines.len() - 1 {
        let operands: Vec<i64> = re
            .split(lines[l_i].trim())
            .map(|nb| nb.parse::<i64>().unwrap())
            .collect();

        for i in 0..operands.len() {
            match operators[i] {
                "+" => {
                    results[i] += operands[i];
                }
                "*" => {
                    if l_i == 0 {
                        results[i] = 1;
                    }
                    results[i] *= operands[i];
                }
                _ => panic!(),
            }
        }
    }

    let mut output = 0;
    for nb in results.iter() {
        output += *nb;
    }
    println!("p1: {output}");
}

fn part2(input: &String) {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut symbols: Vec<Vec<u8>> = vec![];
    for l_i in 0..lines.len() {
        symbols.push(lines[l_i].bytes().collect::<Vec<u8>>());
    }

    let mut output = 0;

    let mut operands = vec![];
    for i in (0..lines[0].len()).rev() {
        let mut operand: i64 = 0;
        for l_i in 0..lines.len() - 1 {
            if symbols[l_i][i] != b' ' {
                operand *= 10;
                operand += (symbols[l_i][i] - b'0') as i64;
            }
        }
        if operand == 0 {
            continue;
        }
        if symbols[lines.len() - 1][i] != b' ' {
            let op = symbols[lines.len() - 1][i] as char;
            if op == '*' {
                let mut res = operand;
                for operand in operands.iter() {
                    res *= operand;
                }
                output += res;
            } else if op == '+' {
                let mut res = operand;
                for operand in operands.iter() {
                    res += operand;
                }
                output += res;
            }
            operands.clear();
        } else {
            operands.push(operand);
        }
    }

    println!("p2: {output}");
}

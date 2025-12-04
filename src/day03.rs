use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/day03/input.txt")
        .expect("Should have been able to read the file");
    part(&input, 2);
    part(&input, 12);
}

fn part(input: &String, bat_count: usize) {
    let mut output: u64 = 0;
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines.iter() {
        let line_chars = line.chars().collect::<Vec<char>>();
        let mut indexes: Vec<usize> = ((line_chars.len() - bat_count)..line_chars.len()).collect();

        let mut bat_idx = 0;
        while bat_idx < bat_count {
            let mut idx: i8 = (indexes[bat_idx]) as i8;
            let up_bound = if bat_idx == 0 {
                0
            } else {
                indexes[bat_idx - 1] + 1
            };
            while idx >= up_bound as i8 {
                if line_chars[idx as usize] >= line_chars[indexes[bat_idx]] {
                    indexes[bat_idx] = idx as usize;
                }
                idx -= 1;
            }

            bat_idx += 1;
        }

        let mut nb = 0;
        for i in indexes.iter() {
            nb += (line_chars[*i] as u8 - b'0') as u64;
            nb *= 10;
        }
        output += nb / 10;
    }
    println!("p2: {output}");
}

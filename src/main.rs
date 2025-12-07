use std::time::Instant;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let days: Vec<(&str, fn())> = vec![
        ("01", day01::run),
        ("02", day02::run),
        ("03", day03::run),
        ("04", day04::run),
        ("05", day05::run),
        ("06", day06::run),
    ];

    for (day, func) in days.iter() {
        println!("day {}\n=============", day);
        let start = Instant::now();
        func();
        let duration = start.elapsed();
        println!("============= ({:?})", duration);
    }
}

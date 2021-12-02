use crate::benchmark;
use crate::common::read_lines;
use std::io::{self};

pub fn run() {
    println!("=== adv1 === ");

    let (r1, rb1) = benchmark!(run_1());
    let (r2, rb2) = benchmark!(run_2());

    println!("Result1: {}", r1);
    println!("-> Took: {:?}", rb1);

    println!("Result2: {}", r2);
    println!("-> Took: {:?}", rb2);
}

fn run_1() -> i32 {
    let mut increased = 0;

    if let Ok(input) = read_lines("./src/data/input.txt") {
        let lines: Vec<i32> = input
            .filter_map(io::Result::ok)
            .map(|x| {
                return match x.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };
            })
            .collect();

        increased = lines
            .windows(2)
            .map(|x| x[1] - x[0])
            .filter(|x| x > &0)
            .count();
    }

    increased as i32
}

fn run_2() -> i32 {
    let mut increased = 0;

    if let Ok(input) = read_lines("./src/data/input.txt") {
        let lines: Vec<i32> = input
            .filter_map(io::Result::ok)
            .map(|x| {
                return match x.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };
            })
            .collect();

        let c: Vec<i32> = lines.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
        increased = c.windows(2).map(|x| x[1] - x[0]).filter(|x| x > &0).count();
    }

    increased as i32
}

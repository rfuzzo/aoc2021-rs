use crate::common::read_lines;
use std::io::{self};

pub fn run() {
    println!("=== adv1 === ");
    println!("Result1: {}", run_1());
    println!("Result2: {}", run_2());
}

fn run_1() -> i32 {
    let mut increased = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        let a1: Vec<i32> = lines
            .filter_map(io::Result::ok)
            .map(|x| {
                return match x.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };
            })
            .collect();

        increased = a1
            .windows(2)
            .map(|x| x[1] - x[0])
            .filter(|x| x > &0)
            .count();
    }

    return increased as i32;
}

fn run_2() -> i32 {
    let mut increased = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        let a1: Vec<i32> = lines
            .filter_map(io::Result::ok)
            .map(|x| {
                return match x.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };
            })
            .collect();

        let c: Vec<i32> = a1.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
        increased = c.windows(2).map(|x| x[1] - x[0]).filter(|x| x > &0).count();
    }

    return increased as i32;
}

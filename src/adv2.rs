use crate::common::read_lines;
use std::io::{self};

pub fn run() {
    println!("=== adv2 === ");
    println!("Result1: {}", run_1());
    println!("Result2: {}", run_2());
}

fn run_1() -> i32 {
    if let Ok(input) = read_lines("./src/data/input2.txt") {
        let lines = input.filter_map(io::Result::ok);

        let (a, b) = lines.fold((0 as i32, 0 as i32), |(accx, accy), l| {
            let splits = l.split(" ").collect::<Vec<&str>>();
            let id = splits[0];
            let val = splits[1];

            if id == "up" {
                let up = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                return (accx, accy - up);
            } else if id == "down" {
                let down = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                return (accx, accy + down);
            } else if id == "forward" {
                let fwd = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                return (accx + fwd, accy);
            } else {
                panic!("Incorrect input")
            };
        });

        return a * b;
    } else {
        panic!("Incorrect input")
    }
}

fn run_2() -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    if let Ok(input) = read_lines("./src/data/input2.txt") {
        let lines = input.filter_map(io::Result::ok);

        for line in lines {
            let splits = line.split(" ").collect::<Vec<&str>>();
            let id = splits[0];
            let val = splits[1];

            if id == "up" {
                let up = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                aim -= up;
            } else if id == "down" {
                let down = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                aim += down;
            } else if id == "forward" {
                let fwd = match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                x += fwd;
                y += aim * fwd;
            } else {
                panic!("Incorrect input")
            };

            //println!("{}{} -> [a: {}, x: {}, y: {}]", id.as_bytes()[0] as char, val, aim, x ,y)
        }
    }

    return x * y;
}

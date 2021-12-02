use crate::benchmark;
use crate::common::read_lines;
use std::io::{self};

pub fn run() {
    println!("=== adv2 === ");

    let (r1, rb1) = benchmark!(run_1());
    let (r2, rb2) = benchmark!(run_2());
    let (r2x, rb2x) = benchmark!(run_2x());

    println!("Result1: {}", r1);
    println!("-> Took: {:?}", rb1);

    println!("Result2: {}", r2);
    println!("-> Took: {:?}", rb2);

    println!("Result2x: {}", r2x);
    println!("-> Took: {:?}", rb2x);
}

fn run_1() -> i32 {
    if let Ok(input) = read_lines("./src/data/input2.txt") {
        let lines = input.filter_map(io::Result::ok);

        let (x, y) = lines.fold((0 as i32, 0 as i32), |(accx, accy), l| {
            let splits = l.split(" ").collect::<Vec<&str>>();
            let id = splits[0];
            let val = splits[1];

            match id {
                "up" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(up) => (accx, accy - up),
                },
                "down" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(down) => (accx, accy + down),
                },
                "forward" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(fwd) => (accx + fwd, accy),
                },
                _ => panic!("Incorrect input"),
            }
        });

        return x * y;
    } else {
        panic!("Incorrect input")
    }
}

fn run_2x() -> i32 {
    if let Ok(input) = read_lines("./src/data/input2.txt") {
        let lines = input.filter_map(io::Result::ok);

        let (_, x, y) = lines.fold((0 as i32, 0 as i32, 0 as i32), |(acca, accx, accy), l| {
            let splits = l.split(" ").collect::<Vec<&str>>();
            let id = splits[0];
            let val = splits[1];

            match id {
                "up" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(up) => (acca - up, accx, accy),
                },
                "down" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(down) => (acca + down, accx, accy),
                },
                "forward" => match val.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(fwd) => (acca, accx + fwd, accy + (acca * fwd)),
                },
                _ => panic!("Incorrect input"),
            }
        });

        return x * y;
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

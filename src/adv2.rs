use std::fs;
use std::io::{self, BufRead};
use std::path;

pub fn run() {
    println!("=== adv2 === ");
    println!("Result1: {}", run_1());
    println!("Result2: {}", run_2());
}

fn run_1() -> i32 {
    let mut x = 0;
    let mut y = 0;

    if let Ok(lines) = read_lines("./src/input2.txt") {
        let a1 = lines.filter_map(io::Result::ok);

        for line in a1 {
            let splits = line.split(" ").collect::<Vec<&str>>();

            if splits[0] == "up" {
                let up = match splits[1].parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                y -= up;
            } else if splits[0] == "down" {
                let down = match splits[1].parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                y += down;
            } else if splits[0] == "forward" {
                let fwd = match splits[1].parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                x += fwd;
            } else {
                panic!("Incorrect input")
            };
        }
    }

    return x * y;
}

fn run_2() -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut y = 0;

    if let Ok(lines) = read_lines("./src/input2.txt") {
        let a1 = lines.filter_map(io::Result::ok);

        for line in a1 {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

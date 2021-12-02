use std::fs;
use std::io::{self, BufRead};
use std::path;

pub fn run() {
    run_1();
    run_2();
}

fn run_1() {
    println!("=== adv2_run_1 === ");
}

fn run_2() {
    println!("=== adv2_run_2 === ");
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

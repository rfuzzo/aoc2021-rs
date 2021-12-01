use std::fs;
use std::io::{self, BufRead};
use std::path;

pub fn adv1_run() {
    adv1_run_1();
    adv1_run_2();
}

fn adv1_run_1() {
    println!("=== adv1_run_1 === ");

    let _path = path::Path::new("./src/input.txt");

    if let Ok(lines) = read_lines(_path) {
        let mut old_depth = 0;
        let mut increased = 0;
        let mut decreased = 0;
        let mut equal = 0;
        let mut total = 0;

        for line in lines {
            if let Ok(depth_str) = line {
                //println!("{}", depth_str);
                let depth = match depth_str.parse::<i32>() {
                    Err(e) => panic!("Incorrect input: {:?}", e),
                    Ok(n) => n,
                };

                if total == 0 {
                    total += 1;
                    continue;
                }

                if depth > old_depth {
                    increased = increased + 1;
                } else if depth < old_depth {
                    decreased = decreased + 1;
                } else {
                    equal += 1;
                }
                old_depth = depth;
                total += 1;
            }
        }

        println!("increased: {}", increased);
        println!("decreased: {}", decreased);
        println!("equal: {}", equal);
        println!("total: {}", total);
    }
}

fn adv1_run_2() {
    println!("=== adv1_run_2 === ");

    let _path = path::Path::new("./src/input.txt");

    if let Ok(lines) = read_lines(_path) {
        let mut old_depth = 0;
        let mut increased = 0;
        let mut decreased = 0;
        let mut equal = 0;
        let mut total = 0;

        let x: Vec<String> = lines.filter_map(io::Result::ok).collect();
        let w = x.windows(3);

        for line in w {
            let a = match line[0].parse::<i32>() {
                Err(e) => panic!("Incorrect input: {:?}", e),
                Ok(n) => n,
            };
            let b = match line[1].parse::<i32>() {
                Err(e) => panic!("Incorrect input: {:?}", e),
                Ok(n) => n,
            };
            let c = match line[2].parse::<i32>() {
                Err(e) => panic!("Incorrect input: {:?}", e),
                Ok(n) => n,
            };

            let depth = a + b + c;
            //println!("[{},{},{}] > {}", a, b, c, depth);

            if total == 0 {
                total += 1;
                continue;
            }

            if depth > old_depth {
                increased = increased + 1;
            } else if depth < old_depth {
                decreased = decreased + 1;
            } else {
                equal += 1;
            }
            old_depth = depth;
            total += 1;
        }

        println!("increased: {}", increased);
        println!("decreased: {}", decreased);
        println!("equal: {}", equal);
        println!("total: {}", total);
    }
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

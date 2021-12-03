use crate::benchmark;
use crate::common::read_as_vec;

pub fn run() {
    println!("=== adv2 === ");

    let lines = read_as_vec("./src/data/input2.txt");

    let (r1, rb1) = benchmark!(run_1(&lines));
    let (r2, rb2) = benchmark!(run_2(&lines));

    println!("Result1: {}", r1);
    println!("-> Took: {:?}", rb1);

    println!("Result2: {}", r2);
    println!("-> Took: {:?}", rb2);
}

pub fn run_1(lines: &[String]) -> i32 {
    let (x, y) = lines.iter().fold((0, 0), |(accx, accy), l| {
        let id = l.as_bytes().get(0).unwrap();
        match id {
            0x75 => (accx, accy - l[3..].parse::<i32>().expect("Incorrect input")),
            0x64 => (accx, accy + l[5..].parse::<i32>().expect("Incorrect input")),
            0x66 => (accx + l[8..].parse::<i32>().expect("Incorrect input"), accy),
            _ => panic!("Incorrect input"),
        }
    });

    x * y
}

pub fn run_2(lines: &[String]) -> i32 {
    let (_, x, y) = lines
        .iter()
        .fold((0_i32, 0_i32, 0_i32), |(acca, accx, accy), l| {
            let id = l.as_bytes().get(0).unwrap();
            match id {
                0x75 => (
                    acca - l[3..].parse::<i32>().expect("Incorrect input"),
                    accx,
                    accy,
                ),
                0x64 => (
                    acca + l[5..].parse::<i32>().expect("Incorrect input"),
                    accx,
                    accy,
                ),
                0x66 => {
                    let fwd = l[8..].parse::<i32>().expect("Incorrect input");
                    (acca, accx + fwd, accy + (acca * fwd))
                }
                _ => panic!("Incorrect input"),
            }
        });

    x * y
}

pub fn run_1x(lines: &[String]) -> u64 {
    let (x, y) = lines.iter().fold::<(u64, u64), _>((0, 0), |(hor, dep), l| {
        let mut chars = l.chars();
        let (dir, units) = (
            chars.next().unwrap(),
            chars.nth_back(0).unwrap() as u8 - b'0',
        );
        let units = units as u64;
        match dir {
            'f' => (hor + units, dep),
            'u' => (hor, dep - units),
            'd' => (hor, dep + units),
            _ => panic!("Incorrect input"),
        }
    });

    x * y
}

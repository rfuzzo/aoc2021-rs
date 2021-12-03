use crate::benchmark;
use crate::common::get_file_as_byte_vec;
use std::io::Read;

pub fn run() {
    println!("=== adv2 === ");

    let (buf, len) = get_file_as_byte_vec("./src/data/input3.txt");

    let (r1, rb1) = benchmark!(run_1(&buf, len));
    //let (r2, rb2) = benchmark!(run_2(&lines));

    println!("Result1: {}", r1);
    println!("-> Took: {:?}", rb1);

    //println!("Result2: {}", r2);
    //println!("-> Took: {:?}", rb2);
}

fn run_1(buffer: &[u8], len: usize) -> isize {
    let mut r: [u32; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let slices_count = len as u32 / 13;
    let mut b = buffer;

    for _ in 0..slices_count {
        let mut buffer2 = [0; 13];
        b.read(&mut buffer2[..]).expect("error");
        for j in 0..12 {
            r[j] += (buffer2[j] - 0x30) as u32;
        }
    }

    let gamma_bin = r
        .map(|x| if x < slices_count / 2 { "0" } else { "1" })
        .join("");
    let gamma = isize::from_str_radix(&gamma_bin, 2).unwrap();
    let epsilon_bin = r
        .map(|x| if x >= slices_count / 2 { "0" } else { "1" })
        .join("");
    let epsilon = isize::from_str_radix(&epsilon_bin, 2).unwrap();

    println!("{} - {}", gamma_bin, gamma);
    println!("{} - {}", epsilon_bin, epsilon);

    gamma * epsilon
}

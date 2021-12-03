// #![feature(test)]
// extern crate test;

#[allow(dead_code)]
#[allow(soft_unstable)]
//mod adv1;
//mod adv2;
mod adv3;
mod common;

fn main() {
    //adv1::run();
    //adv2::run();
    adv3::run();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test::Bencher;

//     #[bench]
//     fn bench_adv3_part1(b: &mut Bencher) {
//         let (buf, len) = common::get_file_as_byte_vec("./src/data/input3.txt");
//         b.iter(|| adv3::run_1(&buf, len));
//     }

//     #[bench]
//     fn bench_adv3_part2(b: &mut Bencher) {
//         let (buf, len) = common::get_file_as_byte_vec("./src/data/input3.txt");
//         b.iter(|| adv3::run_2(&buf, len));
//     }
// }

#![feature(test)]
extern crate test;

#[allow(dead_code)]
#[allow(soft_unstable)]
mod adv1;
mod adv2;
mod common;

fn main() {
    //adv1::run();
    adv2::run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_adv2_part1(b: &mut Bencher) {
        let lines = common::read_as_vec("./src/data/input2.txt");
        b.iter(|| adv2::run_1(&lines));
    }

    #[bench]
    fn bench_adv2_part2(b: &mut Bencher) {
        let lines = common::read_as_vec("./src/data/input2.txt");
        b.iter(|| adv2::run_2(&lines));
    }

    #[bench]
    fn bench_adv1_part1x(b: &mut Bencher) {
        let lines = common::read_as_vec("./src/data/input2.txt");
        b.iter(|| adv2::run_1x(&lines));
    }
}

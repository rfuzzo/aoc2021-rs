pub use common::read_lines;

mod common {
    use std::fs;
    use std::io::{self, BufRead};
    use std::path;

    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where
        P: AsRef<path::Path>,
    {
        let file = fs::File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

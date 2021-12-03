use std::fs;
use std::io::Read;
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

pub fn read_as_vec<P>(filename: P) -> Vec<String>
where
    P: AsRef<path::Path>,
{
    read_lines(filename)
        .expect("Incorrect input")
        .filter_map(io::Result::ok)
        .collect()
}

pub fn get_file_as_byte_vec<P>(filename: P) -> (std::vec::Vec<u8>, usize)
where
    P: AsRef<path::Path>,
{
    let mut f = fs::File::open(filename).expect("Incorrect input");
    let mut buffer = Vec::new();

    // read the whole file
    let total = f.read_to_end(&mut buffer).expect("Incorrect input");
    (buffer, total)
}

#[macro_export]
macro_rules! benchmark {
    ($code:expr) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let end = std::time::Instant::now();

        (result, end - start)
    }};
}

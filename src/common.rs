pub use common::read_lines;
pub use common::read_as_vec;

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

    pub fn read_as_vec<P>(filename: P) -> Vec<String>
    where
        P: AsRef<path::Path>,
    {
        let lines: Vec<String>;
        match read_lines(filename) {
            Err(e) => panic!("Incorrect input {}", e),
            Ok(input) => {
                lines = input.filter_map(io::Result::ok).collect();
            }
        }
        lines
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
}

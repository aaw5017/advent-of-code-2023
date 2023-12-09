use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub fn get_file_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Can't read input file. Something ain't right.");
    return BufReader::new(file).lines();
}

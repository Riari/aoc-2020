use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

pub fn file_to_vec<T>(filename: impl AsRef<Path>) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<T>().unwrap())
        .collect()
}
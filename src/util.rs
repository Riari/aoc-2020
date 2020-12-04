use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    str::FromStr,
};

fn open_file(filename: impl AsRef<Path>) -> File {
    return File::open(filename).expect("No such file");
}

pub fn file_to_vec<T>(filename: impl AsRef<Path>) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let file = open_file(filename);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<T>().unwrap())
        .collect()
}

pub fn file_to_string(filename: impl AsRef<Path>) -> String {
    let mut file = open_file(filename);
    let mut s = String::new();
    file.read_to_string(&mut s).expect("Unable to read file");
    return s;
}
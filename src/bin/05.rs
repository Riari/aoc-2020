use std::collections::BTreeSet;
use std::iter::FromIterator;
use util;

fn address_to_id(address: &String) -> u16 {
    let bin = address.chars().map(|c| if c == 'B' || c == 'R' { '1' } else { '0' }).collect::<String>();
    u16::from_str_radix(&bin, 2).unwrap()
}

fn main() {
    let seats: Vec<u16> = util::file_to_vec("input/05").iter().map(address_to_id).collect();
    let min = seats.iter().min().unwrap();
    let max = seats.iter().max().unwrap();

    let a: BTreeSet<_> = BTreeSet::from_iter((*min..=*max).into_iter());
    let b: BTreeSet<_> = seats.iter().map(|s| *s).collect();
    let missing = a.difference(&b).next().unwrap();

    println!("Part 1: {}", max);
    println!("Part 2: {}", missing);
}
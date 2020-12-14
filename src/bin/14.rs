#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::iter::FromIterator;
use regex::Regex;
use util;

enum Op {
    Mask(Vec<char>),
    Mem(usize, String),
}

fn input() -> Vec<Op> {
    lazy_static! {
        static ref NUMBER_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let input: String = util::file_to_string("input/14");
    input.lines()
        .map(|line| {
            let mut pair = line.split(" = ");
            let key = pair.next().unwrap();
            let value = pair.next().unwrap();
            if key == "mask" {
                Op::Mask(value.chars().collect())
            } else {
                let address = NUMBER_REGEX.captures(key).unwrap().get(0).unwrap();
                let mut bin = format!("{:b}", value.parse::<usize>().unwrap());
                bin = format!("{:0>36}", bin);
                Op::Mem(address.as_str().parse::<usize>().unwrap(), bin)
            }
        })
        .collect()
}

fn part1(input: &Vec<Op>) -> usize {
    let mut mask: Vec<char> = vec!['0';36];
    let mut memory: BTreeMap<usize, usize> = BTreeMap::new();

    for op in input.iter() {
        match op {
            Op::Mask(m) => mask = m.to_vec(),
            Op::Mem(address, value) => {
                let mut chars: Vec<char> = vec![];
                for (i, c) in mask.iter().enumerate() {
                    if *c == 'X' {
                        chars.push(value.chars().nth(i).unwrap());
                    } else {
                        chars.push(*c);
                    }
                }

                let dec = usize::from_str_radix(&String::from_iter(chars), 2).unwrap();
                memory.insert(*address, dec);
            }
        }
    }

    memory.iter().map(|v| v.1).sum()
}

fn apply_mask(mask: &Vec<char>, value: &Vec<char>) -> Vec<String> {
    let mut addresses: Vec<String> = vec!["".to_string()];

    for i in 0..mask.len() {
        match mask[i] {
            '0' => {
                for j in 0..addresses.len() {
                    addresses[j].push(value[i]);
                }
            },
            '1' => {
                for j in 0..addresses.len() {
                    addresses[j].push(mask[i]);
                }
            },
            'X' => {
                let mut new: Vec<String> = vec![];
                for j in 0..addresses.len() {
                    new.push(addresses[j].clone() + "1");
                    new.push(addresses[j].clone() + "0");
                }
                addresses = new;
            },
            _ => unreachable!()
        }
    }
    
    addresses
}

fn part2(input: &Vec<Op>) -> usize {
    let mut mask: Vec<char> = vec!['0';36];
    let mut memory: BTreeMap<usize, usize> = BTreeMap::new();

    for op in input.iter() {
        match op {
            Op::Mask(m) => mask = m.to_vec(),
            Op::Mem(address, value) => {
                let bin_addr: Vec<char> = format!("{:0>36}", format!("{:b}", address)).chars().collect();
                let addresses: Vec<String> = apply_mask(&mask, &bin_addr);
                for address in addresses.iter() {
                    memory.insert(usize::from_str_radix(&address, 2).unwrap(), usize::from_str_radix(&value, 2).unwrap());
                }
            }
        }
    }

    memory.iter().map(|v| v.1).sum()
}

fn main() {
    let input = input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
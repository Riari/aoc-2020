#![feature(map_first_last)]
use std::collections::BTreeMap;
use util;

fn part1(depart_after: usize, buses: &Vec<usize>) -> usize {
    let mut wait_times: BTreeMap<usize, usize> = BTreeMap::new();
    for bus in buses {
        if *bus == 1 as usize {
            continue;
        }

        let mut wait_time = *bus;
        while wait_time < depart_after {
            wait_time += bus;
        }
        wait_times.insert(wait_time - depart_after, *bus);
    }

    let result = wait_times.first_key_value().unwrap();

    result.0 * result.1
}

// Based on https://github.com/sandouli/advent-of-code/blob/master/2020/day_13/src/main.rs
fn part2(buses: &Vec<usize>) -> usize {
    let coprime: usize = buses.iter().product();

    buses.iter().rev()
        .enumerate()
        .map(|(i, b)| {
            let current_factor = coprime / b;
            let mut j = 1;
            while (current_factor * j) % b != i % b {
                j += 1;
            }
            current_factor * j
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>() % coprime - buses.len() + 1
}

fn main() {
    let input = util::file_to_string("input/13");
    let mut input_split = input.split_whitespace();
    let depart_after = input_split.next().unwrap().parse::<usize>().unwrap();
    let buses: Vec<usize> = input_split.next().unwrap()
        .split(',')
        .map(|b| if b == "x" { 1 } else { b.parse::<usize>().unwrap() })
        .collect();
    
    println!("Part 1: {}", part1(depart_after, &buses));
    println!("Part 2: {}", part2(&buses));
}
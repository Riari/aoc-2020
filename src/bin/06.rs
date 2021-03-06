use std::collections::BTreeSet;
use std::iter::FromIterator;
use util;

fn groups() -> Vec<String> {
    util::file_to_string("input/06").split("\n\n").map(String::from).collect()
}

fn part1(groups: &Vec<String>) -> usize {
    return groups.iter()
        .map(|g| {
            g.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<BTreeSet<_>>()
                .len()
        })
        .sum()
}

fn part2(groups: &Vec<String>) -> usize {
    return groups.iter()
        .map(|g| g.split("\n").map(|p| BTreeSet::from_iter(p.chars())).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .iter()
        .map(|g| {
            let first = g.first().unwrap().clone();
            g.iter().skip(1)
                .fold(first, |a, b| a.intersection(&b).cloned().collect())
                .len()
        })
        .sum()
}

fn main() {
    let groups = groups();

    println!("Part 1: {}", part1(&groups));
    println!("Part 2: {}", part2(&groups));
}
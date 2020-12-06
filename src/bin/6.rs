use std::collections::BTreeSet;
use util;

fn answers() -> Vec<String> {
    util::file_to_string("input/6").split("\n\n").map(String::from).collect::<Vec<String>>()
}

fn part1(groups: &Vec<String>) -> usize {
    return groups.iter()
        .map(|g| {
            g.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<BTreeSet<char>>()
                .len()
        })
        .sum()
}

fn part2(groups: &Vec<String>) -> usize {
    return groups.iter()
        .map(|g| g.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>()
        .iter()
        .map(|g| {
            let first = g.first().unwrap().chars().collect::<BTreeSet<_>>();
            g.iter().skip(1)
                .fold(first, |a, b| a.intersection(&b.chars().collect()).cloned().collect())
                .len()
        })
        .sum()
}

fn main() {
    let answers = answers();

    println!("Part 1: {}", part1(&answers));
    println!("Part 2: {}", part2(&answers));
}
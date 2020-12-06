use std::collections::BTreeSet;
use util;

fn answers() -> Vec<Vec<BTreeSet<char>>> {
    return util::file_to_string("input/6")
        .split("\n\n")
        .map(|g| {
            g.split_whitespace()
                .map(|p| p.chars().collect::<BTreeSet<_>>())
                .collect()
        })
        .collect();
}

fn part1(groups: &Vec<Vec<BTreeSet<char>>>) -> usize {
    return groups.iter()
        .map(|g| {
            let first = g.first().unwrap().clone();
            g.iter()
                .skip(1)
                .fold(first, |a, b| a.union(&b).cloned().collect())
                .len()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn part2(groups: &Vec<Vec<BTreeSet<char>>>) -> usize {
    return groups.iter()
        .map(|g| {
            let first = g.first().unwrap().clone();
            g.iter()
                .skip(1)
                .fold(first, |a, b| a.intersection(&b).cloned().collect())
                .len()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}

fn main() {
    let answers = answers();

    println!("Part 1: {}", part1(&answers));
    println!("Part 2: {}", part2(&answers));
}
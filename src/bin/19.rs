use std::collections::HashMap;
use itertools::Itertools;

enum Rule {
    Comb(Vec<Vec<usize>>),
    Char(char),
}

struct Input {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

fn input() -> Input {
    let input = util::file_to_string("input/19");
    let mut sections = input.split("\n\n");

    let rules = sections.next().unwrap().lines()
        .flat_map(|s| s.split(": "))
        .tuples()
        .map(|(id, s)| {
            let id = id.parse::<usize>().unwrap();
            if s.contains('"') {
                return (id, Rule::Char(s.as_bytes()[1] as char));
            }
            let v = s.split(" | ")
                .map(|p| p.split_whitespace().map(|p| p.parse::<usize>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            (id, Rule::Comb(v))
        })
        .collect::<HashMap<_,_>>();

    let messages = sections.next().unwrap().lines().map(|m| m.to_string()).collect();

    Input { rules, messages }
}

// Borrowed from https://github.com/AxlLind/AdventOfCode2020/blob/master/src/bin/19.rs

fn matches(rules: &HashMap<usize, Rule>, id: usize) -> Vec<String> {
    match &rules[&id] {
        Rule::Char(c) => vec![c.to_string()],
        Rule::Comb(v) => {
            v.iter().flat_map(|p| match p[..] {
                [p] => matches(rules, p),
                [p1,p2] => matches(rules, p1).iter()
                    .cartesian_product(matches(rules, p2).iter())
                    .map(|(s1,s2)| format!("{}{}", s1, s2))
                    .collect(),
                _ => unreachable!()
            }).collect()
        }
    }
}

fn part1(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
    prefixes.iter()
        .filter(|&p| s.starts_with(p))
        .any(|p| {
                let s = &s[p.len()..];
                prefixes.iter()
                .cartesian_product(suffixes.iter())
                .filter(|(p1,p2)| s.len() == p1.len() + p2.len())
                .any(|(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
        })
}

fn part2(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
    prefixes.iter()
        .filter(|&p| s.starts_with(p))
        .map(|p| &s[p.len()..])
        .any(|s| part2(prefixes, suffixes, s) || check_rule_11(prefixes, suffixes, s))
}

fn check_rule_11(prefixes: &[String], suffixes: &[String], s: &str) -> bool {
    prefixes.iter()
        .cartesian_product(suffixes.iter())
        .filter(|&(p1,p2)| p1.len() + p2.len() <= s.len())
        .filter(|&(p1,p2)| s.starts_with(p1) && s.ends_with(p2))
        .map(|(p1,p2)| &s[p1.len()..(s.len() - p2.len())])
        .any(|s| s.len() == 0 || check_rule_11(prefixes, suffixes, s))
}

fn main() {
    let input = input();
    let v42 = matches(&input.rules, 42);
    let v31 = matches(&input.rules, 31);

    println!("Part 1: {}", input.messages.iter().filter(|s| part1(&v42, &v31, &s)).count());
    println!("Part 2: {}", input.messages.iter().filter(|s| part2(&v42, &v31, &s)).count());
}
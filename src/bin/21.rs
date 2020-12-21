use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use util;

fn input() -> Vec<(String, String)> {
    let raw: Vec<String> = util::file_to_vec("input/21");
    raw.iter()
        .map(|l| {
            let mut parts = l.split(" (contains ");
            let ingredients = parts.next().unwrap();
            let allergens = parts.next().unwrap().trim_matches(|c| c == ')');
            (ingredients.to_string(), allergens.to_string())
        })
        .collect::<Vec<(_, _)>>()
}

fn part1<'a>(list: &Vec<(&'a str, &'a str)>, allergen_map: &HashMap<&'a str, &'a str>) -> usize {
    let allergens = allergen_map.values().collect::<HashSet<_>>();
    list.iter()
        .flat_map(|(s, _)| s.split_whitespace())
        .filter(|i| !allergens.contains(i))
        .count()
}

fn part2<'a>(allergen_map: &HashMap<&'a str, &'a str>) -> String {
    allergen_map.iter()
        .sorted()
        .map(|(_, i)| i)
        .join(",")
}

fn main() {
    let input = input();
    let list: Vec<(&str, &str)> = input.iter().map(|v| (v.0.as_str(), v.1.as_str())).collect();

    let mut candidates = HashMap::new();
    for (ingredients, allergens) in list.clone() {
        let ingredients = ingredients.split_whitespace().collect::<HashSet<_>>();
        for a in allergens.split(", ") {
            let set = candidates.entry(a).or_insert(ingredients.clone());
            *set = &*set & &ingredients;
        }
    }

    let mut allergen_map = HashMap::new();
    while let Some((&a, _)) = candidates.iter().find(|(_, s)| s.len() == 1) {
        let &i = candidates[a].iter().next().unwrap();
        allergen_map.insert(a, i);
        for (_, s) in &mut candidates { s.remove(&i); }
    }

    println!("Part 1: {}", part1(&list, &allergen_map));
    println!("Part 2: {}", part2(&allergen_map));
}
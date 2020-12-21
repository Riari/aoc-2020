use std::collections::HashMap;
use regex::Regex;
use util;

const THE_BAG: &str = "shiny gold";

type ContentsMap = HashMap<String, usize>;
type BagMap = HashMap<String, ContentsMap>;

fn bags() -> BagMap {
    let input = util::file_to_string("input/07");
    let re = Regex::new(r"(?P<count>\d+) (?P<color>.+?) bags?").unwrap();

    let mut map: BagMap = HashMap::new();
    for line in input.lines() {
        match line.split(" bags contain ").collect::<Vec<_>>().as_slice() {
            &[source, targets] => {
                let v = map.entry(source.to_string()).or_insert_with(HashMap::new);

                for caps in re.captures_iter(targets) {
                    if caps.name("count").is_some() {
                        v.insert(
                            String::from(caps.name("color").unwrap().as_str()),
                            caps.name("count").unwrap().as_str().parse().unwrap(),
                        );
                    }
                }
            },
            _ => panic!("Invalid format"),
        }
    }

    map
}

fn bag_contains(bags: &BagMap, bag: &ContentsMap, color: &str) -> bool {
    for (k, _) in bag {
        if *k == color || bag_contains(bags, bags.get(k).unwrap(), color) {
            return true;
        }
    }

    false
}

fn count_bags_inside(bags: &BagMap, bag: &ContentsMap) -> usize {
    bag.iter()
        .map(|(k, v)| v * (1 + count_bags_inside(bags, bags.get(k).unwrap())))
        .sum()
}

fn part1(bags: &BagMap) -> usize {
    bags.iter()
        .map(|(_, v)| bag_contains(bags, v, THE_BAG) as usize)
        .sum()
}

fn part2(bags: &BagMap) -> usize {
    count_bags_inside(bags, bags.get(THE_BAG).unwrap())
}

fn main() {
    let bags = bags();

    println!("Part 1: {}", part1(&bags));
    println!("Part 2: {}", part2(&bags));
}
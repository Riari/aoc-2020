use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
use regex::Regex;
use util;

type Pos = [i8;2];

lazy_static! {
    static ref DIRECTIONS: HashMap<&'static str, Pos> = {
        let mut m = HashMap::new();
        m.insert("nw", [-1, 1]);
        m.insert("ne", [0, 1]);
        m.insert("e", [1, 0]);
        m.insert("se", [1, -1]);
        m.insert("sw", [0, -1]);
        m.insert("w", [-1, 0]);
        m
    };
}

fn add(a: Pos, b: Pos) -> Pos {
    let mut z: Pos = [0, 0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        z[i] = aval + bval;
    }
    z
}

fn black_tiles() -> HashSet<Pos> {
    lazy_static! {
        static ref DIR_REGEX: Regex = Regex::new(r"nw|ne|se|sw|e|w").unwrap();
    }

    let mut black_tiles: HashSet<Pos> = HashSet::new();
    for line in util::file_to_string("input/24").lines() {
        let mut pos: Pos = [0, 0];

        for caps in DIR_REGEX.captures_iter(line) {
            pos = add(pos, *DIRECTIONS.get(caps.get(0).unwrap().as_str()).unwrap());
        }

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }

    black_tiles
}

fn enumerate_neighbours(tile: &Pos, black_tiles: &HashSet<Pos>) -> HashMap<Pos, bool> {
    let mut neighbours: HashMap<Pos, bool> = HashMap::new();
    for pos in DIRECTIONS.values() {
        let neighbour = add(*tile, *pos);
        neighbours.insert(neighbour, black_tiles.contains(&neighbour));
    }

    neighbours
}

fn part2(black_tiles: &mut HashSet<Pos>) -> usize {
    for _ in 0..100 {
        let mut new: HashSet<Pos> = HashSet::new();
        for tile in black_tiles.iter() {
            let neighbours = enumerate_neighbours(&tile, &black_tiles);

            let mut black_neighbours = 0;
            for (neighbour, is_black) in neighbours.iter() {
                if *is_black {
                    black_neighbours += 1;
                } else {
                    if enumerate_neighbours(&neighbour, &black_tiles).iter().filter(|(_, &b)| b).count() == 2 {
                        new.insert(*neighbour);
                    }
                }
            }

            if (1..=2).contains(&black_neighbours) {
                new.insert(*tile);
            }
        }
        
        *black_tiles = new;
    }

    black_tiles.len()
}

fn main() {
    let mut black_tiles = black_tiles();

    println!("Part 1: {}", black_tiles.len());
    println!("Part 2: {}", part2(&mut black_tiles));
}
use std::collections::{HashMap, HashSet};
use lazy_static::lazy_static;
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

fn tiles() -> Vec<Vec<Pos>> {
    let input = util::file_to_string("input/24");

    input.lines()
        .map(|l| {
            let mut i = 0;
            let mut tile: Vec<Pos> = vec![];

            loop {
                let c = l.chars().nth(i).unwrap();

                match c {
                    'n' => {
                        match l.chars().nth(i + 1).unwrap() {
                            'w' => tile.push(*DIRECTIONS.get("nw").unwrap()),
                            'e' => tile.push(*DIRECTIONS.get("ne").unwrap()),
                            _ => unreachable!()
                        }
                        i += 1;
                    }
                    'e' => tile.push(*DIRECTIONS.get("e").unwrap()),
                    's' => {
                        match l.chars().nth(i + 1).unwrap() {
                            'e' => tile.push(*DIRECTIONS.get("se").unwrap()),
                            'w' => tile.push(*DIRECTIONS.get("sw").unwrap()),
                            _ => unreachable!()
                        }
                        i += 1;
                    }
                    'w' => tile.push(*DIRECTIONS.get("w").unwrap()),
                    _ => unreachable!()
                }

                i += 1;

                if i >= l.len() {
                    break;
                }
            }

            tile
        })
        .collect::<Vec<Vec<_>>>()
}

fn add(a: Pos, b: Pos) -> Pos {
    let mut z: Pos = [0, 0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        z[i] = aval + bval;
    }
    z
}

fn part1(tiles: &Vec<Vec<Pos>>) -> HashSet<Pos> {
    let mut black_tiles: HashSet<Pos> = HashSet::new();

    for moves in tiles.iter() {
        let mut pos: Pos = [0, 0];

        for mv in moves.iter() {
            pos = add(pos, *mv);
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
        let mut add: HashSet<Pos> = HashSet::new();
        let mut remove: HashSet<Pos> = HashSet::new();
        for tile in black_tiles.iter() {
            let neighbours = enumerate_neighbours(&tile, &black_tiles);

            let mut black_neighbours = 0;
            for (neighbour, is_black) in neighbours.iter() {
                if *is_black {
                    black_neighbours += 1;
                } else {
                    if enumerate_neighbours(&neighbour, &black_tiles).iter().filter(|(_, &b)| b).count() == 2 {
                        add.insert(*neighbour);
                    }
                }
            }

            if black_neighbours == 0 || black_neighbours > 2 {
                remove.insert(*tile);
            }
        }

        for pos in add.iter() {
            black_tiles.insert(*pos);
        }

        for pos in remove.iter() {
            black_tiles.remove(pos);
        }
    }

    black_tiles.len()
}

fn main() {
    let tiles = tiles();

    let mut black_tiles = part1(&tiles);
    println!("Part 1: {}", black_tiles.len());
    println!("Part 2: {}", part2(&mut black_tiles));
}
#![feature(iterator_fold_self)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

use std::collections::HashSet;
use regex::Regex;
use util;

#[derive(Clone)]
struct TileEdges {
    top: Vec<char>,
    right: Vec<char>,
    bottom: Vec<char>,
    left: Vec<char>,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coords {
    y: isize,
    x: isize,
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<char>>,
    pos: Coords,
}

bitflags! {
    struct TileEdge: u8 {
        const NONE = 0x00;
        const TOP = 0x01;
        const RIGHT = 0x02;
        const BOTTOM = 0x04;
        const LEFT = 0x08;
    }
}

impl Tile {
    const SIZE: usize = 10;
    const SIZE_CROPPED: usize = 8;

    fn new(id: usize, data: Vec<Vec<char>>) -> Tile {
        Tile { id, data, pos: Coords { x: 0, y: 0 } }
    }

    fn get_edges(&self) -> TileEdges {
        TileEdges {
            top: self.data[0].clone(),
            right: self.data.iter().map(|r| r[Tile::SIZE - 1]).collect(),
            bottom: self.data[Tile::SIZE - 1].clone(),
            left: self.data.iter().map(|r| r[0]).collect(),
        }
    }

    fn find_matching_edge(&self, other: &Tile, check: TileEdge) -> TileEdge {
        let self_edges = self.get_edges();
        let other_edges = other.get_edges();

        if check.contains(TileEdge::TOP) && self_edges.top == other_edges.bottom {
            return TileEdge::TOP;
        }
        if check.contains(TileEdge::RIGHT) && self_edges.right == other_edges.left {
            return TileEdge::RIGHT;
        }
        if check.contains(TileEdge::BOTTOM) && self_edges.bottom == other_edges.top {
            return TileEdge::BOTTOM;
        }
        if check.contains(TileEdge::LEFT) && self_edges.left == other_edges.right {
            return TileEdge::LEFT;
        }

        TileEdge::NONE
    }

    fn cropped(&self) -> Tile {
        let mut clone = self.clone();

        clone.data.remove(Tile::SIZE - 1); // Bottom
        clone.data.remove(0); // Top

        for i in 0..clone.data.len() {
            clone.data[i].remove(Tile::SIZE - 1); // Right
            clone.data[i].remove(0); // Left
        }

        clone
    }
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = grid.len();
    let mut rotated: Vec<Vec<char>> = vec![vec!['.';size];size];
    for i in 0..size {
        for j in 0..size {
            rotated[i][j] = grid[size - j - 1][i];
        }
    }
    rotated
}

fn tiles() -> Vec<Tile> {
    lazy_static! {
        static ref ID_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
    }

    util::file_to_string("input/20").split("\n\n")
        .map(|v| {
            let id = ID_REGEX.captures(v.lines().next().unwrap()).unwrap().get(0).unwrap().as_str().parse::<usize>().unwrap();
            let data = v.lines().skip(1)
                .map(|row| row.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<_>>>();

            Tile::new(id, data)
        })
        .collect::<Vec<_>>()
}

fn arrange_tiles(tiles: &Vec<Tile>, flip_first: bool) -> Vec<Tile> {
    let mut grid: Vec<Tile> = vec![];
    let mut first = tiles[0].clone();
    if flip_first {
        first.data.reverse();
    }
    grid.push(first);

    let mut seen_ids: HashSet<usize> = HashSet::new();
    let mut seen_coords: HashSet<Coords> = HashSet::new();
    seen_ids.insert(tiles[0].id);
    seen_coords.insert(Coords { x: 0, y: 0 });

    while grid.len() < tiles.len() {
        for i in 0..grid.len() {
            let a = &mut grid.to_vec()[i];
            let mut free_edges: TileEdge = TileEdge::NONE;
            if !seen_coords.contains(&Coords { x: a.pos.x, y: a.pos.y - 1 }) {
                free_edges = free_edges | TileEdge::TOP;
            }
            if !seen_coords.contains(&Coords { x: a.pos.x + 1, y: a.pos.y }) {
                free_edges = free_edges | TileEdge::RIGHT;
            }
            if !seen_coords.contains(&Coords { x: a.pos.x - 1, y: a.pos.y }) {
                free_edges = free_edges | TileEdge::LEFT;
            }
            if !seen_coords.contains(&Coords { x: a.pos.x, y: a.pos.y + 1 }) {
                free_edges = free_edges | TileEdge::BOTTOM;
            }

            if free_edges == TileEdge::NONE {
                continue;
            }

            let mut found: Vec<Tile> = vec![];
            let mut edge: TileEdge = TileEdge::NONE;
            'outer: for j in 0..tiles.len() {
                let mut b = tiles.to_vec()[j].clone();
    
                if seen_ids.contains(&b.id) {
                    continue;
                }
    
                for _ in 0..4 {
                    for _ in 0..2 {
                        edge = a.find_matching_edge(&b, free_edges);
        
                        if edge != TileEdge::NONE {
                            found.push(b);
                            break 'outer;
                        }

                        b.data.reverse();
                    }

                    b.data = rotate_grid(&b.data);
                }
            }

            if found.len() == 0 {
                continue;
            }

            let mut neighbour = found[0].clone();
            neighbour.pos = match edge {
                TileEdge::TOP => Coords { x: a.pos.x, y: a.pos.y - 1 },
                TileEdge::RIGHT => Coords { x: a.pos.x + 1, y: a.pos.y },
                TileEdge::BOTTOM => Coords { x: a.pos.x, y: a.pos.y + 1 },
                TileEdge::LEFT => Coords { x: a.pos.x - 1, y: a.pos.y },
                _ => unreachable!()
            };
    
            grid.push(neighbour.clone());
            seen_ids.insert(neighbour.id);
            seen_coords.insert(neighbour.pos);
        }
    }

    grid
}

fn part1(tiles: &Vec<Tile>) -> usize {
    let grid = arrange_tiles(&tiles, false);
    let mut corners: Vec<usize> = vec![];
    corners.push(grid.iter().fold_first(|a, b| if a.pos.x <= b.pos.x && a.pos.y <= b.pos.y { a } else { b }).unwrap().id);
    corners.push(grid.iter().fold_first(|a, b| if a.pos.x <= b.pos.x && a.pos.y >= b.pos.y { a } else { b }).unwrap().id);
    corners.push(grid.iter().fold_first(|a, b| if a.pos.x >= b.pos.x && a.pos.y >= b.pos.y { a } else { b }).unwrap().id);
    corners.push(grid.iter().fold_first(|a, b| if a.pos.x >= b.pos.x && a.pos.y <= b.pos.y { a } else { b }).unwrap().id);

    corners.iter().product()
}

fn part2(tiles: &Vec<Tile>) -> usize {
    const SEA_MONSTER_SIZE: usize = 15;

    lazy_static! {
        static ref SEA_MONSTER_TOP_REGEX: Regex = Regex::new(r".{18}#.").unwrap();
        static ref SEA_MONSTER_MID_REGEX: Regex = Regex::new(r"#.{4}##.{4}##.{4}###").unwrap();
        static ref SEA_MONSTER_BOT_REGEX: Regex = Regex::new(r".#..#..#..#..#..#...").unwrap();
    }

    let mut grid = arrange_tiles(&tiles, true);
    let sqrt = (grid.len() as f64).sqrt() as usize;
    
    grid.sort_by_key(|t| t.clone().pos);

    let mut image: Vec<Vec<char>> = vec![];
    for i in 0..sqrt {
        for j in 0..Tile::SIZE_CROPPED {
            let mut line: Vec<char> = vec![];
            for k in 0..sqrt {
                let tile = grid[(i as usize * sqrt) + k as usize].cropped();
                for c in tile.data[j as usize].iter() {
                    line.push(*c);
                }
            }
            image.push(line);
        }
    }

    let num_pounds: usize = image.iter().map(|r| r.iter().collect::<String>().matches("#").count()).sum();

    let mut monsters = 0;
    for _ in 0..4 {
        for _ in 0..2 {
            for i in 0..image.len() - 1 {
                let row_mid = image[i].iter().collect::<String>();
                let match_mid = SEA_MONSTER_MID_REGEX.find(&row_mid);

                if match_mid.is_some() {
                    let index = match_mid.unwrap().start();

                    let row_top = image[i - 1].iter().skip(index).collect::<String>().clone();
                    let match_top = SEA_MONSTER_TOP_REGEX.find(&row_top);

                    if match_top.is_some() {
                        let row_bot = image[i + 1].iter().skip(index).collect::<String>();
                        let match_bot = SEA_MONSTER_BOT_REGEX.find(&row_bot);
        
                        if match_bot.is_some() {
                            monsters += 1;
                        }
                    }
                }
            }

            image.reverse();
        }

        image = rotate_grid(&image);
    }

    num_pounds - (monsters * SEA_MONSTER_SIZE)
}

fn main() {
    let tiles = tiles();

    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
}
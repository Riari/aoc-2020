#[macro_use]
extern crate lazy_static;

use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};
use regex::Regex;
use util;

type Ranges = (RangeInclusive<u16>, RangeInclusive<u16>);
type Fields = HashMap<String, Ranges>;
type Ticket = Vec<u16>;

struct Notes {
    fields: Fields,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn notes() -> Notes {
    lazy_static! {
        static ref FIELD_RANGE_REGEX: Regex = Regex::new(r"(?P<field>.+):\s(?P<range_a>\S+) or (?P<range_b>\S+)").unwrap();
    }

    let mut notes = Notes {
        fields: HashMap::new(),
        my_ticket: vec![],
        nearby_tickets: vec![],
    };

    let input = util::file_to_string("input/16");

    for (i, line) in input.lines().enumerate() {
        if i < 20 {
            let caps = FIELD_RANGE_REGEX.captures(line).unwrap();
            let mut range_a = caps.name("range_a").unwrap().as_str().split("-");
            let mut range_b = caps.name("range_b").unwrap().as_str().split("-");
            notes.fields.insert(
                caps.name("field").unwrap().as_str().to_string(),
                (
                    (range_a.next().unwrap().parse().unwrap()..=range_a.next().unwrap().parse().unwrap()),
                    (range_b.next().unwrap().parse().unwrap()..=range_b.next().unwrap().parse().unwrap()),
                )
            );
        } else if i == 22 || i > 24 {
            let ticket = line.split(",").map(|v| v.parse::<u16>().unwrap()).collect();
            if i == 22 {
                notes.my_ticket = ticket;
            } else if i > 24 {
                notes.nearby_tickets.push(ticket);
            }
        }
    }

    notes
}

fn fits_ranges(ranges: &Ranges, value: u16) -> bool {
    ranges.0.contains(&value) || ranges.1.contains(&value)
}

fn invalidate_ticket(fields: &Fields, ticket: &Ticket) -> Ticket {
    let mut invalid: Ticket = vec![];
    for v in ticket.iter() {
        let mut valid = false;
        for ranges in fields.values() {
            if fits_ranges(&ranges, *v) {
                valid = true;
                break;
            }
        }

        if !valid {
            invalid.push(*v);
        }
    }
    
    invalid
}

fn invalidate_tickets(notes: &Notes) -> HashMap<u16, Ticket> {
    let mut invalid_tickets: HashMap<u16, Ticket> = HashMap::new();

    for (i, t) in notes.nearby_tickets.iter().enumerate() {
        let invalid = invalidate_ticket(&notes.fields, &t);
        if invalid.len() > 0 {
            invalid_tickets.insert(i as u16, invalid);
        }
    }
    
    invalid_tickets
}

fn part1(notes: &Notes) -> u16 {
    invalidate_tickets(&notes).values()
        .map(|t| t.iter().sum::<u16>())
        .sum()
}

fn find_field_positions(fields: &Fields, tickets: &Vec<&Ticket>) -> HashMap<String, u16> {
    let mut valid_rules: HashMap<u16, HashSet<String>> = HashMap::new();
    for i in 0..fields.len() {
        for (field, ranges) in fields.iter() {
            let valid_field_count = tickets.iter()
                .filter(|t| fits_ranges(&ranges, t[i]))
                .count();
    
            if valid_field_count == tickets.len() {
                // Rule is valid for this column
                valid_rules.entry(i as u16).or_insert_with(HashSet::new).insert(field.to_string());
            }
        }
    }

    let mut rules: Vec<(u16, &HashSet<String>)> = vec![];
    for (column, field_names) in valid_rules.iter() {
        rules.push((*column, field_names));
    }

    rules.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut positions: HashMap<String, u16> = HashMap::new();
    let mut dupes: Vec<String> = vec![];
    for (column, field_names) in rules {
        let mut names = field_names.clone();
        for dupe in dupes.iter() {
            names.remove(dupe);
        }

        let name = names.iter().nth(0).unwrap();

        dupes.push(name.to_string());
        positions.insert(name.to_string(), column);
    }

    positions
}

fn part2(notes: &Notes) -> u64 {
    let tickets: Vec<&Ticket> = notes.nearby_tickets.iter()
        .filter(|t| invalidate_ticket(&notes.fields, &t).len() == 0)
        .collect::<Vec<_>>();

    let field_positions = find_field_positions(&notes.fields, &tickets);
    field_positions.iter()
        .filter(|(f, _)| f.starts_with("departure"))
        .collect::<HashMap<_, _>>()
        .iter()
        .map(|(_, p)| notes.my_ticket[**p as usize] as u64)
        .product()
}

fn main() {
    let notes = notes();

    println!("Part 1: {}", part1(&notes));
    println!("Part 2: {}", part2(&notes));
}
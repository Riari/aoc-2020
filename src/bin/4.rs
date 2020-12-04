use std::collections::HashMap;
use util;

enum Height {
    Cm(u16),
    In(u16),
}

impl Height {
    fn from_string(input: &str) -> Option<Height> {
        let without_suffix = input
            .replace(|c: char| c.is_alphabetic(), "")
            .parse()
            .ok()?;

        Some(match input {
            h if h.ends_with("cm") => Height::Cm(without_suffix),
            h if h.ends_with("in") => Height::In(without_suffix),
            _ => return None,
        })
    }
}

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn from_map<'a>(map: &'a HashMap<&str, &str>) -> Option<Passport> {
        Some(Passport {
            byr: map.get("byr")?.parse().ok()?,
            iyr: map.get("iyr")?.parse().ok()?,
            eyr: map.get("eyr")?.parse().ok()?,
            hgt: map.get("hgt")?.to_string(),
            hcl: map.get("hcl")?.to_string(),
            ecl: map.get("ecl")?.to_string(),
            pid: map.get("pid")?.to_string(),
        })
    }

    fn is_valid(&self) -> bool {
        let mut hcl_chars = self.hcl.chars();

        self.byr >= 1920
            && self.byr <= 2002
            && self.iyr >= 2010
            && self.iyr <= 2020
            && self.eyr >= 2020
            && self.eyr <= 2030
            && match Height::from_string(&self.hgt) {
                Some(Height::Cm(h)) => h >= 150 && h <= 193,
                Some(Height::In(h)) => h >= 59 && h <= 76,
                None => false,
            }
            && hcl_chars.next().unwrap() == '#'
            && hcl_chars.all(|c| c.is_ascii_hexdigit())
            && self.hcl.len() == 7
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().any(|e| e == &self.ecl)
            && self.pid.chars().all(|c| c.is_numeric())
            && self.pid.len() == 9
    }
}

fn passports() -> Vec<Passport> {
    return util::file_to_string("input/4")
        .split("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|pair| {
                    let mut parts = pair.split(':');
                    (parts.next().unwrap(), parts.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .filter_map(Passport::from_map)
        .collect();
}

fn part1(passports: &Vec<Passport>) -> usize {
    return passports.iter().count();
}

fn part2(passports: &Vec<Passport>) -> usize {
    return passports.iter().filter(|p| p.is_valid()).count();
}

fn main() {
    let passports = passports();

    println!("Part 1: {}", part1(&passports));
    println!("Part 2: {}", part2(&passports));
}
use std::ops::RangeInclusive;
use util;

struct PasswordModel {
    accepted_range: RangeInclusive<i32>,
    character: char,
    password: String,
}

fn values() -> Vec<PasswordModel> {
    let rows: Vec<String> = util::file_to_vec("input/02");
    let mut values = Vec::<PasswordModel>::new();

    for row in rows.iter() {
        let parts: Vec<&str> = row.split_whitespace().collect();
        let range: Vec<&str> = parts[0].split('-').collect();
        let character = parts[1].chars().nth(0).unwrap();
        let password = parts[2];

        values.push(PasswordModel {
            accepted_range: (range[0].parse().unwrap()..=range[1].parse().unwrap()),
            character,
            password: password.to_owned()
        });
    }

    return values;
}

fn part1() -> i32 {
    let values = values();

    let mut result: i32 = 0;
    for value in values.iter() {
        let occurrences = value.password.matches(value.character).count() as i32;
        
        if value.accepted_range.contains(&occurrences) {
            result = result + 1;
        }
    }

    return result;
}

fn part2() -> i32 {
    let values = values();

    let mut result: i32 = 0;
    for value in values.iter() {
        let mut chars = String::from("");
        chars.push(value.password.chars().nth((value.accepted_range.start() - 1) as usize).unwrap());
        chars.push(value.password.chars().nth((value.accepted_range.end() - 1) as usize).unwrap());
        let occurrences = chars.matches(value.character).count();

        if occurrences == 1 {
            result = result + 1;
        }
    }
    
    return result;
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
#![feature(array_windows)]
use util;

fn part1(adapters: &Vec<i32>) -> usize {
    let mut diffs_one: usize = 0;
    let mut diffs_three: usize = 1; // Accounts for device
    
    for (i, jolt) in adapters.iter().skip(1).enumerate() {
        let diff = jolt - adapters[i];

        match diff {
            1 => diffs_one += 1,
            3 => diffs_three += 1,
            _ => panic!("Unexpected jolt difference")
        }
    }

    diffs_one * diffs_three
}

// https://www.reddit.com/r/rust/comments/ka9nre/advent_of_code_2020_day_10/gf9gtnk/
fn part2(adapters: &Vec<i32>) -> i64 {
    adapters.array_windows()  
        .collect::<Vec<_>>()  
        .split(|[a,b]| b - a == 3)  
        .map(|x| match x.len() {  
            4 => 7,  
            3 => 4,  
            2 => 2,  
            _ => 1  
        })  
        .product()
}

fn main() {
    let mut adapters: Vec<i32> = util::file_to_vec("input/10");
    adapters.sort();
    adapters.insert(0, 0); // Charging port

    println!("Part 1: {}", part1(&adapters));
    println!("Part 2: {}", part2(&adapters));
}
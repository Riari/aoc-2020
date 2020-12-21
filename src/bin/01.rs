use util;

fn values() -> Vec<i32> {
    return util::file_to_vec("input/01");
}

fn part1() -> i32 {
    let values = values();

    let mut result: i32 = 0;
    for a in &values {
        let b = &(2020 - a);
        if values.contains(b) {
            result = a * b;
            break;
        }
    }

    return result;
}

fn part2() -> i32 {
    let mut values = values();
    let mut l: usize;
    let mut r: usize;
    let sum = 2020;

    values.sort();
    
    let mut result: i32 = 0;
    for (i, a) in values.iter().enumerate() {
        l = i + 1;
        r = values.len() - 1;

        while l < r {
            let b = values[l];
            let c = values[r];
            let total = a + b + c;

            if total == sum {
                result = a * b * c;
                break;
            } else if total < sum {
                l = l + 1;
            } else {
                r = r - 1;
            }
        } 
    }

    return result;
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

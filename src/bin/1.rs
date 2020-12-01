use util;

fn values() -> Vec<i32> {
    return util::file_to_vec("input/1");
}

fn part1() {
    let values = values();
    for a in &values {
        let b = &(2020 - a);
        if values.contains(b) {
            println!("{}", a * b);
            return;
        }
    }
}

fn part2() {
    let mut values = values();
    let mut l: usize;
    let mut r: usize;
    let sum = 2020;

    values.sort();
    
    for (i, a) in values.iter().enumerate() {
        l = i + 1;
        r = values.len() - 1;

        while l < r {
            let b = values[l];
            let c = values[r];
            let total = a + b + c;

            if total == sum { 
                println!("{}", a * b * c); 
                return;
            } else if total < sum {
                l = l + 1;
            } else {
                r = r - 1;
            }
        } 
    }
}

fn main() {
    part1();
    part2();
}

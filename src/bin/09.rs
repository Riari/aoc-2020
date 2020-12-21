use std::cmp::Ordering;
use std::ops::Add;
use util;

struct Row {
    index: usize,
    value: usize,
}

fn find_first_invalid_row(input: &Vec<usize>) -> Option<Row> {
    let preamble = 25;
    for (i, n) in input.iter().skip(preamble).enumerate() {
        let slice: &[usize] = &input[i..preamble + i];

        if !contains_pair_summing_to(slice, *n) {
            return Some(Row { index: preamble + i, value: *n });
        }
    }

    None
}

fn contains_pair_summing_to<T>(slice: &[T], sum: T) -> bool
where
    T: Add<Output = T> + Ord + Copy,
{
    let mut s = slice.to_owned();
    s.sort();

    let mut i = 0;
    let mut j = s.len() - 1;
 
    while i < j {
        match (s[i] + s[j]).cmp(&sum) {
            Ordering::Equal => return true,
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }
 
    false
}

fn find_vec_summing_to(input: &Vec<usize>, row: &Row) -> Option<Vec<usize>> {
    let slice = &input.to_owned()[0..row.index];

    let mut sum = slice[0];
    let mut start = 0;
    let mut end = 1;

    loop {
        while sum < row.value && end < row.index {
            sum = sum + slice[end];
            end += 1;
        }

        if sum == row.value {
            return Some(slice[start..end].to_vec());
        }

        sum = sum - slice[start];
        start += 1;
    }
}

fn main() {
    let input: Vec<usize> = util::file_to_vec("input/09");
    let first_invalid_row = find_first_invalid_row(&input).unwrap();
    let vec_summing_to = find_vec_summing_to(&input, &first_invalid_row).unwrap();
    let final_sum = vec_summing_to.iter().min().unwrap() + vec_summing_to.iter().max().unwrap();

    println!("Part 1: {}", first_invalid_row.value);
    println!("Part 2: {}", final_sum);
}
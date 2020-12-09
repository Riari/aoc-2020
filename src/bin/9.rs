use std::cmp::Ordering;
use std::ops::Add;
use util;

fn find_pair_summing_to<T>(slice: &[T], sum: T) -> Option<(usize, usize)>
where
    T: Add<Output = T> + Ord + Copy,
{
    let mut s = slice.to_owned();
    s.sort();

    let mut i = 0;
    let mut j = s.len() - 1;
 
    while i < j {
        match (s[i] + s[j]).cmp(&sum) {
            Ordering::Equal => return Some((i, j)),
            Ordering::Less => i += 1,
            Ordering::Greater => j -= 1,
        }
    }
 
    None
}

fn find_first_invalid_number(input: &Vec<usize>) -> Option<(usize, usize)> {
    let preamble = 25;
    for (i, n) in input.iter().skip(preamble).enumerate() {
        let slice: &[usize] = &input[i..preamble + i];
        let result = find_pair_summing_to(slice, *n);

        if result.is_none() {
            return Some((preamble + i, *n));
        }
    }

    None
}

fn find_set_summing_to(input: &Vec<usize>, i: usize, value: usize) -> Option<Vec<usize>> {
    let slice = &input.to_owned()[0..i];

    let mut sum = slice[0];
    let mut start = 0;
    let mut end = 1;

    loop {
        while sum < value && end < i {
            sum = sum + slice[end];
            end += 1;
        }

        if sum == value {
            return Some(slice[start..end].to_vec());
        }

        sum = sum - slice[start];
        start += 1;
    }
}

fn main() {
    let input: Vec<usize> = util::file_to_vec("input/9");
    let first_invalid = find_first_invalid_number(&input).unwrap();
    let mut set_summing_to = find_set_summing_to(&input, first_invalid.0, first_invalid.1).unwrap();
    set_summing_to.sort();
    let final_sum = set_summing_to[0] + set_summing_to[set_summing_to.len() - 1];

    println!("Part 1: {}", first_invalid.1);
    println!("Part 2: {}", final_sum);
}
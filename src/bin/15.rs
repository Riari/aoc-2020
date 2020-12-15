use std::collections::HashMap;

fn play(until: u32) -> u32 {
    let mut played = [13,16,0,12,15,1].iter()
        .enumerate()
        .map(|(i,&n)| (n, (i + 1) as u32))
        .collect::<HashMap<_,_>>();
    
    (7..until).fold(0, |prev, i| i - played.insert(prev, i).unwrap_or(i))
}

fn main() {
    println!("Part 1: {}", play(2020));
    println!("Part 2: {}", play(30000000));
}
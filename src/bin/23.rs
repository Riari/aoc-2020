fn play(input: &Vec<usize>, moves: usize) -> Vec<usize> {
    let mut cups: Vec<usize> = vec![0;input.len() + 1];
    for (i, v) in input.iter().enumerate() {
        let val = input[if i >= input.len() - 1 { 0 } else { i + 1 }];
        cups[*v] = val;
    }

    cups[0] = input[0];

    let max = cups.len() - 1;

    let mut current = cups[0];
    for _ in 0..moves {
        let p1 = cups[current];
        let p2 = cups[p1];
        let p3 = cups[p2];

        let mut dest = current;
        while vec![current, p1, p2, p3].contains(&dest) {
            dest = if dest > 1 { dest - 1 } else { max };
        }
        
        cups[current] = cups[p3];
        cups[p3] = cups[dest];
        cups[dest] = p1;
        current = cups[current];
    }

    let mut final_cups: Vec<usize> = vec![];
    let mut cup = cups[1];
    while cups[cup] != 1 {
        final_cups.push(cup);
        cup = cups[cup];
    }
    final_cups.push(cup);

    final_cups
}

fn main() {
    let mut input: Vec<usize> = vec![4,6,7,5,2,8,1,9,3];

    println!("Part 1: {}", play(&input, 100).iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""));

    for i in input.len() + 1..=1_000_000 {
        input.push(i);
    }

    let p2 = play(&input, 10_000_000);
    println!("Part 2: {}", p2[0] * p2[1]);
}
use std::collections::HashMap;
use itertools::{iproduct, izip};
use util;

type State = Vec<Vec<i8>>;

const MODIFIERS: [i8; 3] = [-1, 0, 1];
const CYCLES: i8 = 6;

fn get_initial_state() -> State {
    let input: Vec<String> = util::file_to_vec("input/17");

    let mut state: State = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y].chars().nth(x).unwrap() == '#' {
                state.push(vec![x as i8, y as i8, 0]);
            }
        }
    }

    state
}

fn simulate(initial_state: &State, moves: &State) -> State {
    let mut state = initial_state.clone();

    let dimensions = moves[0].len();
    if state[0].len() < dimensions {
        state = state.iter_mut()
            .map(|c| {
                for _ in 0..dimensions - c.len() {
                    c.push(0);
                }
                c.clone()
            })
            .collect();
    }

    for _ in 0..CYCLES {
        let mut neighbours: HashMap<Vec<i8>, i8> = HashMap::new();
        for cube in state.iter() {
            for mv in moves.iter() {
                let mut n: Vec<i8> = vec![];
                for (c, m) in izip!(cube, mv) {
                    n.push(c + m);
                }
                let count = if neighbours.contains_key(&n) { neighbours[&n] + 1 } else { 1 };
                neighbours.insert(n, count);
            }
        }

        state = neighbours.iter()
            .filter(|(c, n)| {
                state.contains(c) && (**n == 2 || **n == 3)
                || !state.contains(c) && **n == 3
            })
            .map(|(c, _)| c.clone())
            .collect();
    }

    state
}

fn part1(initial_state: &State) -> usize {
    let moves = &iproduct!(&MODIFIERS, &MODIFIERS, &MODIFIERS)
        .map(|m| vec![*m.0, *m.1, *m.2])
        .filter(|m| *m != vec![0, 0, 0])
        .collect();

    simulate(&initial_state, moves).len()
}

fn part2(initial_state: &State) -> usize {
    let moves = &iproduct!(&MODIFIERS, &MODIFIERS, &MODIFIERS, &MODIFIERS)
        .map(|m| vec![*m.0, *m.1, *m.2, *m.3])
        .filter(|m| *m != vec![0, 0, 0, 0])
        .collect();

    simulate(&initial_state, moves).len()
}

fn main() {
    let state = get_initial_state();

    println!("Part 1: {}", part1(&state));
    println!("Part 2: {}", part2(&state));
}
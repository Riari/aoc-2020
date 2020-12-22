use std::collections::{HashSet, VecDeque};
use util;

type Deck = VecDeque<usize>;

fn players() -> (Deck, Deck) {
    let input = util::file_to_string("input/22");
    let mut players = input.split("\n\n");

    fn extract_deck(input: &str) -> Deck {
        input.split("\n").filter(|v| !v.starts_with("Player")).map(|v| v.parse::<usize>().unwrap()).collect()
    }

    (extract_deck(players.next().unwrap()), extract_deck(players.next().unwrap()))
}

fn get_score(deck: &Deck) -> usize {
    deck.iter().enumerate()
        .map(|(i, c)| c * (deck.len() - i))
        .sum()
}

fn play(p1: &mut Deck, p2: &mut Deck, recursive: bool) -> usize {
    let mut seen: HashSet<(Deck, Deck)> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if recursive && !seen.insert((p1.clone(), p2.clone())) {
            return 1;
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let p1_wins: bool;
        if recursive && c1 <= p1.len() && c2 <= p2.len() {
            p1_wins = play(&mut p1.iter().take(c1).copied().collect(), &mut p2.iter().take(c2).copied().collect(), recursive) == 1;
        } else {
            p1_wins = c1 > c2;
        }

        if p1_wins {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    if p1.len() > 0 { 1 } else { 2 }
}

fn play_game(players: &(Deck, Deck), recursive: bool) -> usize {
    let mut p1 = &mut players.0.clone();
    let mut p2 = &mut players.1.clone();

    let winner = play(&mut p1, &mut p2, recursive);
    
    get_score(if winner == 1 { p1 } else { p2 })
}

fn main() {
    let players = players();

    println!("Part 1: {}", play_game(&players, false));
    println!("Part 2: {}", play_game(&players, true));
}
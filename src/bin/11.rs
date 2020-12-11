use util;

const EMPTY: char = 'L';
const OCCUPIED: char = '#';
const FLOOR: char = '.';

fn count_occupied_seats(plan: &Vec<String>, at_x: usize, at_y: usize, adjacent_only: bool) -> usize {
    let neighbours = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1)
    ];
    let width = plan[0].chars().count() as isize;
    let height = plan.len() as isize;

    let mut result: usize = 0;
    for (i, j) in neighbours {
        let mut seat = FLOOR;
        let mut y = at_y as isize;
        let mut x = at_x as isize;

        while seat == FLOOR {
            y += i;
            x += j;

            if y < 0 || x < 0 || y >= height || x >= width {
                break;
            }

            seat = plan[y as usize].chars().nth(x as usize).unwrap();

            if seat == OCCUPIED {
                result += 1;
                break;
            }

            if adjacent_only {
                break;
            }
        }
    }

    result
}

fn simulate_seating(plan: &Vec<String>, check_adjacent_only: bool, tolerance: usize) -> Vec<String> {
    let mut changed = false;
    let new_plan: Vec<String> = plan.iter()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| {
                    match c {
                        EMPTY => {
                            if count_occupied_seats(&plan, j, i, check_adjacent_only) == 0 {
                                changed = true;
                                OCCUPIED
                            } else {
                                c
                            }
                        },
                        OCCUPIED => {
                            if count_occupied_seats(&plan, j, i, check_adjacent_only) >= tolerance {
                                changed = true;
                                EMPTY
                            } else {
                                c
                            }
                        },
                        FLOOR => c,
                        _ => panic!("Invalid seat state"),
                    }
                })
                .collect::<String>()
        })
        .collect();

    if changed {
        return simulate_seating(&new_plan, check_adjacent_only, tolerance);
    }

    new_plan
}

fn part1(plan: &Vec<String>) -> usize {
    simulate_seating(&plan, true, 4)
        .iter()
        .map(|row| row.matches(OCCUPIED).count())
        .sum()
}

fn part2(plan: &Vec<String>) -> usize {
    simulate_seating(&plan, false, 5)
        .iter()
        .map(|row| row.matches(OCCUPIED).count())
        .sum()
}

fn main() {
    let plan: Vec<String> = util::file_to_vec("input/11");

    println!("Part 1: {}", part1(&plan));
    println!("Part 1: {}", part2(&plan));
}
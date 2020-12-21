use util;

fn rows() -> Vec<String> {
    return util::file_to_vec("input/3");
}

fn part1(rows: &[String]) -> usize {
    return traverse(rows, &3, &1);
}

fn part2(rows: &[String]) -> usize {
    let slopes = vec![
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2],
    ];

    let mut tree_counts: Vec<usize> = vec![];
    for slope in slopes {
        tree_counts.push(traverse(rows, &slope[0], &slope[1]));
    }
    
    return tree_counts.iter().product();
}

fn traverse(rows: &[String], right: &usize, down: &usize) -> usize {
    let width = rows[0].chars().count();

    let mut x = 0;
    let mut y = 0;
    let mut result: usize = 0;
    for row in rows.iter().skip(1) {
        y += 1;

        if y % down != 0 {
            continue;
        }

        x += right;
        result += (row.chars().nth(x % width).unwrap() == '#') as usize;
    }

    return result;
}

fn main() {
    let rows: Vec<String> = rows();
    println!("Part 1: {}", part1(&rows));
    println!("Part 2: {}", part2(&rows));
}
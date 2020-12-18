use util;

fn extract_first_num(expr: &String) -> (usize, usize) {
    let mut end = 0;
    for (j, next) in expr.chars().enumerate() {
        if next == '+' || next == '*' {
            break;
        }
        end = j;
    }
    (
        expr[..=end].chars().collect::<String>().parse::<usize>().unwrap(),
        end + 1
    )
}

fn op_expr(expr: &String, op: Option<char>) -> String {
    let mut new_expr: Vec<String> = vec![];
    let mut acc = extract_first_num(&expr).0;
    let mut i = 0;
    let mut c: char;
    loop {
        c = expr.chars().nth(i).unwrap();
        if c == '+' || c == '*' {
            let b = extract_first_num(&expr[i+1..].to_string());

            if (op.is_some() && c == op.unwrap()) || op.is_none() {
                acc = op_pair(acc, b.0, c);
            } else {
                new_expr.push(acc.to_string());
                acc = b.0;
                new_expr.push(c.to_string());
            }
            
            i += b.1;
        } else {
            i += 1;
        }

        if i >= expr.len() {
            break;
        }
    }

    new_expr.push(acc.to_string());

    new_expr.join("")
}

fn op_pair(a: usize, b: usize, op: char) -> usize {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unexpected op {}", op),
    }
}

fn solve(expression: &String, priority_op: Option<char>) -> usize {
    let mut expr = expression.clone();
    while expr.contains("(") {
        let mut lparen = 0;
        for (i, c) in expr.chars().enumerate() {
            if c == '(' {
                lparen = i;
            }
        }

        let mut rparen = 0;
        for (i, c) in expr[lparen+1..].chars().enumerate() {
            if c == ')' {
                rparen = lparen + i;
                break;
            }
        }

        let mut inner_expr = expr[lparen+1..=rparen].to_string();
        if priority_op.is_some() {
            inner_expr = op_expr(&inner_expr, priority_op);
        }
        inner_expr = op_expr(&inner_expr, None);
        let mut new_expr: Vec<String> = vec![];
        for (i, c) in expr.chars().enumerate() {
            if i == lparen+1 {
                new_expr.push(inner_expr.clone());
            } else if (lparen..=rparen+1).contains(&i) {
                continue;
            } else {
                new_expr.push(c.to_string());
            }
        }

        expr = new_expr.join("");
    }

    if priority_op.is_some() {
        expr = op_expr(&expr, priority_op);
    }
    expr = op_expr(&expr, None);

    expr.to_string().parse::<usize>().unwrap()
}

fn part1(input: &String) -> usize {
    input.lines().map(|l| solve(&l.to_string().split_whitespace().collect(), None)).sum()
}

fn part2(input: &String) -> usize {
    input.lines().map(|l| solve(&l.to_string().split_whitespace().collect(), Some('+'))).sum()
}

fn main() {
    let input = util::file_to_string("input/18");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
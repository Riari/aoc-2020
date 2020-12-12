use util;

struct Point(isize, isize);

enum Cardinal {
    North,
    East,
    South,
    West,
}

struct Instruction {
    action: char,
    value: isize,
}

struct Ship {
    angle: isize,
    position: Point,
    waypoint: Point,
    use_waypoint: bool,
}

impl Ship {
    fn new(waypoint: Option<Point>) -> Ship {
        let use_waypoint = waypoint.is_some();
        Ship {
            angle: 90, // Starts by facing east
            position: Point(0, 0),
            waypoint: waypoint.unwrap_or(Point(0, 0)),
            use_waypoint: use_waypoint,
        }
    }

    fn get_cardinal(&self) -> Cardinal {
        match ((self.angle % 360) + 360) % 360 {
            0 => Cardinal::North,
            90 => Cardinal::East,
            180 => Cardinal::South,
            270 => Cardinal::West,
            _ => panic!("Impossible...")
        }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst.action {
            'N' => self.translate(Cardinal::North, inst.value),
            'S' => self.translate(Cardinal::South, inst.value),
            'E' => self.translate(Cardinal::East, inst.value),
            'W' => self.translate(Cardinal::West, inst.value),
            'L' => self.rotate(inst.action, inst.value),
            'R' => self.rotate(inst.action, inst.value),
            'F' => {
                if self.use_waypoint {
                    self.translate_to_waypoint(inst.value);
                } else {
                    self.translate(self.get_cardinal(), inst.value)
                }
            },
            _ => panic!("Invalid instruction"),
        }
    }

    fn rotate(&mut self, direction: char, value: isize) {
        let v = if direction == 'L' { -value } else { value };
        if self.use_waypoint {
            for _ in 0..((((v / 90) % 4) + 4) % 4) {
                self.waypoint = Point(self.waypoint.1, -self.waypoint.0);
            }
        } else {
            self.angle += v;
        }
    }

    fn translate(&mut self, cardinal: Cardinal, value: isize) {
        if self.use_waypoint {
            match cardinal {
                Cardinal::North => self.waypoint.1 += value,
                Cardinal::South => self.waypoint.1 -= value,
                Cardinal::East => self.waypoint.0 += value,
                Cardinal::West => self.waypoint.0 -= value,
            }
        } else {
            match cardinal {
                Cardinal::North => self.position.1 += value,
                Cardinal::South => self.position.1 -= value,
                Cardinal::East => self.position.0 += value,
                Cardinal::West => self.position.0 -= value,
            }
        }
    }

    fn translate_to_waypoint(&mut self, value: isize) {
        self.position.0 += value * self.waypoint.0;
        self.position.1 += value * self.waypoint.1;
    }
}

fn part1(instructions: &Vec<Instruction>) -> isize {
    let mut ship = Ship::new(None);

    for inst in instructions {
        ship.execute(&inst);
    }
    
    ship.position.0.abs() + ship.position.1.abs()
}

fn part2(instructions: &Vec<Instruction>) -> isize {
    let mut ship = Ship::new(Some(Point(10, 1)));

    for inst in instructions {
        ship.execute(&inst);
    }
    
    ship.position.0.abs() + ship.position.1.abs()
}

fn main() {
    let instructions: Vec<Instruction> = util::file_to_string("input/12").lines()
        .map(|l| Instruction { action: l.chars().nth(0).unwrap(), value: l.get(1..).unwrap().parse().unwrap() })
        .collect();

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
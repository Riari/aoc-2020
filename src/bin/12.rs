use util;

struct Point(isize, isize);

struct Ship {
    angle: isize,
    position: Point,
    waypoint: Point,
    use_waypoint: bool,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            angle: 90, // Starts by facing east
            position: Point(0, 0),
            waypoint: Point(1, 1),
            use_waypoint: false,
        }
    }

    fn set_waypoint(&mut self, value: Point) {
        self.waypoint = value;
    }

    fn set_use_waypoint(&mut self, use_waypoint: bool) {
        self.use_waypoint = use_waypoint;
    }

    fn get_cardinal(&self) -> Cardinal {
        match self.angle % 360 {
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
            'F' => self.translate(self.get_cardinal(), inst.value),
            _ => panic!("Invalid instruction"),
        }
    }

    fn rotate(&mut self, direction: char, value: isize) {
        let mut v = value;
        if direction == 'L' && value != 180 {
            v += 180;
        }
        self.angle += v % 360;
    }

    fn translate(&mut self, cardinal: Cardinal, value: isize) {
        match cardinal {
            Cardinal::North => self.position.1 += value,
            Cardinal::South => self.position.1 -= value,
            Cardinal::East => self.position.0 += value,
            Cardinal::West => self.position.0 -= value,
        }
    }
}

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

fn part1(instructions: &Vec<Instruction>) -> isize {
    let mut ship = Ship::new();

    for inst in instructions {
        ship.execute(&inst);
    }
    
    ship.position.0.abs() + ship.position.1.abs()
}

fn part2(instructions: &Vec<Instruction>) -> isize {
    let mut ship = Ship::new();
    ship.set_use_waypoint(true);
    ship.set_waypoint(Point(10, 1));

    for inst in instructions {
        ship.execute(&inst);
    }
    
    ship.position.0.abs() + ship.position.1.abs()
}

fn main() {
    let input: Vec<String> = util::file_to_vec("input/12");
    let instructions: Vec<Instruction> = input.iter()
        .map(|l| Instruction { action: l.chars().nth(0).unwrap(), value: l.get(1..).unwrap().parse().unwrap() })
        .collect();

    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
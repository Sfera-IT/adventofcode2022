use std::collections::HashSet;
use std::iter::Iterator;

const MOVES: &str = "R2;D2;U2;D1;L1;D2;R2;L2;R1;D2;L1;U2;R1;D1;R1;L1;D1;R2;U2;L2;R2;U2;R1;L1;D2;R1;D1;R1;D1;R1;L2;R1;D1;L1;U2;L1;D1;L1;R1;U2;D1;U2;L2;R2;D1;L1;R1;D1;L1;U1;R2;D1;R1;D2;L1;D2;L1;D2;R1;L1;D2;L1;R1;D2;L2;R2;L1;U2;R1;U2;R2;U1;D1;U2;L1;D2;L2;R2;L1;R1;U2;D1;R1;L2;U1;D2;U1;L2;R2;D1;U1;L2;U2;L1;R1;D1;R2;D1;R1;D2;R1;D1;R1;D2;L2;U2;L1;D1;L1;R2;U1;D1;U2;L2;D3;L3;U3;R1;D3;R1;D3;R2;D2;U3;D2;L3;R3;U3;R1;U3;D3;L1;D2;U1;R2;L2;R3;L2;D3;L2;R1;D1;U1;D2;R2;L1;U1;R3;U2;D3;L3;U2;L3;R2;D2;U2;R1;U3;R2;D1;U2;D3;U2;R3;U2;L3;U3;D2;R3;U3;L3;U2;L1;D3;L2;D3;R3;L2;R3;D1;U1;L3;R3;D2;L3;U2;R2;U1;R3;D3;R1;U2;L2;U1;R1;U1;D2;L2;R1;D3;R1;D3;U1;R2;U2;L2;U2;L3;D3;U2;L3;U1;L2;D1;U3;L2;R3;D1;R3;L3;U1;L3;R2;D3;U3;L3;D4;L1;U1;R3;L1;D4;U4;L4;D4;U1;D3;R1;D4;L1;U2;D1;R3;L4;D2;U2;R1;L4;U4;R4;D1;R3;D1;R4;U4;D1;U3;D4;L3;R2;D4;L2;U4;D3;U4;D2;U2;L3;R4;L4;U2;L1;U4;D4;R4;U4;R2;U4;R1;U3;R3;U4;R4;U2;L1;U3;D4;L1;R1;L2;R2;D2;U3;R3;L1;D2;R4;D1;R4;L2;U4;D2;R4;U2;D4;R1;U3;L3;D2;R2;U1;L4;R1;U2;R4;D3;R1;L2;U1;R2;L2;U2;R3;L2;D4;U4;L1;U2;D1;U2;D1;L2;R4;L5;D2;U5;L1;R2;L3;U3;D3;L1;D1;U4;L4;U3;D5;R1;L4;U1;L2;R4;D1;L4;D1;U4;D1;L2;D5;L5;D4;R3;U2;L4;U4;L4;R5;D1;L2;R5;U4;D5;L2;U4;D1;L3;R5;D4;L1;D3;L3;U2;R2;D4;L4;U1;D2;L5;D2;L2;U3;R3;U3;D2;U4;R3;L3;D2;L2;R3;D2;L3;R4;U4;R4;D2;R1;D1;L4;R4;D3;R2;U4;L2;D3;U4;L3;R5;L2;R1;D4;L5;D2;U5;L4;U2;R3;L4;R1;U3;R3;L2;U2;L3;R5;L3;R3;U3;L3;U5;D4;U5;D4;L4;D2;R5;L3;U2;D3;L4;R1;D1;R5;L3;R2;L3;U3;D1;U5;R2;L2;D5;R3;U4;R3;L1;R6;L4;D4;U4;R4;U2;R4;D1;U3;R1;U5;D3;L4;R5;L3;U6;L5;D3;R1;D2;U5;L1;R5;U4;R1;U1;R3;L2;D5;U4;L1;U2;R3;L5;R2;D3;U3;D6;R1;D2;L3;R3;L3;U4;R4;U4;R6;U1;L6;R6;L4;U6;L2;D1;R5;L5;U3;D6;U5;L4;U2;R1;L3;D6;U5;R4;U2;D3;R4;L1;D4;U6;R1;D6;U2;D5;U4;L4;R6;D2;R6;L1;R3;D4;R5;L6;U5;L3;R6;U1;L3;D7;U2;D5;R5;D7;R6;D6;R7;D7;L3;R4;L1;U6;R2;L7;U7;D1;R1;D1;U5;R4;U2;D5;U6;L2;R1;U5;L1;U7;R1;D2;L4;D4;L5;U4;L2;D5;R7;D7;R5;L3;U7;R7;D1;L2;U3;L2;U7;L2;U3;L6;D6;L2;D6;L6;U3;L3;U7;D3;L4;U5;D5;L4;D7;U3;R7;D5;R3;L6;U2;L2;U1;L1;U4;D7;L7;D2;U6;L2;U2;L4;R3;L6;R2;D7;L6;D3;L6;R1;U4;R6;L7;D7;R4;U7;L6;R3;D1;U3;D2;U3;R2;L7;U1;R4;D4;U2;R5;D5;R4;L3;U1;R7;L5;D4;L3;D7;U2;D4;U4;L1;D4;R7;L6;U3;L4;D5;U1;D1;L6;R7;D1;L4;D1;L5;R8;D4;L8;R4;D4;U3;L5;R5;L8;U4;R2;D6;R5;L1;U8;L1;D5;U5;D4;L6;U6;D8;R6;D3;U7;R8;D7;L2;U4;D4;L2;R2;U5;L4;R3;L2;U5;D8;L8;D3;U4;D2;L2;U4;R5;L6;U4;R5;U4;L7;D8;R8;D7;R5;D6;U2;R1;D3;L6;U3;D5;U6;L7;R1;L4;R4;L4;R3;L4;U3;D7;U8;D4;R6;L7;R2;L2;U7;D2;R1;D5;U1;D7;R2;L8;R8;U4;R9;L4;R5;L2;D6;L9;U2;R5;D4;R9;U4;D8;R9;U5;D4;U4;L4;U3;R5;L8;D9;U2;L8;U5;D4;L1;D4;U3;D7;U5;D5;R9;L2;D5;L7;D1;L2;R6;D9;U1;L6;R2;D2;R5;D8;R2;D3;L4;D5;R2;D8;L3;D6;U2;L4;D9;L6;R5;L7;U2;D8;U5;L6;U1;L9;R9;D1;R3;U9;L2;U8;R8;D8;R7;U7;R6;L2;U4;D2;L4;U4;D2;R8;L6;R9;D9;L6;D4;L5;U8;D2;R6;D7;U3;R6;L9;U6;L8;U8;D6;U9;D5;U8;L6;R6;L6;R9;L2;U4;L4;U1;D4;R1;U7;D5;R4;U4;D5;L9;R2;U5;L2;R10;L5;R1;L2;U3;D2;R7;L2;R6;L3;R6;U7;L1;U6;R6;U2;L3;R3;L1;R9;L5;U9;L9;D9;U4;D2;R6;D6;R8;D2;U6;L5;R2;U10;L3;D6;R3;D6;U2;D7;R1;U7;D2;R8;L5;U4;L5;R6;U6;D1;L5;D5;R5;D4;L5;U3;R4;L5;R2;L5;D10;R4;L2;R4;L7;U9;R3;D6;R7;L10;D7;U4;L9;D6;L7;R3;U7;R5;U6;L1;U2;L5;D6;R8;L7;R7;D1;U9;R1;L4;U4;R1;L3;D2;L8;D10;L9;D8;R5;D6;L7;D2;U11;D2;U6;L3;U8;D11;U9;D9;R11;U11;L8;U11;R11;U2;D6;R11;D7;L6;U5;R1;L3;D11;U3;D5;R4;U4;D8;L6;U1;R10;L5;D3;U8;R7;L8;R2;U6;D9;R1;D1;R3;U3;L10;U7;R1;D3;U6;L10;R6;L6;R4;L2;R10;L11;U5;R5;U5;R2;D10;L8;U9;D11;R10;D11;R11;D2;U7;R11;U2;L9;R11;U1;R8;L5;U6;D5;L10;U1;L4;R7;L5;D2;R9;D10;U4;R1;D7;R10;D10;L5;R3;L8;R8;U10;R9;D11;R1;D9;R5;L3;D5;L8;U8;R10;D10;U1;D3;R6;U1;L2;U6;L7;R11;U2;R2;L5;U5;R2;D12;U3;L2;D5;U3;R11;D5;R10;U8;L7;R6;D2;U11;L10;R12;D8;U12;L7;D5;R11;U10;R12;U1;L1;U10;L2;R9;U1;R4;U2;R8;D1;R2;L3;R7;D7;L10;R2;U11;R8;U11;D12;U8;L5;U12;L12;U12;L7;R5;D4;U9;L5;R7;L10;R4;L11;D3;R1;D12;U10;R10;D5;R11;U2;L8;R4;D1;L10;R2;D11;L4;U12;D11;L11;U3;L9;U8;D8;R10;L8;U2;D9;U12;D7;U8;L8;D9;L6;U8;D11;U12;D9;L1;D4;R2;U10;D2;L10;U6;L9;D4;U4;R4;L5;R9;U2;R5;U4;D12;R3;L10;U7;D1;U9;R6;L4;U9;L13;D2;L8;R12;D7;U13;D6;U12;L4;R4;D10;U8;L4;R4;D2;U9;L9;D11;L6;R2;L8;R9;L6;U13;R10;D3;U10;L12;R9;U12;R10;U10;D11;L7;R5;L13;U11;R13;U11;R12;U8;R12;D4;L10;R1;L9;R13;L2;U7;L13;R4;U13;L6;R12;D1;U7;L10;R6;D6;U2;R2;L8;D5;U9;D5;L9;R1;U5;L13;D1;R5;L8;U5;R2;U8;L1;U13;R6;U12;L3;D1;U4;L12;R1;D6;L11;U7;D1;L9;R4;U7;D7;U9;D1;L10;D14;U3;D9;L7;U1;L9;R13;L14;R11;D8;L12;U11;R11;U9;R2;U13;R5;D3;U3;R9;D14;U2;D8;L7;D2;U8;L12;U7;D12;R11;D14;R13;L13;R9;L7;U4;D9;U9;L12;U3;D4;R5;D14;L9;D14;U3;L8;R1;U2;R14;D5;L6;R9;D12;R3;D1;L8;U11;D11;L7;R14;D9;L9;R13;L4;D11;R14;U6;L10;R7;D9;U7;D5;R8;U2;R9;D6;R13;U1;L14;R6;L9;D11;L3;U5;R12;U8;L3;D5;U7;R2;D14;R14;L2;U5;L11;U7;L14;U5;D6;R14;D12;U4;D5;R14;L14;D12;L10;D2;L1;R5;D8;R2;L14;R5;L12;D9;L6;D8;L8;R12;U1;L11;U3;D7;R11;D15;L4;U9;R15;U8;D4;L10;U2;L13;U9;L5;D13;L5;R1;D3;U11;D5;L13;R3;D4;U3;R5;U14;R4;D9;R3;D11;U13;R7;U10;R1;L9;D15;R6;U12;L4;R13;U7;D10;R13;U1;R4;D15;L10;R11;U9;L1;D6;R15;U10;L10;D6;U10;L8;R5;L14;D6;L2;U2;L9;U2;L7;U9;L2;D8;R6;D6;R7;U2;L13;R13;U3;D13;U14;R9;L4;U15;R1;D2;U15;R12;U12;L12;U9;R8;U3;L7;D3;U2;R7;D4;R6;U13;D7;U7;D8;R13;U10;D16;L12;U8;R6;U10;L14;D12;R2;L9;U8;D7;U6;R7;L14;D6;L10;U11;L3;D6;R2;U8;L2;D10;R16;U14;R10;D13;R3;D8;R2;D16;L12;D13;U6;D2;L4;R14;D16;U12;L1;D9;L7;U1;L3;U7;R7;D3;L11;R9;D5;R10;D12;R1;U8;L8;D5;R12;U8;R11;U3;D5;R9;L11;U14;L14;D5;U12;L9;D2;U6;L7;D6;L2;R2;D11;R13;D10;U4;L13;U5;L6;R3;U16;D13;R1;U2;D4;R3;U10;D16;L1;U1;R16;D6;U7;D3;R4;U2;D4;L8;U9;R8;U15;D4;R11;D4;R12;U1;D2;R4;D17;L4;U1;D1;L3;R16;D12;R6;U3;R4;U2;D8;R5;D4;R1;U5;L12;D1;R12;U3;D13;L13;U11;D8;R3;D14;L10;D1;L4;D4;R9;L10;U6;L8;R7;L2;R1;L4;D8;R14;U8;L13;D12;R17;D2;R5;U12;R8;D16;L6;R14;D9;U7;L5;U14;L10;U5;L4;U7;R17;D1;U6;L12;D10;L2;D17;L14;R3;D6;U11;R14;L11;D8;U1;R1;L14;U17;L15;U3;R5;D3;L12;R10;U17;R7;D2;U16;D10;L12;R5;D10;L14;D17;U4;L9;D11;R5;D4;U5;L16;D16;L5;U10;D8;L14;U11;L17;D1;U10;R9;U17;L8;R11;L15;D9;U18;D3;R4;L7;R2;U5;L17;U2;R11;L11;U7;D17;U11;R5;D14;U7;R17;D10;U18;D15;L7;U2;D3;U14;R6;D2;L5;R16;D8;U2;D16;R9;U12;R4;D14;R10;U1;R1;L8;D15;L8;D16;U9;R11;L17;U18;L2;R4;L1;U9;L13;R1;U17;L12;R3;D15;R3;U3;D8;L6;R4;D13;U2;R1;L10;U9;R2;D16;R9;U15;R10;U4;R6;D10;U4;R11;D1;R10;U17;R4;D9;U17;R7;U11;L10;R10;L10;U8;L18;U14;R16;L12;U18;R1;D4;L9;R18;D12;L17;D12;U4;D1;U16;R17;U3;D13;L14;D9;L19;U13;L2;U9;D7;U7;D4;L10;R6;D11;R7;U10;D13;U18;L14;D7;L4;R11;D8;L8;R9;L2;U11;R13;L13;R11;L16;D10;L6;R3;D8;L19;U18;L14;D15;U10;D11;R19;D14;U2;R7;U6;D15;U10;D4;U10;L9;R15;L8;U7;R4;D5;R18;L3;R11;D11;R2;U1;L6;D18;R15;U2;R10;L18;R1;U19;D15;R18;L18;R4;L7;D18;R19;U11;D3;L3;R11;U16;R3;D4;U2;R10;U9;L18;U6;L5;D7;L17;U3;L5;D6;U19;R14;U3;D1;U16;L16;D12;L11;D16;R15;U5;D14;L3;U18;L8;U10;L10;D10;R5";

#[allow(dead_code)]
const TEST_MOVES: &str = "R4;U4;L3;D1;R4;D1;L5;R2";

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Move {
    dir: Direction,
    dist: i32,
}

impl Move {
    pub fn parse(s: &str) -> Self {
        let (sdir, sdist) = s.split_at(1);
        let dist = sdist.parse::<i32>().unwrap();

        let dir = match sdir {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!(),
        };

        Self { dir, dist }
    }
}

struct BridgeModel {
    knots: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl BridgeModel {
    pub fn new(knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots],
            visited: HashSet::new(),
        }
    }

    pub fn exec_move(&mut self, m: &Move) {
        for _ in 0..m.dist {
            self.move_dir(m.dir);
        }
    }

    fn move_dir(&mut self, dir: Direction) {
        let movement = match dir {
            Direction::Up => (0, 1),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
        };

        self.knots[0].0 += movement.0;
        self.knots[0].1 += movement.1;

        self.visited.insert(self.knots[self.knots.len() - 1]);

        for knot_idx in 1..self.knots.len() {
            self.follow_tail(knot_idx);
        }

        self.visited.insert(self.knots[self.knots.len() - 1]);
    }

    fn follow_tail(&mut self, knot_idx: usize) {
        let ofs = (
            self.knots[knot_idx - 1].0 - self.knots[knot_idx].0,
            self.knots[knot_idx - 1].1 - self.knots[knot_idx].1,
        );

        if ofs.0.abs() <= 1 && ofs.1.abs() <= 1 {
            return;
        }

        let movement = match ofs {
            (-2, o) if o.abs() <= 1 => (-1, o),
            (2, o) if o.abs() <= 1 => (1, o),
            (o, -2) if o.abs() <= 1 => (o, -1),
            (o, 2) if o.abs() <= 1 => (o, 1),
            (a, b) => (a.signum() * (a.abs() - 1), b.signum() * (b.abs() - 1)),
        };

        self.knots[knot_idx].0 += movement.0;
        self.knots[knot_idx].1 += movement.1;
    }
}

pub fn test1() {
    let path = MOVES.split(';').map(Move::parse).collect::<Vec<_>>();
    let mut bridge = BridgeModel::new(2);

    for m in path.iter() {
        bridge.exec_move(m);
    }

    println!("{}", bridge.visited.len());
}

pub fn test2() {
    let path = MOVES.split(';').map(Move::parse).collect::<Vec<_>>();
    let mut bridge = BridgeModel::new(10);

    for m in path.iter() {
        bridge.exec_move(m);
    }

    println!("{}", bridge.visited.len());
}

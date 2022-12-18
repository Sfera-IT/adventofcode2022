use crate::data::day9data;

use std::collections::HashSet;
use std::iter::Iterator;

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
    let path = day9data::MOVES
        .split(';')
        .map(Move::parse)
        .collect::<Vec<_>>();
    let mut bridge = BridgeModel::new(2);

    for m in path.iter() {
        bridge.exec_move(m);
    }

    println!("{}", bridge.visited.len());
}

pub fn test2() {
    let path = day9data::MOVES
        .split(';')
        .map(Move::parse)
        .collect::<Vec<_>>();
    let mut bridge = BridgeModel::new(10);

    for m in path.iter() {
        bridge.exec_move(m);
    }

    println!("{}", bridge.visited.len());
}

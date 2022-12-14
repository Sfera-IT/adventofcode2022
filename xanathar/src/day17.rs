#![allow(clippy::needless_range_loop)]

use crate::data::day17data;

use bidivec::*;
use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Void,
    Rock,
    Wall,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => '.',
                Self::Rock => '@',
                Self::Wall => '#',
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CaveState {
    bottoms: (usize, usize, usize, usize, usize, usize, usize),
    input: usize,
    rocks: u64,
}

struct Cave {
    map: BidiVec<Tile>,
    fallen_rocks: u64,
}

impl Cave {
    pub fn new() -> Self {
        Self { map: BidiVec::new(), fallen_rocks: 0 }
    }

    pub fn state(&self, input_idx: usize) -> CaveState {
        let mut b = [0; 7];

        let y_bottom = self.calc_bottom();

        for x in 0..7 {
            for dy in 0..y_bottom {
                let tile = self.map.get(x, y_bottom - dy).copied().unwrap_or(Tile::Void);
                if tile == Tile::Wall {
                    b[x] = dy;
                    break;
                }
            }
        }

        CaveState {
            bottoms: (b[0], b[1], b[2], b[3], b[4], b[5], b[6]),
            input: input_idx,
            rocks: self.fallen_rocks % 5,
        }
    }

    pub fn calc_bottom(&self) -> usize {
        self.map
            .iter()
            .with_coords()
            .filter(|(_, _, t)| **t == Tile::Wall)
            .map(|(_, y, _)| y + 1)
            .max()
            .unwrap_or(0)
    }

    pub fn push_rock(&mut self) {
        let y = self.calc_bottom() + 3;

        while self.map.height() > y {
            self.map.remove_row(self.map.height() - 1).unwrap();
        }

        while self.map.height() < y {
            self.map.push_row(vec![Tile::Void; 7]).unwrap();
        }

        match self.fallen_rocks % 5 {
            0 => {
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Void,
                    ])
                    .unwrap();
            }
            1 => {
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
            }
            2 => {
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
            }
            3 => {
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
            }
            4 => {
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
                self.map
                    .push_row(vec![
                        Tile::Void,
                        Tile::Void,
                        Tile::Rock,
                        Tile::Rock,
                        Tile::Void,
                        Tile::Void,
                        Tile::Void,
                    ])
                    .unwrap();
            }
            _ => panic!(),
        }

        self.fallen_rocks += 1;
    }

    pub fn try_move_rock(&mut self, dir: u8) {
        let dir = match dir {
            b'<' => -1,
            b'>' => 1,
            _ => panic!(),
        };

        if self.rock_can_move(dir, 0) {
            self.exec_move_rock(dir, 0);
        }
    }

    pub fn exec_move_rock(&mut self, ofs_x: isize, ofs_y: isize) {
        let points = self
            .map
            .iter()
            .with_coords()
            .filter(|(_, _, t)| **t == Tile::Rock)
            .map(|(x, y, _)| (x, y))
            .collect::<Vec<_>>();

        for t in self.map.iter_mut().filter(|t| **t == Tile::Rock) {
            *t = Tile::Void;
        }

        for (x, y) in points {
            let ox = x as isize + ofs_x;
            let oy = y as isize + ofs_y;

            self.map[(ox as usize, oy as usize)] = Tile::Rock;
        }
    }

    pub fn exec_freeze_rock(&mut self) {
        for t in self.map.iter_mut().filter(|t| **t == Tile::Rock) {
            *t = Tile::Wall;
        }
    }

    pub fn rock_can_move(&self, ofs_x: isize, ofs_y: isize) -> bool {
        for (x, y) in self
            .map
            .iter()
            .with_coords()
            .filter(|(_, _, t)| **t == Tile::Rock)
            .map(|(x, y, _)| (x, y))
        {
            let ox = x as isize + ofs_x;
            let oy = y as isize + ofs_y;

            if ox < 0
                || oy < 0
                || ox >= self.map.width() as isize
                || oy >= self.map.height() as isize
            {
                return false;
            }

            if self.map[(ox as usize, oy as usize)] == Tile::Wall {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bottom = self.calc_bottom();

        for y in 1..=self.map.height() {
            let y = self.map.height() - y;

            if y == bottom {
                write!(f, ">")?;
            } else {
                write!(f, "|")?;
            }

            for x in 0..self.map.width() {
                write!(f, "{}", self.map[(x, y)])?;
            }

            if y == bottom {
                writeln!(f, "<")?;
            } else {
                writeln!(f, "|")?;
            }
        }
        writeln!(f, "+-------+")
    }
}

pub fn test1() {
    let mut cave = Cave::new();
    let input = day17data::REAL_INPUT;
    let mut input_idx = 0;

    while cave.fallen_rocks < 2022 {
        cave.push_rock();

        loop {
            let dir = input[input_idx];
            input_idx = (input_idx + 1) % input.len();

            cave.try_move_rock(dir);

            if cave.rock_can_move(0, -1) {
                cave.exec_move_rock(0, -1);
            } else {
                cave.exec_freeze_rock();
                break;
            }
        }
    }

    println!("{}", cave.calc_bottom());
}

pub fn test2() {
    let mut cave = Cave::new();
    let input = day17data::REAL_INPUT;
    let mut input_idx = 0;
    let mut cycle_detection = HashMap::<CaveState, (u64, usize)>::new();
    let mut bottom_to_remove = None;
    let mut result = None;

    while cave.fallen_rocks < 1_000_000_000_000 {
        let state = cave.state(input_idx);

        if bottom_to_remove.is_none() {
            if let Some(prev) = cycle_detection.get(&state) {
                println!();
                println!("FOUND! @{:?}", state);
                println!();
                println!("prev: {:?}", prev);
                println!(" cur: {:?}", (cave.fallen_rocks, cave.calc_bottom()));

                let base = prev.0;
                let bottom_base = prev.1;
                let bottom_diff = cave.calc_bottom() - bottom_base;
                let divider = cave.fallen_rocks - base;
                let rounds = (1_000_000_000_000 - base) / divider;
                let bottoms_to = rounds * (bottom_diff as u64) + bottom_base as u64;
                let remaining_rounds = (1_000_000_000_000 - base) % divider;

                println!("{} rounds remaining", remaining_rounds);
                println!("{} bottoms @ round {}", bottoms_to, 1_000_000_000_000 - remaining_rounds);
                println!("{} <- current bottom", cave.calc_bottom());

                result = Some(bottoms_to);
                bottom_to_remove = Some(cave.calc_bottom());

                cave.fallen_rocks = 1_000_000_000_000 - remaining_rounds;
            } else {
                cycle_detection.insert(state, (cave.fallen_rocks, cave.calc_bottom()));
            }
        }

        if (cave.fallen_rocks % 5) == 0 && input_idx == 0 && cave.fallen_rocks != 0 {
            println!("MODULUS FOUND!! : {}", cave.fallen_rocks);
            return;
        }

        cave.push_rock();

        loop {
            let dir = input[input_idx];
            input_idx = (input_idx + 1) % input.len();

            cave.try_move_rock(dir);

            if cave.rock_can_move(0, -1) {
                cave.exec_move_rock(0, -1);
            } else {
                cave.exec_freeze_rock();
                break;
            }
        }
    }

    let result = (cave.calc_bottom() - bottom_to_remove.unwrap()) as u64 + result.unwrap();

    println!("{}", result);
}

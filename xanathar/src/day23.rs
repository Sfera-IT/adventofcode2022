use crate::utils;
use std::cell::RefCell;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn all(turn: u32) -> &'static [Self] {
        match turn % 4 {
            0 => &[Self::North, Self::South, Self::West, Self::East],
            1 => &[Self::South, Self::West, Self::East, Self::North],
            2 => &[Self::West, Self::East, Self::North, Self::South],
            3 => &[Self::East, Self::North, Self::South, Self::West],
            _ => unreachable!(),
        }
    }

    pub fn as_move_offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
        }
    }

    pub fn as_check_offsets(self) -> &'static [(i32, i32)] {
        match self {
            Self::North => &[(-1, -1), (0, -1), (1, -1)],
            Self::South => &[(-1, 1), (0, 1), (1, 1)],
            Self::East => &[(1, -1), (1, 0), (1, 1)],
            Self::West => &[(-1, -1), (-1, 0), (-1, 1)],
        }
    }

    pub fn surrounding() -> &'static [(i32, i32)] {
        &[(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
    }
}

struct ElfData {
    pos: (i32, i32),
    mov: Option<(i32, i32)>,
}

type Elf = Rc<RefCell<ElfData>>;

struct Map {
    occupied_locations: HashSet<(i32, i32)>,
    elves: Vec<Elf>,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let mut elves = Vec::new();
        let mut occupied_locations = HashSet::new();
        let mut bottom_right = (i32::MIN, i32::MIN);
        let mut top_left = (i32::MAX, i32::MAX);

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    let elf = ElfData { pos: (x as i32, y as i32), mov: None };
                    top_left.0 = cmp::min(top_left.0, elf.pos.0);
                    top_left.1 = cmp::min(top_left.1, elf.pos.1);
                    bottom_right.0 = cmp::max(bottom_right.0, elf.pos.0);
                    bottom_right.1 = cmp::max(bottom_right.1, elf.pos.1);
                    occupied_locations.insert(elf.pos);
                    elves.push(Rc::new(RefCell::new(elf)));
                }
            }
        }

        Self { occupied_locations, elves, top_left, bottom_right }
    }

    fn any_elves_in_tiles(&self, pos: (i32, i32), offsets: &[(i32, i32)]) -> bool {
        for offs in offsets {
            if self.occupied_locations.contains(&(pos.0 + offs.0, pos.1 + offs.1)) {
                return true;
            }
        }
        false
    }

    fn exec_round(&mut self, round_num: u32) -> bool {
        let mut intentions = HashMap::<(i32, i32), Elf>::new();
        let mut any_moved = false;

        for elf_ref in &self.elves {
            let mut elf = elf_ref.borrow_mut();

            elf.mov = None;

            if !self.any_elves_in_tiles(elf.pos, Direction::surrounding()) {
                continue;
            }

            for d in Direction::all(round_num) {
                if !self.any_elves_in_tiles(elf.pos, d.as_check_offsets()) {
                    let offs = d.as_move_offset();
                    let mov = (elf.pos.0 + offs.0, elf.pos.1 + offs.1);
                    if let Some(other) = intentions.get(&mov) {
                        other.borrow_mut().mov = None;
                    } else {
                        elf.mov = Some(mov);
                        intentions.insert(mov, elf_ref.clone());
                    }
                    break;
                }
            }
        }

        self.bottom_right = (i32::MIN, i32::MIN);
        self.top_left = (i32::MAX, i32::MAX);

        for elf in &self.elves {
            let mut elf = elf.borrow_mut();
            if let Some(mov) = elf.mov.take() {
                self.occupied_locations.remove(&elf.pos);
                elf.pos = mov;
                self.occupied_locations.insert(elf.pos);
                any_moved = true;
            }

            elf.mov = None;

            self.top_left.0 = cmp::min(self.top_left.0, elf.pos.0);
            self.top_left.1 = cmp::min(self.top_left.1, elf.pos.1);
            self.bottom_right.0 = cmp::max(self.bottom_right.0, elf.pos.0);
            self.bottom_right.1 = cmp::max(self.bottom_right.1, elf.pos.1);
        }

        any_moved
    }

    fn ground(&self) -> i32 {
        let b = self.bottom_right.0 - self.top_left.0 + 1;
        let h = self.bottom_right.1 - self.top_left.1 + 1;

        (b * h) - (self.elves.len() as i32)
    }
}

pub fn test1() {
    let lines = utils::read_lines_with_empties("./data/day23.txt").collect::<Vec<_>>();
    let mut map = Map::new(&lines);

    for round in 0..10 {
        println!("Round {}", round);
        map.exec_round(round);
        println!("\tground: {}", map.ground());
    }
}

pub fn test2() {
    let lines = utils::read_lines_with_empties("./data/day23.txt").collect::<Vec<_>>();
    let mut map = Map::new(&lines);

    let mut round = 0;
    let result = loop {
        if !map.exec_round(round) {
            break round + 1;
        }
        round += 1;
    };

    println!("Nobody moved in round {}", result);
}

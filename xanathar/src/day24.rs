use crate::utils;
use bidivec::*;
use std::cmp;
use std::collections::{HashSet, VecDeque};
use std::iter::Iterator;

#[derive(Clone, Debug)]
struct Blizzard {
    pos: (usize, usize),
    dir: (isize, isize),
}

struct Map {
    goal: (usize, usize),
    blizzards: Vec<Blizzard>,
    start: (usize, usize),
    width: usize,
    height: usize,
}

impl Map {
    fn new(lines: &[String]) -> Self {
        let mut blizzards = Vec::new();
        let mut start = (0, 0);
        let mut goal = (0, 0);
        let mut width = 0;
        let mut height = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => start = (x, y),
                    'E' => goal = (x, y),
                    '>' => blizzards.push(Blizzard { dir: (1, 0), pos: (x, y) }),
                    '<' => blizzards.push(Blizzard { dir: (-1, 0), pos: (x, y) }),
                    'v' => blizzards.push(Blizzard { dir: (0, 1), pos: (x, y) }),
                    '^' => blizzards.push(Blizzard { dir: (0, -1), pos: (x, y) }),
                    '#' => {
                        width = cmp::max(x + 1, width);
                        height = cmp::max(y + 1, height);
                    }
                    '.' => (),
                    _ => panic!("INVALID CHAR {}", c),
                }
            }
        }

        Self { blizzards, start, goal, width, height }
    }

    fn snap_and_evolve(&mut self) -> BidiArray<bool> {
        let mut snap = bidiarray![true; self.width, self.height];

        for x in 0..self.width {
            snap[(x, 0)] = false;
            snap[(x, self.height - 1)] = false;
        }

        for y in 0..self.height {
            snap[(0, y)] = false;
            snap[(self.width - 1, y)] = false;
        }

        snap[self.start] = true;
        snap[self.goal] = true;

        for b in &mut self.blizzards {
            b.pos.0 =
                (b.pos.0 as isize + b.dir.0 - 1).rem_euclid((self.width - 2) as isize) as usize + 1;
            b.pos.1 = (b.pos.1 as isize + b.dir.1 - 1).rem_euclid((self.height - 2) as isize)
                as usize
                + 1;

            snap[b.pos] = false;
        }

        snap
    }
}

pub fn test1() {
    let mut map = Map::new(&utils::read_lines("./data/day24.txt").collect::<Vec<_>>());
    let mut last_round = 0;
    let mut queue = VecDeque::<(usize, usize, usize)>::new();
    let mut done = HashSet::<(usize, usize, usize)>::new();
    let offsets = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

    queue.push_back((map.start.0, map.start.1, 0));

    let mut snap = map.snap_and_evolve();
    let width = snap.width() as isize;
    let height = snap.height() as isize;

    let round = loop {
        let state = queue.pop_front().unwrap();
        let (x, y, round) = state;

        if last_round != state.2 {
            snap = map.snap_and_evolve();
            last_round = state.2;
        }

        if x == map.goal.0 && y == map.goal.1 {
            break round;
        }

        for offs in &offsets {
            let nx = (x as isize) + offs.0;
            let ny = (y as isize) + offs.1;

            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);

            if snap[(nx, ny)] && !done.contains(&(nx, ny, round + 1)) {
                done.insert((nx, ny, round + 1));
                queue.push_back((nx, ny, round + 1))
            }
        }
    };

    println!("{}", round);
}

pub fn test2() {
    let mut map = Map::new(&utils::read_lines("./data/day24.txt").collect::<Vec<_>>());
    let mut last_round = 0;
    let mut queue = VecDeque::<(usize, usize, usize)>::new();
    let mut done = HashSet::<(usize, usize, usize)>::new();
    let offsets = [(1, 0), (0, 1), (0, 0), (-1, 0), (0, -1)];

    queue.push_back((map.start.0, map.start.1, 0));

    let mut snap = map.snap_and_evolve();
    let width = snap.width() as isize;
    let height = snap.height() as isize;
    let mut sum = 2;

    for (start, goal) in &[(map.start, map.goal), (map.goal, map.start), (map.start, map.goal)] {
        queue.clear();
        done.clear();
        queue.push_back((start.0, start.1, 0));

        sum += loop {
            let state = queue.pop_front().unwrap();
            let (x, y, round) = state;

            if last_round != state.2 {
                snap = map.snap_and_evolve();
                last_round = state.2;
            }

            if x == goal.0 && y == goal.1 {
                break round;
            }

            for offs in &offsets {
                let nx = (x as isize) + offs.0;
                let ny = (y as isize) + offs.1;

                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    continue;
                }

                let (nx, ny) = (nx as usize, ny as usize);

                if snap[(nx, ny)] && !done.contains(&(nx, ny, round + 1)) {
                    done.insert((nx, ny, round + 1));
                    queue.push_back((nx, ny, round + 1))
                }
            }
        };
    }

    println!("{}", sum);
}

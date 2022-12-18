use crate::utils;
use bidivec::*;
use std::cmp;
use std::iter::Iterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Wall,
    Sand,
}

fn parse_line(s: String) -> Option<Vec<(usize, usize)>> {
    let mut v = Vec::new();
    let s = s.replace(" -> ", ">");

    for point_str in s.split('>') {
        let mut tokens = point_str.split(',');
        let x = tokens.next().unwrap().parse::<usize>().unwrap();
        let y = tokens.next().unwrap().parse::<usize>().unwrap();
        v.push((x, y));
    }

    Some(v)
}

fn trace_line(cave: &mut BidiVec<Cell>, points: &[(usize, usize)]) {
    for i in 1..points.len() {
        trace_segment(cave, points[i - 1], points[i]);
    }
}

fn trace_segment(cave: &mut BidiVec<Cell>, start: (usize, usize), end: (usize, usize)) {
    let min_x = cmp::min(start.0, end.0);
    let max_x = cmp::max(start.0, end.0);
    let min_y = cmp::min(start.1, end.1);
    let max_y = cmp::max(start.1, end.1);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            cave[(x, y)] = Cell::Wall;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum DropSand {
    AtRest,
    Diverged,
    Stuck,
}

fn drop_sand(cave: &mut BidiVec<Cell>) -> DropSand {
    let mut x = 500;
    let mut y = 0;

    if cave[(x, y)] == Cell::Sand {
        return DropSand::Stuck;
    }

    loop {
        if x == 0 || x >= 999 {
            panic!("X = {}", x);
        }

        if y >= 249 {
            return DropSand::Diverged;
        }

        if cave[(x, y + 1)] == Cell::Empty {
            y += 1;
        } else if cave[(x - 1, y + 1)] == Cell::Empty {
            x -= 1;
            y += 1;
        } else if cave[(x + 1, y + 1)] == Cell::Empty {
            x += 1;
            y += 1;
        } else {
            cave[(x, y)] = Cell::Sand;
            return DropSand::AtRest;
        }
    }
}

pub fn test1() {
    let mut cave = BidiVec::<Cell>::new();
    cave.resize(1000, 250, Cell::Empty);

    let lines = utils::parse_lines("./data/day14.txt", parse_line);
    for l in lines.iter() {
        trace_line(&mut cave, l);
    }

    let mut iterations = 0;
    while drop_sand(&mut cave) == DropSand::AtRest {
        iterations += 1;
    }

    println!("Iterations: {}", iterations);
}

pub fn test2() {
    let mut cave = BidiVec::<Cell>::new();
    cave.resize(1000, 250, Cell::Empty);

    let lines = utils::parse_lines("./data/day14.txt", parse_line);
    for l in lines.iter() {
        trace_line(&mut cave, l);
    }
    let max_y = lines
        .iter()
        .flat_map(|v| v.iter())
        .map(|p| p.1)
        .max()
        .unwrap();

    trace_line(&mut cave, &[(0, max_y + 2), (999, max_y + 2)]);

    let mut iterations = 0;
    while drop_sand(&mut cave) != DropSand::Stuck {
        iterations += 1;
    }

    println!("Iterations: {}", iterations);
}

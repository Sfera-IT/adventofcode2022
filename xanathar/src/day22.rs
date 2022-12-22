use crate::utils;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;
use std::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Orientation {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Orientation {
    pub fn to_signed_step(self) -> isize {
        match self {
            Self::Right | Self::Down => 1,
            Self::Left | Self::Up => -1,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    pub fn turn_right(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CubeCoord {
    pub x: usize,
    pub y: usize,
    pub face: CubeFace,
}

impl CubeCoord {
    pub fn new(x: usize, y: usize, face: CubeFace) -> Self {
        Self { x, y, face }
    }
}

#[derive(Debug)]
enum Move {
    TurnLeft,
    TurnRight,
    Walk(usize),
}

struct Map {
    tiles: HashMap<(usize, usize), Tile>,
    width: usize,
    height: usize,
    row_ranges: Vec<Range<usize>>,
    col_ranges: Vec<Range<usize>>,
    cube_size: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct CubeFace(u8);

impl Map {
    pub fn next_flat(&self, x: usize, y: usize, or: Orientation) -> (usize, usize) {
        if !self.tiles.contains_key(&(x, y)) {
            panic!("next_flat out of map: ({}, {})", x, y);
        }

        match or {
            Orientation::Down | Orientation::Up => {
                let range = &self.col_ranges[x as usize];
                let iy = (y - range.start) as isize;
                let iy = (iy + or.to_signed_step()).rem_euclid((range.end - range.start) as isize);
                (x, iy as usize + range.start)
            }
            _ => {
                let range = &self.row_ranges[y as usize];
                let ix = (x - range.start) as isize;
                let ix = (ix + or.to_signed_step()).rem_euclid((range.end - range.start) as isize);
                (ix as usize + range.start, y)
            }
        }
    }

    fn flatten(&self, c: CubeCoord) -> (usize, usize) {
        let origin = match c.face {
            CubeFace(1) => (1, 0),
            CubeFace(2) => (2, 0),
            CubeFace(3) => (1, 1),
            CubeFace(4) => (0, 2),
            CubeFace(5) => (1, 2),
            CubeFace(6) => (0, 3),
            _ => panic!(),
        };

        (
            c.x + origin.0 * self.cube_size,
            c.y + origin.1 * self.cube_size,
        )
    }

    #[allow(dead_code)]
    pub fn next_cubic_test(&self, c: CubeCoord, or: Orientation) -> (CubeCoord, Orientation) {
        let last = self.cube_size - 1;

        match (or, c.x, c.y) {
            (Orientation::Left, 0, _) => (),
            (Orientation::Up, _, 0) => (),
            (Orientation::Down, _, y) if y == last => (),
            (Orientation::Right, x, _) if x == last => (),
            (Orientation::Up, x, y) => return (CubeCoord::new(x, y - 1, c.face), Orientation::Up),
            (Orientation::Down, x, y) => {
                return (CubeCoord::new(x, y + 1, c.face), Orientation::Down)
            }
            (Orientation::Right, x, y) => {
                return (CubeCoord::new(x + 1, y, c.face), Orientation::Right)
            }
            (Orientation::Left, x, y) => {
                return (CubeCoord::new(x - 1, y, c.face), Orientation::Left)
            }
        }

        let flip = |c: usize| last - c;

        match (c.face, or) {
            // face 1
            (CubeFace(1), Orientation::Right) => (
                CubeCoord::new(last, flip(c.y), CubeFace(6)),
                Orientation::Left,
            ),
            (CubeFace(1), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(4)), Orientation::Down)
            }
            (CubeFace(1), Orientation::Left) => {
                (CubeCoord::new(c.y, 0, CubeFace(3)), Orientation::Down)
            }
            (CubeFace(1), Orientation::Up) => {
                (CubeCoord::new(flip(c.x), 0, CubeFace(2)), Orientation::Down)
            }
            // face 2
            (CubeFace(2), Orientation::Right) => {
                (CubeCoord::new(0, c.y, CubeFace(3)), Orientation::Right)
            }
            (CubeFace(2), Orientation::Down) => (
                CubeCoord::new(flip(c.x), last, CubeFace(5)),
                Orientation::Up,
            ),
            (CubeFace(2), Orientation::Left) => (
                CubeCoord::new(flip(c.y), last, CubeFace(6)),
                Orientation::Up,
            ),
            (CubeFace(2), Orientation::Up) => {
                (CubeCoord::new(flip(c.x), 0, CubeFace(1)), Orientation::Down)
            }
            // face 3
            (CubeFace(3), Orientation::Right) => {
                (CubeCoord::new(0, c.y, CubeFace(4)), Orientation::Right)
            }
            (CubeFace(3), Orientation::Down) => (
                CubeCoord::new(0, flip(c.x), CubeFace(5)),
                Orientation::Right,
            ),
            (CubeFace(3), Orientation::Left) => {
                (CubeCoord::new(last, c.y, CubeFace(2)), Orientation::Left)
            }
            (CubeFace(3), Orientation::Up) => {
                (CubeCoord::new(0, c.x, CubeFace(1)), Orientation::Right)
            }
            // face 4
            (CubeFace(4), Orientation::Right) => {
                (CubeCoord::new(flip(c.y), 0, CubeFace(6)), Orientation::Down)
            }
            (CubeFace(4), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(5)), Orientation::Down)
            }
            (CubeFace(4), Orientation::Left) => {
                (CubeCoord::new(last, c.y, CubeFace(3)), Orientation::Left)
            }
            (CubeFace(4), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(1)), Orientation::Up)
            }
            // face 5
            (CubeFace(5), Orientation::Right) => {
                (CubeCoord::new(0, c.y, CubeFace(6)), Orientation::Right)
            }
            (CubeFace(5), Orientation::Down) => (
                CubeCoord::new(flip(c.x), last, CubeFace(2)),
                Orientation::Up,
            ),
            (CubeFace(5), Orientation::Left) => (
                CubeCoord::new(flip(c.y), last, CubeFace(3)),
                Orientation::Up,
            ),
            (CubeFace(5), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(4)), Orientation::Up)
            }
            // face 6
            (CubeFace(6), Orientation::Right) => (
                CubeCoord::new(last, flip(c.y), CubeFace(1)),
                Orientation::Left,
            ),
            (CubeFace(6), Orientation::Down) => (
                CubeCoord::new(0, flip(c.x), CubeFace(2)),
                Orientation::Right,
            ),
            (CubeFace(6), Orientation::Left) => {
                (CubeCoord::new(last, c.y, CubeFace(5)), Orientation::Left)
            }
            (CubeFace(6), Orientation::Up) => (
                CubeCoord::new(last, flip(c.x), CubeFace(4)),
                Orientation::Left,
            ),
            _ => panic!(),
        }
    }

    pub fn next_cubic(&self, c: CubeCoord, or: Orientation) -> (CubeCoord, Orientation) {
        let last = self.cube_size - 1;

        match (or, c.x, c.y) {
            (Orientation::Left, 0, _) => (),
            (Orientation::Up, _, 0) => (),
            (Orientation::Down, _, y) if y == last => (),
            (Orientation::Right, x, _) if x == last => (),
            (Orientation::Up, x, y) => return (CubeCoord::new(x, y - 1, c.face), Orientation::Up),
            (Orientation::Down, x, y) => {
                return (CubeCoord::new(x, y + 1, c.face), Orientation::Down)
            }
            (Orientation::Right, x, y) => {
                return (CubeCoord::new(x + 1, y, c.face), Orientation::Right)
            }
            (Orientation::Left, x, y) => {
                return (CubeCoord::new(x - 1, y, c.face), Orientation::Left)
            }
        }

        let flip = |c: usize| last - c;

        let (new_c, new_or) = match (c.face, or) {
            // face 1
            (CubeFace(1), Orientation::Right) => {
                (CubeCoord::new(0, c.y, CubeFace(2)), Orientation::Right)
            }
            (CubeFace(1), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(3)), Orientation::Down)
            }
            (CubeFace(1), Orientation::Left) => (
                CubeCoord::new(0, flip(c.y), CubeFace(4)),
                Orientation::Right,
            ),
            (CubeFace(1), Orientation::Up) => {
                (CubeCoord::new(0, c.x, CubeFace(6)), Orientation::Right)
            }
            // face 2
            (CubeFace(2), Orientation::Right) => (
                CubeCoord::new(last, flip(c.y), CubeFace(5)),
                Orientation::Left,
            ),
            (CubeFace(2), Orientation::Down) => {
                (CubeCoord::new(last, c.x, CubeFace(3)), Orientation::Left)
            }
            (CubeFace(2), Orientation::Left) => {
                (CubeCoord::new(last, c.y, CubeFace(1)), Orientation::Left)
            }
            (CubeFace(2), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(6)), Orientation::Up)
            }
            // face 3
            (CubeFace(3), Orientation::Right) => {
                (CubeCoord::new(c.y, last, CubeFace(2)), Orientation::Up)
            }
            (CubeFace(3), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(5)), Orientation::Down)
            }
            (CubeFace(3), Orientation::Left) => {
                (CubeCoord::new(c.y, 0, CubeFace(4)), Orientation::Down)
            }
            (CubeFace(3), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(1)), Orientation::Up)
            }
            // face 4
            (CubeFace(4), Orientation::Right) => {
                (CubeCoord::new(0, c.y, CubeFace(5)), Orientation::Right)
            }
            (CubeFace(4), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(6)), Orientation::Down)
            }
            (CubeFace(4), Orientation::Left) => (
                CubeCoord::new(0, flip(c.y), CubeFace(1)),
                Orientation::Right,
            ),
            (CubeFace(4), Orientation::Up) => {
                (CubeCoord::new(0, c.x, CubeFace(3)), Orientation::Right)
            }
            // face 5
            (CubeFace(5), Orientation::Right) => (
                CubeCoord::new(last, flip(c.y), CubeFace(2)),
                Orientation::Left,
            ),
            (CubeFace(5), Orientation::Down) => {
                (CubeCoord::new(last, c.x, CubeFace(6)), Orientation::Left)
            }
            (CubeFace(5), Orientation::Left) => {
                (CubeCoord::new(last, c.y, CubeFace(4)), Orientation::Left)
            }
            (CubeFace(5), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(3)), Orientation::Up)
            }
            // face 6
            (CubeFace(6), Orientation::Right) => {
                (CubeCoord::new(c.y, last, CubeFace(5)), Orientation::Up)
            }
            (CubeFace(6), Orientation::Down) => {
                (CubeCoord::new(c.x, 0, CubeFace(2)), Orientation::Down)
            }
            (CubeFace(6), Orientation::Left) => {
                (CubeCoord::new(c.y, 0, CubeFace(1)), Orientation::Down)
            }
            (CubeFace(6), Orientation::Up) => {
                (CubeCoord::new(c.x, last, CubeFace(4)), Orientation::Up)
            }
            _ => panic!(),
        };

        (new_c, new_or)
    }
}

#[cfg(test)]
fn load_test_map() -> Map {
    let cube_map = vec![
        "    ########",
        "    ########",
        "    ########",
        "    ########",
        "    ####",
        "    ####",
        "    ####",
        "    ####",
        "########",
        "########",
        "########",
        "########",
        "####",
        "####",
        "####",
        "####",
        "",
        "5",
    ];

    parse_input(cube_map.iter().map(|s| s.to_string())).0
}

#[test]
fn test_cubic_next_and_back() {
    let map = load_test_map();

    println!("{:?}\n\n", map);

    for face in 1..=6 {
        let face = CubeFace(face);
        for or in [
            Orientation::Right,
            Orientation::Left,
            Orientation::Up,
            Orientation::Down,
        ]
        .iter()
        .copied()
        {
            let (x, y) = match or {
                Orientation::Right => (3, 2),
                Orientation::Left => (0, 2),
                Orientation::Up => (2, 0),
                Orientation::Down => (2, 3),
            };

            println!();
            println!("Testing ({}, {}) face {:?}, {:?}...", x, y, face, or);

            let c = CubeCoord::new(x, y, face);

            let (nc, no) = map.next_cubic(c, or);
            let (rc, ro) = map.next_cubic(nc, no.turn_left().turn_left());

            if rc.x != c.x || rc.y != c.y || rc.face != c.face || ro.turn_left().turn_left() != or {
                panic!("Move across face {:?} towards {:?} failed from ({:?}, {:?}) to ({:?}, {:?}) then back to ({:?}, {:?})",
                    face, or,
                    c, or,
                    nc, no,
                    rc, ro,
                );
            }
        }
    }
}

#[test]
fn test_cubic_next_loop() {
    let map = load_test_map();

    println!("{:?}\n\n", map);

    for face in 1..=6 {
        let face = CubeFace(face);
        for or in [
            Orientation::Right,
            Orientation::Left,
            Orientation::Up,
            Orientation::Down,
        ]
        .iter()
        .copied()
        {
            let (x, y) = match or {
                Orientation::Right => (3, 2),
                Orientation::Left => (0, 2),
                Orientation::Up => (2, 0),
                Orientation::Down => (2, 3),
            };

            println!();
            println!("Testing ({}, {}) face {:?}, {:?}...", x, y, face, or);

            let c = CubeCoord::new(x, y, face);

            let mut nc = c.clone();
            let mut no = or.clone();

            for _ in 1..=16 {
                let next = map.next_cubic(nc, no);
                nc = next.0;
                no = next.1;
            }

            if nc.x != c.x || nc.y != c.y || nc.face != c.face || no != or {
                panic!(
                    "Looping face {:?} towards {:?} failed from ({:?}, {:?}) to ({:?}, {:?}) ",
                    face, or, c, or, nc, no,
                );
            }
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles.get(&(x, y)) {
                    None => write!(f, " ")?,
                    Some(Tile::Floor) => write!(f, ".")?,
                    Some(Tile::Wall) => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }

        write!(
            f,
            "width: {}, height: {}, row_ranges: {:?}, col_ranges: {:?}, cube_size: {}",
            self.width, self.height, self.row_ranges, self.col_ranges, self.cube_size
        )
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = Tile;
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.tiles.get(&index).unwrap()
    }
}

impl std::ops::Index<CubeCoord> for Map {
    type Output = Tile;
    #[inline(always)]
    fn index(&self, index: CubeCoord) -> &Self::Output {
        self.tiles.get(&self.flatten(index)).unwrap()
    }
}

fn load_map_line(map: &mut Map, line: String) {
    map.width = cmp::max(map.width, line.len());

    for (x, c) in line.chars().enumerate() {
        let tile = match c {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            ' ' => continue,
            c => panic!("Unexpected char {}", c),
        };

        map.tiles.insert((x, map.height), tile);
    }

    map.height += 1;
}

fn calc_map_extent<F>(map: &Map, range_max: usize, projection: F) -> Range<usize>
where
    F: Fn(usize) -> (usize, usize),
{
    let mut min = None;
    let mut max = None;

    for coord in 0..range_max {
        if map.tiles.contains_key(&projection(coord)) {
            min = Some(coord);
            break;
        }
    }

    let min = match min {
        Some(v) => v,
        None => return 0..0,
    };

    for coord in min..range_max {
        if !map.tiles.contains_key(&projection(coord)) {
            max = Some(coord);
            break;
        }
    }

    let max = match max {
        Some(v) => v,
        None => range_max,
    };

    min..max
}

fn finish_load_map(map: &mut Map) {
    map.row_ranges = vec![0..0; map.height];
    map.col_ranges = vec![0..0; map.width];

    for y in 0..map.height {
        map.row_ranges[y] = calc_map_extent(map, map.width, |x| (x, y));
    }

    for x in 0..map.width {
        map.col_ranges[x] = calc_map_extent(map, map.height, |y| (x, y));
    }

    map.cube_size = map.row_ranges[0].start;
}

fn load_move_line(moves: &mut Vec<Move>, line: String) {
    let mut current_num = 0;

    for c in line.chars() {
        if c.is_alphabetic() {
            if current_num != 0 {
                moves.push(Move::Walk(current_num));
                current_num = 0;
            }
            match c {
                'R' => moves.push(Move::TurnRight),
                'L' => moves.push(Move::TurnLeft),
                c => panic!("Invalid char in movement: {}", c),
            }
        } else if c.is_ascii_digit() {
            current_num = 10 * current_num + (((c as u8) - b'0') as usize);
        } else {
            panic!("Invalid char in movement: {}", c);
        }
    }

    if current_num != 0 {
        moves.push(Move::Walk(current_num));
    }
}

fn parse_input(mut lines: impl Iterator<Item = String>) -> (Map, Vec<Move>) {
    let mut map = Map {
        tiles: HashMap::new(),
        height: 0,
        width: 0,
        col_ranges: Vec::new(),
        row_ranges: Vec::new(),
        cube_size: 0,
    };

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        load_map_line(&mut map, line);
    }

    finish_load_map(&mut map);

    let mut moves = Vec::new();

    for line in lines {
        load_move_line(&mut moves, line);
    }

    (map, moves)
}

pub fn test1() {
    let (map, moves) = parse_input(utils::read_lines_with_empties("./data/day22.txt"));

    let (mut x, mut y, mut o) = (map.row_ranges[0].start, 0, Orientation::Right);
    println!("{},{} facing {:?}", x, y, o);

    for mov in moves {
        match mov {
            Move::TurnLeft => o = o.turn_left(),
            Move::TurnRight => o = o.turn_right(),
            Move::Walk(steps) => {
                for _ in 0..steps {
                    let next_pos = map.next_flat(x, y, o);

                    if map[next_pos] == Tile::Floor {
                        x = next_pos.0;
                        y = next_pos.1;
                    }
                }
            }
        }
    }

    let password = 1000 * (y + 1) + 4 * (x + 1) + (o as usize);

    println!("passwd: {}", password);
}

pub fn test2() {
    let (map, moves) = parse_input(utils::read_lines_with_empties("./data/day22.txt"));
    let (mut c, mut o) = (CubeCoord::new(0, 0, CubeFace(1)), Orientation::Right);

    for mov in moves {
        match mov {
            Move::TurnLeft => o = o.turn_left(),
            Move::TurnRight => o = o.turn_right(),
            Move::Walk(steps) => {
                for _ in 0..steps {
                    let (next_c, next_o) = map.next_cubic(c, o);

                    if map[next_c] == Tile::Floor {
                        c = next_c;
                        o = next_o;
                    }
                }
            }
        }
    }

    let (x, y) = map.flatten(c);

    let password = 1000 * (y + 1) + 4 * (x + 1) + (o as usize);

    println!("passwd: {} / {}, {}, {:?}", password, x, y, o);
}

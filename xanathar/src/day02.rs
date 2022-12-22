use crate::utils;
use std::iter::Iterator;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Round {
    play: Shape,
    resp: Shape,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RpsResult {
    Loss,
    Draw,
    Win,
}

impl Shape {
    pub fn from_byte(b: u8) -> Self {
        match b {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("Unexpected char '{}'", b as char),
        }
    }

    pub fn from_byte_and_play(b: u8, play: Self) -> Self {
        match (b, play) {
            (b'Y', _) => play,
            (b'Z', Self::Rock) => Self::Paper,
            (b'Z', Self::Paper) => Self::Scissors,
            (b'Z', Self::Scissors) => Self::Rock,
            (b'X', Self::Rock) => Self::Scissors,
            (b'X', Self::Paper) => Self::Rock,
            (b'X', Self::Scissors) => Self::Paper,
            (_, _) => panic!(),
        }
    }

    pub fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn fight(self, other: Self) -> RpsResult {
        match (self, other) {
            (a, b) if a == b => RpsResult::Draw,
            (Self::Rock, Self::Scissors) => RpsResult::Win,
            (Self::Paper, Self::Rock) => RpsResult::Win,
            (Self::Scissors, Self::Paper) => RpsResult::Win,
            _ => RpsResult::Loss,
        }
    }
}

impl RpsResult {
    pub fn score(self) -> u32 {
        match self {
            Self::Draw => 3,
            Self::Win => 6,
            Self::Loss => 0,
        }
    }
}

impl Round {
    fn parse(s: String) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            return None;
        }

        Some(Round { play: Shape::from_byte(bytes[0]), resp: Shape::from_byte(bytes[2]) })
    }

    fn parse2(s: String) -> Option<Self> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            return None;
        }

        let play = Shape::from_byte(bytes[0]);

        Some(Round { play, resp: Shape::from_byte_and_play(bytes[2], play) })
    }

    pub fn score(&self) -> u32 {
        self.resp.score() + self.resp.fight(self.play).score()
    }
}

pub fn test1() {
    let data = utils::parse_lines("./data/day2.txt", Round::parse);

    println!("Score: {}", data.iter().map(|r| r.score()).sum::<u32>());
}

pub fn test2() {
    let data = utils::parse_lines("./data/day2.txt", Round::parse2);

    println!("{:?}", data);

    println!("Score: {}", data.iter().map(|r| r.score()).sum::<u32>());
}

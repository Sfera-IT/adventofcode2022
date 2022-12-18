use crate::utils;
use crate::utils::ansi_colors;
use std::iter::Iterator;

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Add(i32),
    Nop,
}

impl Opcode {
    pub fn parse(s: String) -> Option<Self> {
        let s = s.split(' ').collect::<Vec<_>>();

        if s[0] == "noop" {
            Some(Self::Nop)
        } else if s[0] == "addx" {
            Some(Self::Add(s[1].parse::<i32>().unwrap()))
        } else {
            panic!()
        }
    }

    pub fn decode(self) -> Vec<ExecPlan> {
        match self {
            Opcode::Add(v) => vec![ExecPlan::DecodeAdd(v), ExecPlan::Add(v)],
            Opcode::Nop => vec![ExecPlan::Nop],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ExecPlan {
    DecodeAdd(i32),
    Add(i32),
    Nop,
}

pub fn test1() {
    let prog = utils::parse_lines("./data/day10.txt", Opcode::parse)
        .iter()
        .flat_map(|o| o.decode())
        .collect::<Vec<_>>();

    let mut reg_x: i32 = 1;
    let mut strength = 0;

    for (clock, op) in prog.iter().enumerate() {
        let clock = clock + 1;

        if ((clock + 20) % 40) == 0 {
            strength += reg_x * (clock as i32);
            println!("{} * {} = {}", clock, reg_x, reg_x * (clock as i32));
        }

        if let ExecPlan::Add(v) = op {
            reg_x += v;
        }
    }

    println!("STRENGTH: {}", strength);
}

pub fn test2() {
    let prog = utils::parse_lines("./data/day10.txt", Opcode::parse)
        .iter()
        .flat_map(|o| o.decode())
        .collect::<Vec<_>>();

    let mut reg_x: i32 = 1;

    for (crt_clock, op) in prog.iter().enumerate() {
        let row_clock = (crt_clock as i32) % 40;

        if row_clock == 0 {
            println!();
        }

        if (row_clock - reg_x).abs() <= 1 {
            print!("{}#{}", ansi_colors::BRIGHT_WHITE, ansi_colors::DEFAULT);
        } else {
            print!("{}.{}", ansi_colors::DARK_GRAY, ansi_colors::DEFAULT);
        }

        if let ExecPlan::Add(v) = op {
            reg_x += v;
        }
    }

    println!();
}

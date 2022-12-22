use crate::utils;
use std::collections::VecDeque;
use std::fmt;
use std::iter::Iterator;

#[derive(Debug)]
pub enum Line {
    Ls,
    CdPop,
    Cd(String),
    Dir(String),
    File(String, u64),
}

pub struct FsEntry {
    entries: Vec<FsEntry>,
    name: String,
    own_size: u64,
    dir: bool,
}

impl FsEntry {
    pub fn size(&self) -> u64 {
        self.own_size + self.entries.iter().map(|e| e.size()).sum::<u64>()
    }

    pub fn parse_dir_contents(lines: &mut VecDeque<Line>) -> Vec<FsEntry> {
        let mut result = Vec::new();

        while let Some(line) = lines.front() {
            match line {
                Line::Ls | Line::Dir(_) => {
                    lines.pop_front();
                }
                Line::CdPop => {
                    lines.pop_front();
                    break;
                }
                Line::Cd(_) => {
                    result.push(Self::build_dir(lines));
                }
                Line::File(name, size) => {
                    let name = name.clone();
                    let size = *size;
                    lines.pop_front();
                    result.push(FsEntry {
                        entries: Vec::new(),
                        name: name.clone(),
                        own_size: size,
                        dir: false,
                    });
                }
            }
        }

        result
    }

    pub fn build_dir(lines: &mut VecDeque<Line>) -> Self {
        let name = if let Line::Cd(name) = lines.pop_front().unwrap() {
            println!("DEBUG> build_dir {}", name);

            name
        } else {
            panic!("wtf");
        };

        Self { entries: Self::parse_dir_contents(lines), name, own_size: 0, dir: true }
    }

    pub fn display(&self, identation: u32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ident = (0..identation).map(|_| "  ").collect::<String>();

        if self.dir {
            writeln!(f, "{}- {} (dir)", ident, self.name)?;

            for e in self.entries.iter() {
                e.display(identation + 1, f)?;
            }
        } else {
            writeln!(f, "{}- {} (file, size={})", ident, self.name, self.own_size)?;
        }

        Ok(())
    }

    pub fn flatten_sizes(&self, outp: &mut Vec<u64>) {
        outp.push(self.size());
        for e in self.entries.iter() {
            if e.dir {
                e.flatten_sizes(outp);
            }
        }
    }
}

impl fmt::Display for FsEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display(0, f)
    }
}

impl Line {
    pub fn parse(s: String) -> Self {
        if s == "$ ls" {
            Self::Ls
        } else if s.starts_with("$ cd ") {
            let dir = s.chars().skip(5).collect();
            if dir == ".." {
                Self::CdPop
            } else {
                Self::Cd(dir)
            }
        } else if s.starts_with("dir ") {
            Self::Dir(s.chars().skip(4).collect())
        } else {
            let split = s.split(' ').collect::<Vec<&str>>();
            Line::File(split[1].to_string(), split[0].parse::<u64>().unwrap())
        }
    }
}

pub fn test1() {
    let mut lines = utils::parse_lines("./data/day7.txt", |s| Some(Line::parse(s)))
        .drain(..)
        .collect::<VecDeque<Line>>();

    let root = FsEntry::build_dir(&mut lines);

    println!("{}", root);

    let mut sizes = Vec::new();

    root.flatten_sizes(&mut sizes);

    let size: u64 = sizes.iter().filter(|s| **s <= 100000).sum();

    println!("{}", size);
}

pub fn test2() {
    const FREE: u64 = 30000000;
    const TOTAL: u64 = 70000000;

    let mut lines = utils::parse_lines("./data/day7.txt", |s| Some(Line::parse(s)))
        .drain(..)
        .collect::<VecDeque<Line>>();

    let root = FsEntry::build_dir(&mut lines);

    println!("{}", root);

    let freenow = TOTAL - root.size();

    let to_free = FREE - freenow;

    let mut sizes = Vec::new();

    root.flatten_sizes(&mut sizes);

    println!("{:?}", sizes);

    sizes.retain(|e| *e >= to_free);
    sizes.sort();

    println!("{:?}", sizes);
}

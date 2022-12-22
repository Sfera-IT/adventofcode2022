#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

pub mod ansi_colors {
    pub const DEFAULT: &str = "\x1b[0m";
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const GRAY: &str = "\x1b[37m";

    pub const DARK_GRAY: &str = "\x1b[1;30m";
    pub const BRIGHT_RED: &str = "\x1b[1;31m";
    pub const BRIGHT_GREEN: &str = "\x1b[1;32m";
    pub const BRIGHT_YELLOW: &str = "\x1b[1;33m";
    pub const BRIGHT_BLUE: &str = "\x1b[1;34m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[1;35m";
    pub const BRIGHT_CYAN: &str = "\x1b[1;36m";
    pub const BRIGHT_WHITE: &str = "\x1b[1;37m";
}

pub fn read_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("file open failed");
    io::BufReader::new(file).lines().filter_map(|s| match s.as_deref() {
        Ok("") => None,
        Ok(s) => Some(s.to_string()),
        Err(_) => None,
    })
}
pub fn read_lines_with_empties(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("file open failed");
    io::BufReader::new(file).lines().filter_map(|s| match s.as_deref() {
        Ok(s) => Some(s.to_string()),
        Err(_) => None,
    })
}

pub fn parse_lines<T>(filename: &str, parser: impl Fn(String) -> Option<T>) -> Vec<T> {
    let file = File::open(filename).expect("file open failed");
    io::BufReader::new(file)
        .lines()
        .filter_map(|s| match s.as_deref() {
            Ok("") => None,
            Ok(s) => Some(s.to_string()),
            Err(_) => None,
        })
        .filter_map(parser)
        .collect()
}

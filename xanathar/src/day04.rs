use crate::utils;
use std::iter::Iterator;
use std::ops::RangeInclusive;

struct Assignment {
    ass1: RangeInclusive<u32>,
    ass2: RangeInclusive<u32>,
}

impl Assignment {
    pub fn parse(s: String) -> Option<Assignment> {
        let parts = s.split(&['-', ',']).collect::<Vec<&str>>();

        let n1 = parts[0].parse::<u32>().unwrap();
        let n2 = parts[1].parse::<u32>().unwrap();
        let n3 = parts[2].parse::<u32>().unwrap();
        let n4 = parts[3].parse::<u32>().unwrap();

        Some(Assignment { ass1: n1..=n2, ass2: n3..=n4 })
    }

    pub fn is_total_contained(&self) -> bool {
        (self.ass1.contains(self.ass2.start()) && self.ass1.contains(self.ass2.end()))
            || (self.ass2.contains(self.ass1.start()) && self.ass2.contains(self.ass1.end()))
    }

    pub fn is_any_contained(&self) -> bool {
        (self.ass1.contains(self.ass2.start()) || self.ass1.contains(self.ass2.end()))
            || (self.ass2.contains(self.ass1.start()) || self.ass2.contains(self.ass1.end()))
    }
}

pub fn test1() {
    let ass = utils::parse_lines("./data/day4.txt", Assignment::parse);

    println!("Count: {}", ass.iter().filter(|a| a.is_total_contained()).count());
}

pub fn test2() {
    let ass = utils::parse_lines("./data/day4.txt", Assignment::parse);

    println!("Count: {}", ass.iter().filter(|a| a.is_any_contained()).count());
}

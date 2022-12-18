use crate::utils;
use std::collections::HashSet;
use std::iter::Iterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Item(u8);

impl Item {
    pub fn to_priority(self) -> u32 {
        if self.0 >= b'a' && self.0 <= b'z' {
            (self.0 - b'a' + 1) as u32
        } else if self.0 >= b'A' && self.0 <= b'Z' {
            (self.0 - b'A' + 27) as u32
        } else {
            panic!("Invalid item: '{}'", self.0 as char);
        }
    }
}

type Compartment = HashSet<Item>;

struct Rucksack {
    comp1: Compartment,
    comp2: Compartment,
    all: Compartment,
}

impl Rucksack {
    pub fn parse_compartment(b: &[u8]) -> Compartment {
        b.iter().map(|bb| Item(*bb)).collect()
    }

    pub fn parse(s: String) -> Option<Self> {
        let b = s.as_bytes();

        let l = b.len() / 2;
        let comp1 = Self::parse_compartment(&b[0..l]);
        let comp2 = Self::parse_compartment(&b[l..b.len()]);

        let all = comp1.union(&comp2).copied().collect();

        Some(Rucksack { comp1, comp2, all })
    }

    pub fn common_items_pri(&self) -> u32 {
        self.comp1
            .intersection(&self.comp2)
            .into_iter()
            .map(|i| i.to_priority())
            .sum()
    }

    pub fn intersect_with(&self, o: &Compartment) -> Compartment {
        self.all.intersection(o).copied().collect()
    }
}

pub fn test1() {
    let sacks = utils::parse_lines("./data/day3.txt", Rucksack::parse);

    let sum: u32 = sacks.iter().map(|s| s.common_items_pri()).sum();

    println!("Sum: {}", sum);
}

pub fn test2() {
    let sacks = utils::parse_lines("./data/day3.txt", Rucksack::parse);

    let mut sum = 0;

    for i in (0..sacks.len()).step_by(3) {
        let s1 = &sacks[i];
        let s2 = &sacks[i + 1];
        let s3 = &sacks[i + 2];

        let c = s1.intersect_with(&s2.all);
        let c = s3.intersect_with(&c);

        if c.len() != 1 {
            panic!();
        }

        let p = c.iter().next().unwrap().to_priority();
        println!("..{}", p);
        sum += p;
    }

    println!("Sum: {}", sum);
}

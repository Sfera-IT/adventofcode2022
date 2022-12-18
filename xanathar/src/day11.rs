use crate::data::day11data;

use std::collections::{BinaryHeap, VecDeque};
use std::iter::Iterator;

#[derive(Clone)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    modulus: u64,
    on_true: usize,
    on_false: usize,
    inspections: u64,
}

impl Monkey {
    pub fn new(
        items: &[u64],
        operation: fn(u64) -> u64,
        modulus: u64,
        on_true: usize,
        on_false: usize,
    ) -> Self {
        Self {
            items: items.to_vec().into(),
            inspections: 0,
            operation,
            modulus,
            on_false,
            on_true,
        }
    }

    pub fn inspect1(&mut self) -> Option<(u64, usize)> {
        let Some(worry) = self.items.pop_front() else { return None };
        let worry = (self.operation)(worry) / 3;
        self.inspections += 1;
        Some((
            worry,
            if (worry % self.modulus) == 0 {
                self.on_true
            } else {
                self.on_false
            },
        ))
    }

    pub fn inspect2(&mut self, common_divisor: u64) -> Option<(u64, usize)> {
        let Some(worry) = self.items.pop_front() else { return None };
        let worry = (self.operation)(worry) % common_divisor;
        self.inspections += 1;
        Some((
            worry,
            if (worry % self.modulus) == 0 {
                self.on_true
            } else {
                self.on_false
            },
        ))
    }
}

fn round1(monkeys: &mut [Monkey]) {
    let len = monkeys.len();

    for i in 0..len {
        while let Some((item, to_monkey)) = monkeys[i].inspect1() {
            monkeys[to_monkey].items.push_back(item);
        }
    }
}

fn round2(monkeys: &mut [Monkey], common_divisor: u64) {
    let len = monkeys.len();

    for i in 0..len {
        while let Some((item, to_monkey)) = monkeys[i].inspect2(common_divisor) {
            monkeys[to_monkey].items.push_back(item);
        }
    }
}

pub fn test1() {
    let mut monkeys = day11data::monkeys();

    for _ in 0..20 {
        round1(&mut monkeys);
    }

    let mut data: BinaryHeap<_> = monkeys.iter().map(|m| m.inspections).collect();
    let d1 = data.pop().unwrap();
    let d2 = data.pop().unwrap();

    println!("{} * {} = {}", d1, d2, d1 * d2);
}

pub fn test2() {
    let mut monkeys = day11data::monkeys();
    let mut common_divisor = 1u64;

    for m in monkeys.iter() {
        if (common_divisor % m.modulus) != 0 {
            common_divisor *= m.modulus;
        }
    }

    for _ in 0..10000 {
        round2(&mut monkeys, common_divisor);
    }

    let mut data: BinaryHeap<_> = monkeys.iter().map(|m| m.inspections).collect();
    let d1 = data.pop().unwrap();
    let d2 = data.pop().unwrap();

    println!("{} * {} = {}", d1, d2, d1 * d2);
}

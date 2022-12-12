use crate::utils;
use std::iter::Iterator;

pub fn test1() {
    let mut elves = vec![0];

    for line in utils::read_lines_with_empties("../data/day1.txt") {
        let len = elves.len();

        match line.parse::<u32>() {
            Ok(calories) => elves[len - 1] += calories,
            Err(_) => elves.push(0),
        }
    }

    println!("Max: {}", elves.iter().max().unwrap());
}

pub fn test2() {
    let mut elves = vec![0];

    for line in utils::read_lines_with_empties("../data/day1.txt") {
        let len = elves.len();

        match line.parse::<u32>() {
            Ok(calories) => elves[len - 1] += calories,
            Err(_) => elves.push(0),
        }
    }

    elves.sort_by(|a, b| b.cmp(a));

    println!(
        "Top 3: {} + {} + {} = {}",
        elves[0],
        elves[1],
        elves[2],
        elves[0] + elves[1] + elves[2]
    );
}

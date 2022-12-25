use crate::utils;
use std::iter::Iterator;

pub fn decode_snafu(s: &str) -> i64 {
    let mut tot = 0;

    for c in s.chars() {
        let d = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            c => panic!("unexpected char {}", c),
        };

        tot = tot * 5 + d;
    }

    tot
}

pub fn encode_snafu(mut num: i64) -> String {
    let mut base5rev = Vec::new();

    while num != 0 {
        let rem = num % 5;
        base5rev.push(rem);
        num = (num - rem) / 5;
    }

    let mut i = 0;

    loop {
        if i >= base5rev.len() {
            break;
        }

        if base5rev[i] < 3 {
            i += 1;
            continue;
        }

        if i == base5rev.len() - 1 {
            base5rev.push(0);
        }

        if base5rev[i] == 3 {
            base5rev[i + 1] += 1;
            base5rev[i] = -2;
        } else if base5rev[i] == 4 {
            base5rev[i + 1] += 1;
            base5rev[i] = -1;
        } else if base5rev[i] == 5 {
            base5rev[i + 1] += 1;
            base5rev[i] = 0;
        }

        i += 1;
    }

    let mut snafu = String::new();

    for i in (0..base5rev.len()).rev() {
        snafu.push(match base5rev[i] {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            c => panic!("{}", c),
        });
    }

    snafu
}

#[cfg(test)]
fn test_pairs() -> Vec<(i64, &'static str)> {
    vec![
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ]
}

#[test]
fn test_encode() {
    for (num, snafu) in test_pairs().iter() {
        let encoded = encode_snafu(*num);
        assert_eq!(snafu, &encoded);
    }
}

#[test]
fn test_decode() {
    for (num, snafu) in test_pairs().iter() {
        let decoded = decode_snafu(snafu);
        assert_eq!(*num, decoded);
    }
}

pub fn test1() {
    let nums = utils::parse_lines("./data/day25.txt", |s| Some(decode_snafu(&s)));
    let sum: i64 = nums.iter().sum();
    let encoded = encode_snafu(sum);

    println!("Result : {} // {}", sum, encoded);
}

pub fn test2() {}

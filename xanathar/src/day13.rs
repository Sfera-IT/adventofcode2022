use crate::utils;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt;
use std::iter::Iterator;

enum PacketData {
    Num(u32),
    List(Vec<PacketData>),
}

impl PacketData {
    pub fn parse(s: String) -> Option<Self> {
        if s.is_empty() {
            None
        } else {
            Some(Self::parse_list(&mut VecDeque::from_iter(s.chars())))
        }
    }

    pub fn parse_list(s: &mut VecDeque<char>) -> Self {
        let mut data = Vec::new();

        assert_eq!(s.pop_front().unwrap(), '[');

        loop {
            match s.front() {
                Some(c) if c.is_numeric() => {
                    data.push(Self::parse_number(s));
                }
                Some('[') => {
                    data.push(Self::parse_list(s));
                }
                Some(',') => {
                    s.pop_front();
                }
                Some(']') => {
                    s.pop_front();
                    break;
                }
                Some(c) => panic!("Unexpected token '{}'", c),
                None => panic!("Unexpected EOF"),
            }
        }

        Self::List(data)
    }

    pub fn parse_number(s: &mut VecDeque<char>) -> Self {
        let mut result = 0u32;

        while let Some(c) = s.front() {
            let c = *c;
            if c.is_numeric() {
                s.pop_front();
                result = result * 10 + (c as u8 - b'0') as u32;
            } else {
                break;
            }
        }

        Self::Num(result)
    }
}

impl fmt::Display for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(v) => write!(f, "{}", v),
            Self::List(v) => {
                write!(f, "[")?;
                if !v.is_empty() {
                    write!(f, "{}", v[0])?;
                }
                for val in v.iter().skip(1) {
                    write!(f, ", {}", val)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(v1), Self::Num(v2)) => v1.cmp(v2),
            (Self::List(v1), Self::List(v2)) => {
                let min_len = std::cmp::min(v1.len(), v2.len());

                for i in 0..min_len {
                    match v1[i].cmp(&v2[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }

                v1.len().cmp(&v2.len())
            }
            (Self::List(_), Self::Num(n)) => self.cmp(&Self::List(vec![Self::Num(*n)])),
            (Self::Num(n), Self::List(_)) => Self::List(vec![Self::Num(*n)]).cmp(other),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketData {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for PacketData {}

pub fn test1() {
    let packets = utils::parse_lines("./data/day13.txt", PacketData::parse);
    let mut sum = 0;

    for i in (0..packets.len()).step_by(2) {
        let index = i / 2 + 1;
        let res = packets[i].cmp(&packets[i + 1]);

        println!(
            "[{}] Comparing {} with {} ==> {:?}",
            index,
            packets[i],
            packets[i + 1],
            res
        );

        if res != Ordering::Greater {
            sum += index;
        }
    }

    println!("RESULT: {}", sum);
}

pub fn test2() {
    let mut packets = utils::parse_lines("./data/day13.txt", PacketData::parse);
    packets.push(PacketData::parse("[[2]]".into()).unwrap());
    packets.push(PacketData::parse("[[6]]".into()).unwrap());
    packets.sort();

    let div2 = PacketData::parse("[[2]]".into()).unwrap();
    let div6 = PacketData::parse("[[6]]".into()).unwrap();

    let div2idx = packets
        .iter()
        .enumerate()
        .find(|(_, v)| *v == &div2)
        .map(|(i, _)| i)
        .unwrap()
        + 1;
    let div6idx = packets
        .iter()
        .enumerate()
        .find(|(_, v)| *v == &div6)
        .map(|(i, _)| i)
        .unwrap()
        + 1;

    println!("RESULT: {} * {} == {}", div2idx, div6idx, div2idx * div6idx);
}

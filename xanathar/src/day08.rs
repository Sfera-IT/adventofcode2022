use crate::utils;
use bidivec::*;
use std::iter::Iterator;

fn load() -> BidiVec<i32> {
    let lines = utils::read_lines("./data/day8.txt")
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut bv = BidiVec::<i32>::new();

    for line in lines.iter() {
        bv.push_row(line.iter().map(|b| (*b - b'0') as i32))
            .unwrap();
    }

    bv
}

pub fn test1() {
    let map = load();
    let mut count = 0;

    for (x, y, i) in map.iter().with_coords() {
        let hw = map
            .iter()
            .on_column(x)
            .with_coords()
            .any(|(_, yj, j)| yj > y && *j >= *i);
        let he = map
            .iter()
            .on_column(x)
            .with_coords()
            .any(|(_, yj, j)| yj < y && *j >= *i);
        let hs = map
            .iter()
            .on_row(y)
            .with_coords()
            .any(|(xj, _, j)| xj > x && *j >= *i);
        let hn = map
            .iter()
            .on_row(y)
            .with_coords()
            .any(|(xj, _, j)| xj < x && *j >= *i);

        if !hw || !he || !hs || !hn {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn test2() {
    let map = load();
    let mut max_score = 0;

    for (ix, iy, i) in map.iter().with_coords() {
        let mut east_score = 0;
        for x in (0..ix).rev() {
            east_score += 1;
            if map[(x, iy)] >= *i {
                break;
            }
        }

        let mut north_score = 0;
        for y in (0..iy).rev() {
            north_score += 1;
            if map[(ix, y)] >= *i {
                break;
            }
        }

        let mut west_score = 0;
        for x in ix + 1..map.width() {
            west_score += 1;
            if map[(x, iy)] >= *i {
                break;
            }
        }

        let mut south_score = 0;
        for y in iy + 1..map.height() {
            south_score += 1;
            if map[(ix, y)] >= *i {
                break;
            }
        }

        let score = east_score * north_score * south_score * west_score;

        if score > max_score {
            max_score = score;
        }
    }

    println!("Max: {}", max_score);
}

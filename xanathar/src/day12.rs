use crate::data::day12data;

use crate::utils;
use bidivec::pathfinding::*;
use bidivec::*;
use std::iter::Iterator;
use utils::ansi_colors;

fn cost_func(from: u8, to: u8) -> Option<u32> {
    match (from, to) {
        (b'S', b'a') => Some(1),
        (b'S', b'b') => Some(1),
        (b'z', b'E') => Some(1),
        (b'y', b'E') => Some(1),
        (f, t) if (f + 1) >= t && (b'a'..=b'z').contains(&t) => Some(1),
        _ => None,
    }
}

fn result_print(
    map: &BidiSlice<u8>,
    x: usize,
    y: usize,
    resmap: &BidiArray<PathFindDataTile<u32>>,
) {
    if !resmap[(x, y)].in_shortest_path {
        print!("{}", map[(x, y)] as char);
    } else {
        print!(
            "{}{}{}",
            ansi_colors::BRIGHT_RED,
            map[(x, y)] as char,
            ansi_colors::DEFAULT
        );
    }
}

pub fn test1() {
    let map = BidiSlice::new(day12data::MAP, day12data::MAP_WIDTH).unwrap();

    let start = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'S' { Some((x, y)) } else { None })
        .unwrap();

    let dest = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'E' { Some((x, y)) } else { None })
        .unwrap();

    let res = pathfind_to_dest(
        &map,
        start,
        dest,
        BidiNeighbours::Adjacent,
        |from, _, to, _| cost_func(*from, *to),
    )
    .unwrap();

    if let PathFindDataResult::ShortestPathFound(cost) = res.result {
        for y in 0..map.height() {
            for x in 0..map.width() {
                result_print(&map, x, y, &res.tiles);
            }
            println!();
        }

        println!("Path cost: {}", cost);
    } else {
        println!("Path not found 🤷");
    }
}

pub fn test2() {
    let map = BidiSlice::new(day12data::MAP, day12data::MAP_WIDTH).unwrap();

    let dest = map
        .iter()
        .with_coords()
        .find_map(|(x, y, t)| if *t == b'E' { Some((x, y)) } else { None })
        .unwrap();

    let res = pathfind_to_whole(&map, dest, BidiNeighbours::Adjacent, |from, _, to, _| {
        cost_func(*to, *from)
    })
    .unwrap();

    let min = map
        .iter()
        .with_coords()
        .filter_map(|(x, y, t)| {
            if *t == b'S' || *t == b'a' {
                res.tiles[(x, y)].cost
            } else {
                None
            }
        })
        .min();

    println!("Path cost: {:?}", min);
}

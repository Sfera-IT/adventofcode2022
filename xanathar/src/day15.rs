use crate::data::day15data;

use std::cmp;
use std::iter::Iterator;
use std::ops::RangeInclusive;

pub struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    dist: i64,
}

impl Sensor {
    pub fn new(pos: (i64, i64), beacon: (i64, i64)) -> Self {
        Self { pos, beacon, dist: (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs() }
    }

    fn compatible_with_beacon(&self, x: i64, y: i64) -> bool {
        let bdist = (self.pos.0 - x).abs() + (self.pos.1 - y).abs();
        bdist > self.dist
    }

    fn max_x_range(&self) -> RangeInclusive<i64> {
        self.pos.0 - self.dist..=self.pos.0 + self.dist
    }

    fn range_at_line(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let y_dist = (y - self.pos.1).abs();
        let width = self.dist - y_dist;
        if width > 0 {
            Some((self.pos.0 - width)..=(self.pos.0 + width))
        } else {
            None
        }
    }
}

pub fn test1() {
    let sensors = day15data::data();

    let x_range = sensors.iter().fold(0..=0, |r, s| {
        cmp::min(*r.start(), *s.max_x_range().start())
            ..=(cmp::max(*r.end(), *s.max_x_range().end()))
    });

    println!("range: {:?}", x_range);

    const Y: i64 = 2000000;

    let mut count = 0;
    for x in x_range {
        if sensors.iter().any(|s| !s.compatible_with_beacon(x, Y))
            && !sensors.iter().any(|s| s.beacon.0 == x && s.beacon.1 == Y)
        {
            count += 1;
        }
    }

    println!("{}", count);
}

pub fn test2() {
    let sensors = day15data::data();
    let coord_max = 4_000_000;

    for y in 0..=coord_max {
        test2_on(&sensors, coord_max, y);
    }
}

fn test2_on(sensors: &[Sensor], coord_max: i64, y: i64) {
    let mut ranges =
        sensors.iter().filter_map(|s| s.range_at_line(y)).collect::<Vec<RangeInclusive<i64>>>();
    let mut beacons = sensors
        .iter()
        .filter_map(|s| if s.beacon.1 == y { Some(s.beacon.0..=s.beacon.0) } else { None })
        .collect::<Vec<RangeInclusive<i64>>>();
    let mut sensors = sensors
        .iter()
        .filter_map(|s| if s.pos.1 == y { Some(s.pos.0..=s.pos.0) } else { None })
        .collect::<Vec<RangeInclusive<i64>>>();

    ranges.append(&mut beacons);
    ranges.append(&mut sensors);

    if y % 100_000 == 0 {
        println!("{}", y);
    }

    if ranges.is_empty() {
        return;
    }

    ranges.sort_by_key(|r| *r.start());

    for r in &mut ranges {
        let s = cmp::min(coord_max, cmp::max(0, *r.start()));
        let e = cmp::min(coord_max, cmp::max(0, *r.end()));

        *r = s..=e;
    }

    let mut changed = true;
    while changed {
        changed = false;

        for i in 1..ranges.len() {
            if ranges[i - 1].end() + 1 >= *ranges[i].start() {
                ranges[i - 1] =
                    *ranges[i - 1].start()..=cmp::max(*ranges[i].end(), *ranges[i - 1].end());
                ranges.remove(i);
                changed = true;
                break;
            }
        }
    }

    if ranges.len() > 1 {
        println!(
            "Found: {:?} at row {} // res = {}",
            ranges,
            y,
            (ranges[0].end() + 1) * 4_000_000 + y
        );
    }
}

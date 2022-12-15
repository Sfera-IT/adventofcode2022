use std::cmp;
use std::iter::Iterator;
use std::ops::RangeInclusive;

#[allow(dead_code)]
fn data() -> Vec<Sensor> {
    vec![
        Sensor::new((2885528, 2847539), (2966570, 2470834)),
        Sensor::new((2224704, 1992385), (2018927, 2000000)),
        Sensor::new((3829144, 1633329), (2966570, 2470834)),
        Sensor::new((43913, 426799), (152363, 369618)),
        Sensor::new((2257417, 2118161), (2386559, 2090397)),
        Sensor::new((8318, 3994839), (-266803, 2440278)),
        Sensor::new((69961, 586273), (152363, 369618)),
        Sensor::new((3931562, 3361721), (3580400, 3200980)),
        Sensor::new((476279, 3079924), (-266803, 2440278)),
        Sensor::new((2719185, 2361091), (2966570, 2470834)),
        Sensor::new((2533382, 3320911), (2260632, 3415930)),
        Sensor::new((3112735, 3334946), (3580400, 3200980)),
        Sensor::new((1842258, 3998928), (2260632, 3415930)),
        Sensor::new((3712771, 3760832), (3580400, 3200980)),
        Sensor::new((1500246, 2684955), (2018927, 2000000)),
        Sensor::new((3589321, 142859), (4547643, -589891)),
        Sensor::new((1754684, 2330721), (2018927, 2000000)),
        Sensor::new((2476631, 3679883), (2260632, 3415930)),
        Sensor::new((27333, 274008), (152363, 369618)),
        Sensor::new((158732, 2405833), (-266803, 2440278)),
        Sensor::new((2955669, 3976939), (3035522, 4959118)),
        Sensor::new((1744196, 13645), (152363, 369618)),
        Sensor::new((981165, 1363480), (2018927, 2000000)),
        Sensor::new((2612279, 2151377), (2386559, 2090397)),
        Sensor::new((3897, 2076376), (-266803, 2440278)),
        Sensor::new((2108479, 1928318), (2018927, 2000000)),
        Sensor::new((1913043, 3017841), (2260632, 3415930)),
        Sensor::new((2446778, 785075), (2386559, 2090397)),
        Sensor::new((2385258, 2774943), (2386559, 2090397)),
        Sensor::new((3337656, 2916144), (3580400, 3200980)),
        Sensor::new((380595, 66906), (152363, 369618)),
        Sensor::new((1593628, 3408455), (2260632, 3415930)),
    ]
}

#[allow(dead_code)]
fn test_data() -> Vec<Sensor> {
    vec![
        Sensor::new((2, 18), (-2, 15)),
        Sensor::new((9, 16), (10, 16)),
        Sensor::new((13, 2), (15, 3)),
        Sensor::new((12, 14), (10, 16)),
        Sensor::new((10, 20), (10, 16)),
        Sensor::new((14, 17), (10, 16)),
        Sensor::new((8, 7), (2, 10)),
        Sensor::new((2, 0), (2, 10)),
        Sensor::new((0, 11), (2, 10)),
        Sensor::new((20, 14), (25, 17)),
        Sensor::new((17, 20), (21, 22)),
        Sensor::new((16, 7), (15, 3)),
        Sensor::new((14, 3), (15, 3)),
        Sensor::new((20, 1), (15, 3)),
    ]
}

struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    dist: i64,
}

impl Sensor {
    fn new(pos: (i64, i64), beacon: (i64, i64)) -> Self {
        Self {
            pos,
            beacon,
            dist: (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs(),
        }
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
    let sensors = data();

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
    let sensors = data();
    let coord_max = 4_000_000;

    for y in 0..=coord_max {
        test2_on(&sensors, coord_max, y);
    }
}

fn test2_on(sensors: &[Sensor], coord_max: i64, y: i64) {
    let mut ranges = sensors
        .iter()
        .filter_map(|s| s.range_at_line(y))
        .collect::<Vec<RangeInclusive<i64>>>();
    let mut beacons = sensors
        .iter()
        .filter_map(|s| {
            if s.beacon.1 == y {
                Some(s.beacon.0..=s.beacon.0)
            } else {
                None
            }
        })
        .collect::<Vec<RangeInclusive<i64>>>();
    let mut sensors = sensors
        .iter()
        .filter_map(|s| {
            if s.pos.1 == y {
                Some(s.pos.0..=s.pos.0)
            } else {
                None
            }
        })
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

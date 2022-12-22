use crate::utils;
use std::cmp;
use std::collections::HashSet;
use std::iter::Iterator;

type Cube = (i32, i32, i32);

fn parse_cube(line: String) -> Option<Cube> {
    let tokens = line.split(',').collect::<Vec<&str>>();

    Some((
        tokens[0].parse::<i32>().unwrap(),
        tokens[1].parse::<i32>().unwrap(),
        tokens[2].parse::<i32>().unwrap(),
    ))
}

fn bound_box_min(cubes: &[Cube]) -> Cube {
    cubes.iter().fold((i32::MAX, i32::MAX, i32::MAX), |p, q| {
        (cmp::min(p.0, q.0), cmp::min(p.1, q.1), cmp::min(p.2, q.2))
    })
}

fn bound_box_max(cubes: &[Cube]) -> Cube {
    cubes.iter().fold((i32::MIN, i32::MIN, i32::MIN), |p, q| {
        (cmp::max(p.0, q.0), cmp::max(p.1, q.1), cmp::max(p.2, q.2))
    })
}

pub fn test1() {
    let cubes_list = utils::parse_lines("./data/day18.txt", parse_cube);

    let cubes = cubes_list.iter().collect::<HashSet<_>>();

    let mut area = 0;

    for cube in &cubes_list {
        if !cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            area += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            area += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            area += 1;
        }
        if !cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            area += 1;
        }
        if !cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            area += 1;
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            area += 1;
        }
    }

    println!("TOT: {}", area);
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Matter {
    Air,
    Steam,
    Lava,
}

struct Space {
    vec: Vec<Matter>,
    min: Cube,
    max: Cube,
    range: Cube,
}

impl Space {
    pub fn new(min: Cube, max: Cube) -> Self {
        let range = (max.0 - min.0 + 1, max.1 - min.1 + 1, max.2 - min.2 + 1);

        Self { min, max, range, vec: vec![Matter::Air; (range.0 * range.1 * range.2) as usize] }
    }

    fn calc_index(&self, c: Cube) -> Option<usize> {
        if c.0 < self.min.0
            || c.0 > self.max.0
            || c.1 < self.min.1
            || c.1 > self.max.1
            || c.2 < self.min.2
            || c.2 > self.max.2
        {
            return None;
        }

        let c = (c.0 - self.min.0, c.1 - self.min.1, c.2 - self.min.2);

        Some((c.0 + c.1 * self.range.0 + c.2 * self.range.0 * self.range.1) as usize)
    }
}

impl std::ops::Index<Cube> for Space {
    type Output = Matter;
    #[inline(always)]
    fn index(&self, index: Cube) -> &Self::Output {
        if let Some(idx) = self.calc_index(index) {
            &self.vec[idx]
        } else {
            &Matter::Steam
        }
    }
}

impl std::ops::IndexMut<Cube> for Space {
    #[inline(always)]
    fn index_mut(&mut self, index: Cube) -> &mut Self::Output {
        if let Some(idx) = self.calc_index(index) {
            &mut self.vec[idx]
        } else {
            panic!(
                "Invalid cube: {:?} for min:{:?} max:{:?} range:{:?}",
                index, self.min, self.max, self.range
            );
        }
    }
}

pub fn test2() {
    let cubes = utils::parse_lines("./data/day18.txt", parse_cube);

    let min = bound_box_min(&cubes);
    let max = bound_box_max(&cubes);

    let mut space = Space::new(min, max);

    for cube in &cubes {
        space[*cube] = Matter::Lava;
    }

    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            for z in min.2..=max.2 {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
            for z in (min.2..=max.2).rev() {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
        }
    }

    for z in min.2..=max.2 {
        for y in min.1..=max.1 {
            for x in min.0..=max.0 {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
            for x in (min.0..=max.0).rev() {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
        }
    }

    for x in min.0..=max.0 {
        for z in min.1..=max.1 {
            for y in min.1..=max.1 {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
            for y in (min.1..=max.1).rev() {
                if space[(x, y, z)] == Matter::Lava {
                    break;
                }
                space[(x, y, z)] = Matter::Steam;
            }
        }
    }

    let mut flooded = true;

    while flooded {
        flooded = false;

        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                for z in min.2..=max.2 {
                    #[allow(clippy::collapsible_if)]
                    if space[(x, y, z)] == Matter::Air {
                        if space[(x + 1, y, z)] == Matter::Steam
                            || space[(x, y + 1, z)] == Matter::Steam
                            || space[(x, y, z + 1)] == Matter::Steam
                            || space[(x - 1, y, z)] == Matter::Steam
                            || space[(x, y - 1, z)] == Matter::Steam
                            || space[(x, y, z - 1)] == Matter::Steam
                        {
                            space[(x, y, z)] = Matter::Steam;
                            flooded = true;
                        }
                    }
                }
            }
        }
    }

    let mut area = 0;

    for cube in &cubes {
        if space[(cube.0 - 1, cube.1, cube.2)] == Matter::Steam {
            area += 1;
        }
        if space[(cube.0, cube.1 - 1, cube.2)] == Matter::Steam {
            area += 1;
        }
        if space[(cube.0, cube.1, cube.2 - 1)] == Matter::Steam {
            area += 1;
        }
        if space[(cube.0 + 1, cube.1, cube.2)] == Matter::Steam {
            area += 1;
        }
        if space[(cube.0, cube.1 + 1, cube.2)] == Matter::Steam {
            area += 1;
        }
        if space[(cube.0, cube.1, cube.2 + 1)] == Matter::Steam {
            area += 1;
        }
    }

    println!("TOT: {}", area);
}

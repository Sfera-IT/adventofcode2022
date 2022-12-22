use std::cmp;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost: (u32, u32),
    geode_cost: (u32, u32),
}

impl Blueprint {
    pub fn new(
        ore_cost: u32,
        clay_cost: u32,
        obsidian_cost: (u32, u32),
        geode_cost: (u32, u32),
    ) -> Self {
        Self { ore_cost, clay_cost, obsidian_cost, geode_cost }
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct FactoryState {
    robots: Resources,
    resources: Resources,
    time_left: u32,
}

impl FactoryState {
    pub fn new(time: u32) -> Self {
        let mut this =
            Self { robots: Resources::default(), resources: Resources::default(), time_left: time };

        this.robots.ore = 1;

        this
    }

    pub fn next(&self) -> Self {
        let mut new = self.clone();
        new.resources.ore += self.robots.ore;
        new.resources.clay += self.robots.clay;
        new.resources.obsidian += self.robots.obsidian;
        new.resources.geode += self.robots.geode;
        new.time_left -= 1;
        new
    }
}

fn simul(state: FactoryState, bp: &Blueprint, memo: &mut HashMap<FactoryState, u32>) -> u32 {
    let mut geodes = state.resources.geode;

    if state.time_left == 0 {
        return geodes;
    }

    if let Some(g) = memo.get(&state) {
        return *g;
    }

    // try build ore
    if state.resources.ore >= bp.ore_cost {
        let mut new = state.next();
        new.resources.ore -= bp.ore_cost;
        new.robots.ore += 1;
        geodes = cmp::max(geodes, simul(new, bp, memo));
    }
    // try build clay
    if state.resources.ore >= bp.clay_cost {
        let mut new = state.next();
        new.resources.ore -= bp.clay_cost;
        new.robots.clay += 1;
        geodes = cmp::max(geodes, simul(new, bp, memo));
    }
    // try build obsidian
    if state.resources.ore >= bp.obsidian_cost.0 && state.resources.clay >= bp.obsidian_cost.1 {
        let mut new = state.next();
        new.resources.ore -= bp.obsidian_cost.0;
        new.resources.clay -= bp.obsidian_cost.1;
        new.robots.obsidian += 1;
        geodes = cmp::max(geodes, simul(new, bp, memo));
    }
    // try build geode
    if state.resources.ore >= bp.geode_cost.0 && state.resources.obsidian >= bp.geode_cost.1 {
        let mut new = state.next();
        new.resources.ore -= bp.geode_cost.0;
        new.resources.obsidian -= bp.geode_cost.1;
        new.robots.geode += 1;
        geodes = cmp::max(geodes, simul(new, bp, memo));
    }
    // try lazy
    geodes = cmp::max(geodes, simul(state.next(), bp, memo));

    memo.insert(state, geodes);

    geodes
}

fn simul2(state: FactoryState, bp: &Blueprint) -> u32 {
    let mut geodes = state.resources.geode;

    if state.time_left == 0 {
        return geodes;
    }

    // try build ore -- the "10" threshold has been determined experimentally
    if state.resources.ore >= bp.ore_cost && state.time_left > 10 {
        let mut new = state.next();
        new.resources.ore -= bp.ore_cost;
        new.robots.ore += 1;
        geodes = cmp::max(geodes, simul2(new, bp));
    }
    // try build clay -- the "10" threshold has been determined experimentally
    if state.resources.ore >= bp.clay_cost && state.time_left > 10 {
        let mut new = state.next();
        new.resources.ore -= bp.clay_cost;
        new.robots.clay += 1;
        geodes = cmp::max(geodes, simul2(new, bp));
    }
    // always build geode or obsidian if possible
    if state.resources.ore >= bp.geode_cost.0 && state.resources.obsidian >= bp.geode_cost.1 {
        let mut new = state.next();
        new.resources.ore -= bp.geode_cost.0;
        new.resources.obsidian -= bp.geode_cost.1;
        new.robots.geode += 1;
        geodes = cmp::max(geodes, simul2(new, bp));
    } else if state.resources.ore >= bp.obsidian_cost.0
        && state.resources.clay >= bp.obsidian_cost.1
    {
        let mut new = state.next();
        new.resources.ore -= bp.obsidian_cost.0;
        new.resources.clay -= bp.obsidian_cost.1;
        new.robots.obsidian += 1;
        geodes = cmp::max(geodes, simul2(new, bp));
    } else {
        geodes = cmp::max(geodes, simul2(state.next(), bp));
    }

    geodes
}

pub fn test1() {
    let blueprints = crate::data::day19data::data();
    let mut res = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let gmax = simul(FactoryState::new(24), blueprint, &mut HashMap::new());
        println!("gmax for bp {}: {}", i + 1, gmax);
        res += (i + 1) * (gmax as usize);
    }

    println!("Res: {}", res);
}

pub fn test2() {
    let blueprints = crate::data::day19data::data();

    let blueprint1 = blueprints[0].clone();
    let blueprint2 = blueprints[1].clone();
    let blueprint3 = blueprints[2].clone();

    let handle1 = std::thread::spawn(move || {
        let gmax = simul2(FactoryState::new(32), &blueprint1);
        println!("geodes for blueprint 1: {}", gmax);
        gmax
    });

    let handle2 = std::thread::spawn(move || {
        let gmax = simul2(FactoryState::new(32), &blueprint2);
        println!("geodes for blueprint 2: {}", gmax);
        gmax
    });

    let handle3 = std::thread::spawn(move || {
        let gmax = simul2(FactoryState::new(32), &blueprint3);
        println!("geodes for blueprint 3: {}", gmax);
        gmax
    });

    let g1 = handle1.join().unwrap();
    let g2 = handle2.join().unwrap();
    let g3 = handle3.join().unwrap();

    println!("Res: {}", g1 * g2 * g3);
}

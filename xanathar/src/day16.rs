#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]

use crate::data::day16data;

use std::collections::{HashMap, VecDeque};
use std::iter::Iterator;

#[derive(Default, Copy, Clone)]
struct VisitedMask(u64);

impl VisitedMask {
    pub fn set(self, index: usize) -> Self {
        Self(self.0 | 1 << (index as u32))
    }

    pub fn already_visited(&self, index: usize) -> bool {
        (self.0 & 1 << (index as u32)) != 0
    }
}

pub struct Valve {
    label: &'static str,
    index: usize,
    flow: u64,
    out: &'static [usize],
    path_costs: Vec<u32>,
}

impl Valve {
    pub fn new(label: &'static str, index: usize, flow: u64, out: &'static [usize]) -> Self {
        Self {
            label,
            index,
            flow,
            out,
            path_costs: Vec::new(),
        }
    }

    pub fn from_translated(
        index: usize,
        prev_valve: &Valve,
        translated_indices: &[Option<usize>],
        num_valves: usize,
    ) -> Self {
        let flow = prev_valve.flow;
        let out = &[];
        let mut path_costs = vec![0; num_valves];
        let label = prev_valve.label;

        for i in 0..prev_valve.path_costs.len() {
            let Some(translated_index) = translated_indices[i] else { continue; };
            path_costs[translated_index] = prev_valve.path_costs[i];
        }

        Self {
            label,
            index,
            flow,
            out,
            path_costs,
        }
    }
}

fn find_all_path_costs(valve: &Valve, all_valves: &[Valve]) -> Vec<u32> {
    let mut costs = vec![0; all_valves.len()];
    let mut queue = VecDeque::new();

    for i in valve.out.iter() {
        queue.push_back((i, 1));
    }

    while let Some((&index, cost)) = queue.pop_front() {
        if costs[index] != 0 || index == valve.index {
            continue;
        }

        costs[index] = cost;

        for i in all_valves[index].out.iter() {
            queue.push_back((i, cost + 1));
        }
    }

    costs
}

fn optimize_graph(mut valves: Vec<Valve>) -> Vec<Valve> {
    for i in 0..valves.len() {
        let ac = find_all_path_costs(&valves[i], &valves);
        valves[i].path_costs = ac;
    }

    let mut translation_map = vec![None; valves.len()];
    let mut new_indices = Vec::new();

    for i in 0..valves.len() {
        if new_indices.is_empty() || valves[i].flow != 0 {
            translation_map[i] = Some(new_indices.len());
            new_indices.push(i);
        }
    }

    let new_valves = new_indices
        .iter()
        .enumerate()
        .map(|(new_idx, old_idx)| {
            Valve::from_translated(
                new_idx,
                &valves[*old_idx],
                &translation_map,
                new_indices.len(),
            )
        })
        .collect();

    new_valves
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct MemoItem(usize, usize, u32, u32, u64);

impl MemoItem {
    pub fn new_single(index: usize, time_left: u32, visited: VisitedMask) -> Self {
        Self(index, 0, time_left, 0, visited.0)
    }

    pub fn new_double(
        h_index: usize,
        e_index: usize,
        h_time_left: u32,
        e_time_left: u32,
        visited: VisitedMask,
    ) -> Self {
        Self(h_index, e_index, h_time_left, e_time_left, visited.0)
    }

    pub fn new_timeless(h_index: usize, e_index: usize, visited: VisitedMask) -> Self {
        Self(h_index, e_index, 0, 0, visited.0)
    }

    pub fn flipped(&self) -> Self {
        Self(self.1, self.0, self.3, self.2, self.4)
    }
}

fn max_flow_on(
    index: usize,
    valves: &[Valve],
    time_left: u32,
    visited: VisitedMask,
    memo: &mut HashMap<MemoItem, u64>,
) -> u64 {
    let valve = &valves[index];
    if time_left == 0 {
        panic!(
            "Visited {} ({}) with no time_left!",
            valve.label, valve.flow
        );
    }

    let memo_item = MemoItem::new_single(index, time_left, visited);

    if let Some(c) = memo.get(&memo_item) {
        return *c;
    }

    let time_left = if valve.flow > 0 {
        time_left - 1
    } else {
        time_left
    };

    let visited = visited.set(index);

    let sub_flow = valve
        .path_costs
        .iter()
        .enumerate()
        .filter_map(|(next, cost)| {
            if *cost >= time_left || next == index || visited.already_visited(next) {
                None
            } else {
                Some(max_flow_on(next, valves, time_left - cost, visited, memo))
            }
        })
        .max()
        .unwrap_or(0);

    let flow = sub_flow + (time_left as u64) * valve.flow;

    memo.insert(memo_item, flow);

    flow
}

fn max_flow_junglish(
    h_index: usize,
    e_index: usize,
    valves: &[Valve],
    h_time_left: u32,
    e_time_left: u32,
    visited: VisitedMask,
    memo: &mut HashMap<MemoItem, u64>,
    timeless: &mut HashMap<MemoItem, (u32, u32)>,
) -> u64 {
    let h_valve = &valves[h_index];
    let e_valve = &valves[e_index];

    let memo_item = MemoItem::new_double(h_index, e_index, h_time_left, e_time_left, visited);
    let memo_item_timeless = MemoItem::new_timeless(h_index, e_index, visited);

    if let Some(c) = memo.get(&memo_item) {
        return *c;
    }

    if let Some((htl, etl)) = timeless.get(&memo_item_timeless) {
        if *htl >= h_time_left && *etl >= e_time_left {
            return 0;
        }
    }

    timeless.insert(memo_item_timeless.flipped(), (e_time_left, h_time_left));
    timeless.insert(memo_item_timeless, (h_time_left, e_time_left));

    let h_local_flow = if visited.already_visited(h_index) || h_time_left == 0 {
        0
    } else {
        h_valve.flow
    };

    let e_local_flow = if visited.already_visited(e_index) || e_time_left == 0 {
        0
    } else {
        e_valve.flow
    };

    let h_time_left = if h_local_flow > 0 {
        h_time_left.saturating_sub(1)
    } else {
        h_time_left
    };

    let e_time_left = if e_local_flow > 0 {
        e_time_left.saturating_sub(1)
    } else {
        e_time_left
    };

    let visited = visited.set(h_index);
    let visited = visited.set(e_index);

    let mut max_flow = 0;

    for (h_next, h_cost) in h_valve.path_costs.iter().enumerate() {
        if visited.already_visited(h_next) && h_next != h_index {
            continue;
        }

        for (e_next, e_cost) in e_valve.path_costs.iter().enumerate() {
            if e_next == e_index && h_next == h_index {
                continue;
            }

            if e_next == h_next {
                continue;
            }

            if visited.already_visited(e_next) {
                continue;
            }

            let sub_flow = max_flow_junglish(
                h_next,
                e_next,
                valves,
                h_time_left.saturating_sub(*h_cost),
                e_time_left.saturating_sub(*e_cost),
                visited,
                memo,
                timeless,
            );
            max_flow = std::cmp::max(max_flow, sub_flow);
        }
    }

    let flow = max_flow + (h_time_left as u64) * h_local_flow + (e_time_left as u64) * e_local_flow;

    memo.insert(memo_item.flipped(), flow);
    memo.insert(memo_item, flow);

    flow
}

pub fn test1() {
    let valves = optimize_graph(day16data::real_input()); // 1651 / 1775

    let mf = max_flow_on(0, &valves, 30, VisitedMask::default(), &mut HashMap::new());

    println!("MAX: {}", mf);
}

pub fn test2() {
    let valves = optimize_graph(day16data::real_input());

    let mf = max_flow_junglish(
        0,
        0,
        &valves,
        26,
        26,
        VisitedMask::default().set(0),
        &mut HashMap::new(),
        &mut HashMap::new(),
    );

    println!("MAX: {}", mf);
}

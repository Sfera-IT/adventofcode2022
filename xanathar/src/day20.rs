use crate::utils;
use std::iter::Iterator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Number {
    val: i64,
    order: usize,
}

impl Number {
    pub fn new(val: i64, order: usize) -> Self {
        Self { val, order }
    }
}

fn rearrange_with_swaps_mod(data: &mut Vec<Number>, index: usize) {
    let bias = data[index].val.abs() / ((data.len() as i64) - 1);
    let bias = if data[index].val < 0 { -1 - bias } else { bias };

    let val = (data[index].val + bias).rem_euclid(data.len() as i64);

    let sign = val.signum();
    let mut swaps = 0;
    let mut index = index;

    while swaps != val {
        let new_index =
            (((index as isize) + (sign as isize) + (data.len() as isize)) as usize) % data.len();

        data.swap(index, new_index);
        index = new_index;
        swaps += sign;
    }
}

pub fn test1() {
    let raw_data = utils::parse_lines("./data/day20.txt", |s| s.parse::<i64>().ok());
    let mut data = raw_data.iter().enumerate().map(|(i, v)| Number::new(*v, i)).collect::<Vec<_>>();

    for i in 0..data.len() {
        let index_to_swap =
            data.iter().enumerate().find(|(_, item)| item.order == i).map(|(idx, _)| idx).unwrap();
        rearrange_with_swaps_mod(&mut data, index_to_swap);
    }

    let zero_idx =
        data.iter().enumerate().find(|(_, item)| item.val == 0).map(|(idx, _)| idx).unwrap();

    let n1 = data[(1000 + zero_idx) % data.len()].val;
    let n2 = data[(2000 + zero_idx) % data.len()].val;
    let n3 = data[(3000 + zero_idx) % data.len()].val;

    println!("{}+{}+{} = {}", n1, n2, n3, n1 + n2 + n3);
}

pub fn test2() {
    let raw_data = utils::parse_lines("./data/day20.txt", |s| s.parse::<i64>().ok());
    let mut data = raw_data
        .iter()
        .map(|v| v * 811589153)
        .enumerate()
        .map(|(i, v)| Number::new(v, i))
        .collect::<Vec<_>>();

    for _ in 0..10 {
        for i in 0..data.len() {
            let index_to_swap = data
                .iter()
                .enumerate()
                .find(|(_, item)| item.order == i)
                .map(|(idx, _)| idx)
                .unwrap();
            rearrange_with_swaps_mod(&mut data, index_to_swap);
        }
    }

    let zero_idx =
        data.iter().enumerate().find(|(_, item)| item.val == 0).map(|(idx, _)| idx).unwrap();

    let n1 = data[(1000 + zero_idx) % data.len()].val;
    let n2 = data[(2000 + zero_idx) % data.len()].val;
    let n3 = data[(3000 + zero_idx) % data.len()].val;

    println!("{}+{}+{} = {}", n1, n2, n3, n1 + n2 + n3);
}

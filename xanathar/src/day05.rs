use crate::data::day5data;

fn exec(stacks: &mut [Vec<u8>], mov: &[usize; 3]) {
    for _ in 0..mov[0] {
        let crt = stacks[mov[1] - 1].remove(0);
        stacks[mov[2] - 1].insert(0, crt);
    }
}

fn exec2(stacks: &mut [Vec<u8>], mov: &[usize; 3]) {
    let mut tmp = Vec::new();
    for _ in 0..mov[0] {
        let crt = stacks[mov[1] - 1].remove(0);
        tmp.insert(0, crt);
    }

    for t in tmp.drain(..) {
        stacks[mov[2] - 1].insert(0, t);
    }
}

pub fn test1() {
    let mut s = day5data::stacks();

    for m in day5data::moves().iter() {
        exec(&mut s, m);
    }

    for st in s.iter() {
        print!("{}", st[0] as char);
    }
    println!();
}

pub fn test2() {
    let mut s = day5data::stacks();

    for m in day5data::moves().iter() {
        exec2(&mut s, m);
    }

    for st in s.iter() {
        print!("{}", st[0] as char);
    }
    println!();
}

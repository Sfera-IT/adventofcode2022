use crate::data::day6data;

fn distinct(inp: &[u8]) -> bool {
    for i in 0..inp.len() {
        for j in i + 1..inp.len() {
            if inp[i] == inp[j] {
                return false;
            }
        }
    }
    true
}

fn distloc(inp: &[u8], len: usize) -> Option<usize> {
    (len..inp.len()).find(|&i| distinct(&inp[i - len..i]))
}

pub fn test1() {
    let input = day6data::INPUT.as_bytes();
    println!("FOUND: {:?}", distloc(input, 4));
}

pub fn test2() {
    let input = day6data::INPUT.as_bytes();
    println!("FOUND: {:?}", distloc(input, 14));
}

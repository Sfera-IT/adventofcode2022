use crate::day11::Monkey;

pub(crate) fn test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(&[79, 98], |old| old * 19, 23, 2, 3),
        Monkey::new(&[54, 65, 75, 74], |old| old + 6, 19, 2, 0),
        Monkey::new(&[79, 60, 97], |old| old * old, 13, 1, 3),
        Monkey::new(&[74], |old| old + 3, 17, 0, 1),
    ]
}

pub(crate) fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey::new(&[89, 74], |old| old * 5, 17, 4, 7),
        Monkey::new(&[75, 69, 87, 57, 84, 90, 66, 50], |old| old + 3, 7, 3, 2),
        Monkey::new(&[55], |old| old + 7, 13, 0, 7),
        Monkey::new(&[69, 82, 69, 56, 68], |old| old + 5, 2, 0, 2),
        Monkey::new(&[72, 97, 50], |old| old + 2, 19, 6, 5),
        Monkey::new(&[90, 84, 56, 92, 91, 91], |old| old * 19, 3, 6, 1),
        Monkey::new(&[63, 93, 55, 53], |old| old * old, 5, 3, 1),
        Monkey::new(&[50, 61, 52, 58, 86, 68, 97], |old| old + 4, 11, 5, 4),
    ]
}

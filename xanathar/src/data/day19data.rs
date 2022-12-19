use crate::day19::Blueprint;

pub(crate) fn test_data() -> Blueprint {
    Blueprint::new(4, 2, (3, 14), (2, 7))
}

pub(crate) fn data() -> Vec<Blueprint> {
    vec![
        Blueprint::new(3, 3, (2, 19), (2, 12)),
        Blueprint::new(3, 3, (3, 19), (2, 9)),
        Blueprint::new(2, 3, (3, 17), (3, 10)),
        //
        Blueprint::new(4, 4, (2, 14), (4, 15)),
        Blueprint::new(4, 4, (2, 12), (3, 15)),
        Blueprint::new(4, 4, (4, 8), (3, 19)),
        Blueprint::new(4, 4, (4, 7), (4, 17)),
        Blueprint::new(4, 4, (4, 14), (3, 16)),
        Blueprint::new(2, 4, (2, 16), (2, 9)),
        Blueprint::new(4, 4, (4, 5), (3, 7)),
        Blueprint::new(2, 4, (3, 14), (4, 9)),
        Blueprint::new(3, 4, (3, 16), (3, 14)),
        Blueprint::new(2, 3, (3, 18), (2, 19)),
        Blueprint::new(3, 3, (2, 13), (3, 12)),
        Blueprint::new(3, 3, (3, 16), (3, 9)),
        Blueprint::new(3, 3, (3, 17), (4, 8)),
        Blueprint::new(2, 2, (2, 17), (2, 10)),
        Blueprint::new(2, 3, (3, 16), (2, 11)),
        Blueprint::new(4, 4, (4, 12), (3, 8)),
        Blueprint::new(3, 4, (3, 12), (3, 17)),
        Blueprint::new(3, 3, (2, 7), (2, 9)),
        Blueprint::new(2, 4, (3, 17), (4, 20)),
        Blueprint::new(4, 3, (2, 19), (3, 10)),
        Blueprint::new(4, 4, (3, 9), (3, 7)),
        Blueprint::new(2, 3, (3, 14), (3, 19)),
        Blueprint::new(3, 3, (3, 16), (3, 20)),
        Blueprint::new(3, 4, (4, 19), (4, 11)),
        Blueprint::new(4, 3, (3, 11), (4, 7)),
        Blueprint::new(3, 3, (3, 8), (2, 12)),
        Blueprint::new(4, 4, (4, 8), (2, 15)),
    ]
}

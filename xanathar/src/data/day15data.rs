use crate::day15::Sensor;

pub(crate) fn data() -> Vec<Sensor> {
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

pub(crate) fn test_data() -> Vec<Sensor> {
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

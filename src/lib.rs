pub mod computer;
pub mod slice;

pub mod wheels {
    pub const BASE: [[u8; 12]; 4] = [
        [11, 11, 14, 11, 14, 11, 14, 14, 11, 14, 11, 14], // INNER
        [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        [4, 4, 6, 6, 3, 3, 14, 14, 21, 21, 9, 9],
        [8, 3, 4, 12, 2, 5, 10, 7, 16, 8, 7, 8], // OUTER
    ];

    pub const OUTER: [[u8; 12]; 4] = [
        [9, 0, 7, 14, 11, 0, 8, 0, 16, 2, 7, 0], // INNER
        [12, 3, 6, 0, 14, 12, 3, 8, 9, 0, 9, 20],
        [6, 0, 2, 13, 9, 0, 17, 19, 3, 12, 3, 26],
        [9, 0, 12, 0, 6, 0, 10, 0, 10, 0, 1, 0], // OUTER
    ];

    pub const OUTER_MIDDLE: [[u8; 12]; 4] = [
        [7, 8, 9, 13, 9, 7, 13, 21, 17, 4, 5, 0], // INNER
        [12, 0, 21, 6, 15, 4, 9, 18, 11, 26, 14, 1],
        [9, 0, 5, 0, 10, 0, 8, 0, 22, 0, 16, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // OUTER
    ];

    pub const INNER_MIDDLE: [[u8; 12]; 4] = [
        [11, 0, 6, 17, 7, 3, 0, 6, 0, 11, 11, 6], // INNER
        [9, 0, 12, 0, 4, 0, 7, 15, 0, 0, 14, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // OUTER
    ];

    pub const INNER: [[u8; 12]; 4] = [
        [6, 0, 10, 0, 7, 0, 15, 0, 8, 0, 3, 0], // INNER
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], // OUTER
    ];

    pub fn get_wheels(inner_first: bool) -> [[[u8; 12]; 4]; 5] {
        let mut wheels_array = [BASE, OUTER, OUTER_MIDDLE, INNER_MIDDLE, INNER];

        if inner_first {
            wheels_array.reverse();
        }

        wheels_array
    }

    pub fn print_wheels(inner_first: bool) {
        for wheel in get_wheels(inner_first).into_iter() {
            println!("{:?}", &wheel);
        }
    }
}
